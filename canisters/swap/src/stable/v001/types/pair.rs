use ic_canister_kit::common::trap;
use std::collections::HashMap;
use std::sync::RwLock;

use ::common::utils::math::zero;
use ::common::{types::SwapTokenPair, utils::principal::sort_tokens};

use super::*;

use super::super::super::with_mut_state;

use super::{
    BusinessError, InnerTokenPairSwapGuard, MarketMaker, PairRemove, PairSwapToken, SelfCanister, TokenBalances,
    TokenInfo, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess, TokenPairLiquidityRemoveArg,
    TokenPairLiquidityRemoveSuccess, TokenPairSwapTokensSuccess,
};

#[derive(Serialize, Deserialize)]
pub struct TokenPairs {
    #[serde(skip, default = "init_token_pairs")]
    pairs: StableBTreeMap<TokenPairAmm, MarketMaker>,
    #[serde(default = "Default::default")]
    locks: RwLock<HashMap<TokenPairAmm, bool>>,
}

impl Default for TokenPairs {
    fn default() -> Self {
        Self {
            pairs: init_token_pairs(),
            locks: Default::default(),
        }
    }
}

impl TokenPairs {
    pub fn query_all_token_pair_pools(&self) -> Vec<(TokenPairAmm, MarketMaker)> {
        self.pairs
            .keys()
            .filter_map(|pa| self.pairs.get(&pa).map(|maker| (pa, maker)))
            .collect()
    }

    pub fn query_dummy_tokens(
        &self,
        tokens: &HashMap<CanisterId, Cow<'_, TokenInfo>>,
    ) -> HashMap<CanisterId, TokenInfo> {
        self.query_all_token_pair_pools()
            .into_iter()
            .flat_map(|(pa, maker)| maker.dummy_tokens(tokens, &pa))
            .map(|info| (info.canister_id, info))
            .collect()
    }

    pub fn query_dummy_token_info(
        &self,
        tokens: &HashMap<CanisterId, Cow<'_, TokenInfo>>,
        pa: &TokenPairAmm,
    ) -> Option<TokenInfo> {
        let mut tokens = self
            .pairs
            .get(pa)
            .map(|maker| maker.dummy_tokens(tokens, pa))
            .unwrap_or_default();
        if tokens.is_empty() {
            return None;
        }
        if 1 < tokens.len() {
            ic_cdk::trap(format!("too many dummy tokens for: {}", pa))
        }
        tokens.pop()
    }

    /// Query the accounts involved in this coin pair pool
    pub fn get_token_pair_pool(&self, pa: &TokenPairAmm) -> Option<MarketMaker> {
        self.pairs.get(pa)
    }

    // locks
    pub fn lock(&mut self, required: Vec<TokenPairAmm>) -> Result<TokenPairsLock, Vec<TokenPairAmm>> {
        let mut locks = trap(self.locks.write()); // ! what if failed ?

        // duplicate removal
        let locked = required.iter().cloned().collect::<HashSet<_>>();

        // 1. check first
        let mut already_locked: Vec<TokenPairAmm> = vec![];
        for pa in &locked {
            if locks.get(pa).is_some_and(|lock| *lock) {
                already_locked.push(*pa);
            }
        }
        if !already_locked.is_empty() {
            return Err(already_locked);
        }

        // 2. do lock
        for token_account in &locked {
            locks.insert(*token_account, true);
        }

        for pa in &required {
            ic_cdk::println!("🔒 Locked token pair: {pa}",);
        }

        Ok(TokenPairsLock { required, locked })
    }

    pub fn unlock(&mut self, locked: &HashSet<TokenPairAmm>) {
        let mut locks = trap(self.locks.write()); // ! what if failed ?

        // 1. check first
        for pa in locked {
            if locks.get(pa).is_some_and(|lock| *lock) {
                continue; // locked is right
            }
            // if not true, terminator
            let tips = format!("Unlock token pair: {pa} that is not locked.",);
            ic_cdk::trap(&tips); // never be here
        }

        // 2. do unlock
        for token_account in locked {
            locks.remove(token_account);
        }
    }

    pub fn be_guard<'a>(&'a mut self, lock: &'a TokenPairsLock) -> TokenPairsGuard<'a> {
        TokenPairsGuard::new(&mut self.pairs, lock)
    }

    // ============================= create pair pool =============================

    /// * does not need guard
    pub fn create_token_pair_pool(
        &mut self,
        swap_guard: &mut SwapBlockChainGuard,
        trace_guard: &mut RequestTraceGuard,
        arg: ArgWithMeta<TokenPairAmm>,
        token0: &TokenInfo,
        token1: &TokenInfo,
    ) -> Result<MarketMaker, BusinessError> {
        if self.get_token_pair_pool(&arg.arg).is_some() {
            return Err(BusinessError::TokenPairAmmExist(arg.arg));
        }

        // 1. get token block
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::Create(PairCreate {
                pa: arg.arg,
                creator: arg.caller.id(),
            })),
            memo: arg.memo,
            created: arg.created,
        };
        // 2. do create and mint block
        let maker = swap_guard.mint_block(arg.now, transaction, |_| {
            let TokenPairAmm { amm, .. } = &arg.arg;
            let (subaccount, dummy_canister_id) = arg.arg.get_subaccount_and_dummy_canister_id();
            let maker = MarketMaker::new_by_pair(amm, subaccount, dummy_canister_id, token0, token1);
            let maker = trace_guard.handle(
                |trace| {
                    self.pairs.insert(arg.arg, maker.clone()); // do insert token pair pool
                    trace.trace(format!(
                        "*TokenPairCreate* `token0:[{}], token1:[{}], amm:{}, subaccount:({}), dummyCanisterId:[{}]`",
                        arg.arg.pair.get_token0().to_text(),
                        arg.arg.pair.get_token1().to_text(),
                        arg.arg.amm.into_text().as_ref(),
                        hex::encode(subaccount),
                        dummy_canister_id.id().to_text()
                    ));
                    Ok(maker)
                },
                |data| {
                    let view: MarketMakerView = data.clone().into();
                    serde_json::to_string(&view).unwrap_or_default()
                },
            )?;
            Ok(maker)
        })?;
        Ok(maker)
    }

    // ============================= liquidity =============================

    pub fn check_liquidity_removable(
        &self,
        token_balances: &TokenBalances,
        pa: &TokenPairAmm,
        from: &Account,
        liquidity_without_fee: &Nat,
        fee_to: Option<Account>,
    ) -> Result<(), BusinessError> {
        let maker = self.pairs.get(pa).ok_or_else(|| pa.not_exist())?;
        maker.check_liquidity_removable(
            |token, account| token_balances.token_balance_of(token, account),
            from,
            liquidity_without_fee,
            fee_to,
        )
    }

    // ============================= swap =============================

    // Fixed input to calculate the intermediate number of each coin pair
    fn inner_get_amounts_out<F: Fn(&TokenPairAmm) -> Result<MarketMaker, BusinessError>>(
        get_pair: F,
        self_canister: &SelfCanister,
        amount_in: &Nat,
        amount_out_min: &Nat,
        path: &[SwapTokenPair],
        pas: &[TokenPairAmm],
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        let mut amounts = Vec::with_capacity(path.len() + 1);
        amounts.push(amount_in.clone());
        let mut last_amount_in = amount_in.clone();

        assert_eq!(path.len(), pas.len(), "path.len() != pas.len()");

        let mut pool_accounts = vec![];
        for (pool, pa) in path.iter().zip(pas.iter()) {
            // Get the next coin pair
            let maker = get_pair(pa)?;
            // Calculate the output that the coin pair can obtain
            let (pool_account, amount) =
                maker.get_amount_out(self_canister, &last_amount_in, pool.token.0, pool.token.1)?;
            last_amount_in = amount.clone(); // Save the output quantity, and the input quantity in the next cycle

            amounts.push(amount);
            pool_accounts.push(pool_account);
        }

        // Determine whether the output quantity meets the requirements
        if amounts[amounts.len() - 1] < *amount_out_min {
            return Err(BusinessError::Swap(format!(
                "INSUFFICIENT_OUTPUT_AMOUNT: {}",
                amounts[amounts.len() - 1]
            )));
        }

        Ok((amounts, pool_accounts))
    }
    pub fn get_amounts_out(
        &self,
        self_canister: &SelfCanister,
        amount_in: &Nat,
        amount_out_min: &Nat,
        path: &[SwapTokenPair],
        pas: &[TokenPairAmm],
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        Self::inner_get_amounts_out(
            |pa| self.pairs.get(pa).ok_or_else(|| pa.not_exist()),
            self_canister,
            amount_in,
            amount_out_min,
            path,
            pas,
        )
    }

    // Fixed output to calculate the intermediate number of each coin pair
    fn inner_get_amounts_in<F: Fn(&TokenPairAmm) -> Result<MarketMaker, BusinessError>>(
        get_pair: F,
        self_canister: &SelfCanister,
        amount_out: &Nat,
        amount_in_max: &Nat,
        path: &[SwapTokenPair],
        pas: &[TokenPairAmm],
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        let mut amounts = Vec::with_capacity(path.len() + 1);
        amounts.push(amount_out.clone());
        let mut last_amount_out = amount_out.clone();

        assert_eq!(path.len(), pas.len(), "path.len() != pas.len()");

        let mut pool_accounts = vec![];
        for (pool, pa) in path.iter().zip(pas.iter()).rev() {
            // Get the next coin pair
            let maker = get_pair(pa)?;
            // Calculate the input that the coin pair can obtain
            let (pool_account, amount) =
                maker.get_amount_in(self_canister, &last_amount_out, pool.token.0, pool.token.1)?;
            last_amount_out = amount.clone(); // Save the input quantity, and the output quantity in the next loop

            amounts.push(amount);
            pool_accounts.push(pool_account);
        }

        // Reverse order
        amounts.reverse();
        pool_accounts.reverse();

        // Determine whether the input quantity meets the requirements
        if *amount_in_max < amounts[0] {
            return Err(BusinessError::Swap(format!("EXCESSIVE_INPUT_AMOUNT: {}", amounts[0])));
        }

        Ok((amounts, pool_accounts))
    }
    pub fn get_amounts_in(
        &self,
        self_canister: &SelfCanister,
        amount_out: &Nat,
        amount_in_max: &Nat,
        path: &[SwapTokenPair],
        pas: &[TokenPairAmm],
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        Self::inner_get_amounts_in(
            |pa| self.pairs.get(pa).ok_or_else(|| pa.not_exist()),
            self_canister,
            amount_out,
            amount_in_max,
            path,
            pas,
        )
    }

    // ======================== fix ========================

    pub fn fix_bg_pool(&mut self) -> Result<(), BusinessError> {
        let (pa, mut maker) = self
            .pairs
            .iter()
            .find(|(_pa, maker)| {
                maker.dummy_canisters().iter().any(|token| {
                    token.to_text().as_str() == "l7rwb-odqru-vj3u7-n5jvs-fxscz-6hd2c-a4fvt-2cj2r-yqnab-e5jfg-prq"
                })
            })
            .ok_or_else(|| {
                BusinessError::SystemError(
                    "can not find dummy pool by l7rwb-odqru-vj3u7-n5jvs-fxscz-6hd2c-a4fvt-2cj2r-yqnab-e5jfg-prq"
                        .to_string(),
                )
            })?;
        //             amm:"swap_v2_0.05%"
        // token0:"ryjl3-tyaaa-aaaaa-aaaba-cai"
        // token1:"c6zxb-naaaa-aaaah-are2q-cai"
        if pa.pair.get_token0().to_text() != "ryjl3-tyaaa-aaaaa-aaaba-cai"
            || pa.pair.get_token1().to_text() != "c6zxb-naaaa-aaaah-are2q-cai"
            || pa.amm.into_text().as_ref() != "swap_v2_0.05%"
        {
            return Err(BusinessError::SystemError(format!(
                "got wrong pair: {}",
                serde_json::to_string(&pa).unwrap_or_default()
            )));
        }

        // 1. restore total supply
        #[allow(irrefutable_let_patterns)]
        if let MarketMaker::SwapV2(maker) = &mut maker {
            if let ::common::types::PoolLp::InnerLP(lp) = &mut maker.lp {
                lp.total_supply = Nat::from(31_622_770_277_112_u64); // ! now value is 70_277_112, wrong
            }
        }

        self.pairs.insert(pa, maker);

        Ok(())
    }
    pub fn fix_bg_pool_reserve(&mut self, icp_balance: Nat, bg_balance: Nat) -> Result<(), BusinessError> {
        let (pa, mut maker) = self
            .pairs
            .iter()
            .find(|(_pa, maker)| {
                maker.dummy_canisters().iter().any(|token| {
                    token.to_text().as_str() == "l7rwb-odqru-vj3u7-n5jvs-fxscz-6hd2c-a4fvt-2cj2r-yqnab-e5jfg-prq"
                })
            })
            .ok_or_else(|| {
                BusinessError::SystemError(
                    "can not find dummy pool by l7rwb-odqru-vj3u7-n5jvs-fxscz-6hd2c-a4fvt-2cj2r-yqnab-e5jfg-prq"
                        .to_string(),
                )
            })?;
        // amm:"swap_v2_0.05%"
        // token0:"ryjl3-tyaaa-aaaaa-aaaba-cai"
        // token1:"c6zxb-naaaa-aaaah-are2q-cai"
        if pa.pair.get_token0().to_text() != "ryjl3-tyaaa-aaaaa-aaaba-cai"
            || pa.pair.get_token1().to_text() != "c6zxb-naaaa-aaaah-are2q-cai"
            || pa.amm.into_text().as_ref() != "swap_v2_0.05%"
        {
            return Err(BusinessError::SystemError(format!(
                "got wrong pair: {}",
                serde_json::to_string(&pa).unwrap_or_default()
            )));
        }

        // 1. restore total supply
        #[allow(irrefutable_let_patterns)]
        if let MarketMaker::SwapV2(maker) = &mut maker {
            maker.reserve0 = icp_balance;
            maker.reserve1 = bg_balance;
        }

        self.pairs.insert(pa, maker);

        Ok(())
    }
}

// ============================ lock ============================

pub struct TokenPairsLock {
    required: Vec<TokenPairAmm>,   // The target requires locked account, print it to display
    locked: HashSet<TokenPairAmm>, // fee_to must be included
}
impl Drop for TokenPairsLock {
    fn drop(&mut self) {
        with_mut_state(|s| {
            s.get_mut().business_token_pair_unlock(&self.locked);
            for pa in &self.required {
                ic_cdk::println!("🔐 Unlock token pair: {pa}");
            }
        })
    }
}

// ============================ guard ============================

pub use guard::TokenPairsGuard;
mod guard {
    use super::*;
    pub struct TokenPairsGuard<'a> {
        stable_pairs: &'a mut StableBTreeMap<TokenPairAmm, MarketMaker>,
        lock: &'a TokenPairsLock,
        // stack data
        stack_pairs: HashMap<TokenPairAmm, MarketMaker>,
        removed_pairs: HashSet<TokenPairAmm>,
    }
    impl Drop for TokenPairsGuard<'_> {
        fn drop(&mut self) {
            // must drop by manual
        }
    }

    impl<'a> TokenPairsGuard<'a> {
        pub(super) fn new(
            stable_pairs: &'a mut StableBTreeMap<TokenPairAmm, MarketMaker>,
            lock: &'a TokenPairsLock,
        ) -> Self {
            let stack_pairs = lock
                .locked
                .iter()
                .map(|pa| {
                    (
                        *pa,
                        trap(
                            stable_pairs
                                .get(pa)
                                .ok_or_else(|| BusinessError::SystemError(format!("can not find maker by pa: {pa}"))),
                        ),
                    )
                })
                .collect();
            Self {
                stable_pairs,
                lock,
                stack_pairs,
                removed_pairs: Default::default(),
            }
        }

        pub(super) fn get_market_maker(&self, pa: &TokenPairAmm) -> Result<&MarketMaker, BusinessError> {
            if !self.lock.locked.contains(pa) {
                return Err(BusinessError::unlocked_token_pair(*pa));
            }
            let maker = trap(
                self.stack_pairs
                    .get(pa)
                    .ok_or_else(|| BusinessError::system_error("can find market maker.")),
            );
            Ok(maker)
        }
        pub(super) fn get_market_maker_mut(&mut self, pa: &TokenPairAmm) -> Result<&mut MarketMaker, BusinessError> {
            if !self.lock.locked.contains(pa) {
                return Err(BusinessError::unlocked_token_pair(*pa));
            }
            let maker = trap(
                self.stack_pairs
                    .get_mut(pa)
                    .ok_or_else(|| BusinessError::system_error("can find market maker.")),
            );
            Ok(maker)
        }

        pub fn get_locked_pairs(&self) -> Vec<TokenPairAmm> {
            self.lock.required.clone()
        }

        pub(super) fn remove_token_pair(&mut self, pa: &TokenPairAmm) {
            self.removed_pairs.insert(*pa);
        }

        pub fn get_amounts_out(
            &self,
            self_canister: &SelfCanister,
            amount_in: &Nat,
            amount_out_min: &Nat,
            path: &[SwapTokenPair],
            pas: &[TokenPairAmm],
        ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
            TokenPairs::inner_get_amounts_out(
                |pa| {
                    self.stack_pairs
                        .get(pa)
                        .cloned()
                        .ok_or_else(|| BusinessError::unlocked_token_pair(*pa))
                },
                self_canister,
                amount_in,
                amount_out_min,
                path,
                pas,
            )
        }
        pub(super) fn get_amounts_in(
            &self,
            self_canister: &SelfCanister,
            amount_out: &Nat,
            amount_in_max: &Nat,
            path: &[SwapTokenPair],
            pas: &[TokenPairAmm],
        ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
            TokenPairs::inner_get_amounts_in(
                |pa| {
                    self.stack_pairs
                        .get(pa)
                        .cloned()
                        .ok_or_else(|| BusinessError::unlocked_token_pair(*pa))
                },
                self_canister,
                amount_out,
                amount_in_max,
                path,
                pas,
            )
        }

        pub fn dump(self) {
            for (pa, maker) in self.stack_pairs.iter() {
                self.stable_pairs.insert(*pa, maker.clone());
            }
            for pa in self.removed_pairs.iter() {
                self.stable_pairs.remove(pa);
            }
        }
    }
}

impl TokenPairsGuard<'_> {
    // ============================= config protocol fee =============================
    pub fn replace_protocol_fee(&mut self, pa: &TokenPairAmm, protocol_fee: Option<SwapRatio>) -> Option<SwapRatio> {
        let maker = trap(self.get_market_maker_mut(pa));
        maker.replace_protocol_fee(protocol_fee)
    }

    // ============================= remove pair pool =============================

    pub fn remove_token_pair_pool(
        &mut self,
        swap_guard: &mut SwapBlockChainGuard,
        trace_guard: &mut RequestTraceGuard,
        arg: ArgWithMeta<TokenPairAmm>,
    ) -> Result<MarketMaker, BusinessError> {
        let maker = self.get_market_maker(&arg.arg).map_err(|_| arg.arg.not_exist())?;
        if !maker.removable() {
            return Err(BusinessError::TokenPairAmmStillAlive(arg.arg));
        }
        let maker = maker.clone();

        // 1. get token block
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::Remove(PairRemove {
                pa: arg.arg,
                remover: arg.caller.id(),
            })),
            memo: arg.memo,
            created: arg.created,
        };
        // 2. do create and mint block
        let maker = swap_guard.mint_block(arg.now, transaction, |_| {
            let (subaccount, dummy_canister_id) = arg.arg.get_subaccount_and_dummy_canister_id();
            let maker = trace_guard.handle(
                |trace| {
                    self.remove_token_pair(&arg.arg); // ! do remove token pair pool when dump
                    trace.trace(format!(
                        "*TokenPairRemove* `token0:[{}], token1:[{}], amm:{}, subaccount:({}), dummyCanisterId:[{}]`",
                        arg.arg.pair.get_token0().to_text(),
                        arg.arg.pair.get_token1().to_text(),
                        arg.arg.amm.into_text().as_ref(),
                        hex::encode(subaccount),
                        dummy_canister_id.id().to_text()
                    ));
                    Ok(maker)
                },
                |data| {
                    let view: MarketMakerView = data.clone().into();
                    serde_json::to_string(&view).unwrap_or_default()
                },
            )?;
            Ok(maker)
        })?;
        Ok(maker)
    }

    // ============================= liquidity =============================

    fn handle_maker<T, F>(&mut self, pa: TokenPairAmm, handle: F) -> Result<T, BusinessError>
    where
        F: FnOnce(&mut MarketMaker) -> Result<T, BusinessError>,
    {
        let maker = self.get_market_maker_mut(&pa).map_err(|_| pa.not_exist())?;
        handle(maker)
    }

    pub fn add_liquidity(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg>,
        pa: TokenPairAmm,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        self.handle_maker(pa, |maker| super::common::add_liquidity(maker, guard))
    }

    pub fn remove_liquidity(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityRemoveArg>,
        pa: TokenPairAmm,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        self.handle_maker(pa, |maker| super::common::remove_liquidity(maker, guard))
    }

    // ============================= swap =============================

    fn swap<T: SelfCanisterArg + TokenPairSwapArg + Clone>(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, T>,
        amounts: &[Nat],
        pool_accounts: &[Account],
        _from: Account,
        _from_amount: Nat,
        _to: Account,
    ) -> Result<(), BusinessError> {
        let self_canister = guard.arg.arg.get_self_canister();
        let path = guard.arg.arg.get_path().to_vec();
        let pas = guard.arg.arg.get_pas().to_vec();
        let mut last_from = _from;
        let mut last_from_amount = _from_amount;
        for (i, (pool, pa)) in path.iter().zip(pas.into_iter()).enumerate() {
            let (input, output) = pool.token;
            let (token0, _) = sort_tokens(input, output);
            let amount_out = amounts[i + 1].clone();
            let (amount0_out, amount1_out) = if input == token0 {
                (zero(), amount_out.clone())
            } else {
                (amount_out.clone(), zero())
            };
            let to = if i < path.len() - 1 { pool_accounts[i + 1] } else { _to };

            let transaction = SwapTransaction {
                operation: SwapOperation::Pair(PairOperation::Swap(PairSwapToken {
                    token_a: input,
                    token_b: output,
                    amm: pa.amm,
                    from: last_from,
                    to,
                    amount_a: last_from_amount.clone(),
                    amount_b: amount_out.clone(),
                })),
                memo: guard.arg.memo.clone(),
                created: guard.arg.created,
            };
            let trace = format!(
                "*TokenPairSwap* `swap_pair:([{}],[{}],{}), from:({}), to:({}), pay_amount:{}, got_amount:{}`",
                input.to_text(),
                output.to_text(),
                pa.amm.into_text().as_ref(),
                display_account(&last_from),
                display_account(&to),
                last_from_amount,
                amount_out,
            );

            struct WrappedPairAmm {
                pa: TokenPairAmm,
            }
            impl TokenPairArg for WrappedPairAmm {
                fn get_pa(&self) -> &TokenPairAmm {
                    &self.pa
                }
            }
            let wpa = ArgWithMeta {
                now: guard.arg.now,
                caller: guard.arg.caller,
                arg: WrappedPairAmm { pa },
                memo: guard.arg.memo.clone(),
                created: guard.arg.created,
            };
            self.handle_maker(pa, |maker| {
                guard.handle_guard(wpa, |guard| {
                    super::common::swap(
                        maker,
                        guard,
                        transaction,
                        trace,
                        &self_canister,
                        amount0_out,
                        amount1_out,
                        to,
                    )
                })
            })?;

            last_from = to;
            last_from_amount = amount_out;
        }
        Ok(())
    }

    // pair swap pay extra tokens
    pub fn swap_exact_tokens_for_tokens(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairSwapExactTokensForTokensArg>,
        pas: Vec<TokenPairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        let arg = &guard.arg.arg;
        let (amounts, pool_accounts) =
            self.get_amounts_out(&arg.self_canister, &arg.amount_in, &arg.amount_out_min, &arg.path, &pas)?;

        // transfer first
        guard.token_transfer(TransferToken {
            token: arg.path[0].token.0,
            from: arg.from,
            amount: amounts[0].clone(),
            to: pool_accounts[0],
            fee: None,
        })?;

        // do swap
        let arg = &guard.arg.arg;
        self.swap(guard, &amounts, &pool_accounts, arg.from, amounts[0].clone(), arg.to)?;

        Ok(TokenPairSwapTokensSuccess { amounts })
    }

    // pair swap got extra tokens
    pub fn swap_tokens_for_exact_tokens(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairSwapTokensForExactTokensArg>,
        pas: Vec<TokenPairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        let arg = &guard.arg.arg;
        let (amounts, pool_accounts) =
            self.get_amounts_in(&arg.self_canister, &arg.amount_out, &arg.amount_in_max, &arg.path, &pas)?;

        // ! check balance in
        let balance_in = guard.token_balance_of(arg.path[0].token.0, arg.from)?;
        if balance_in < amounts[0] {
            return Err(BusinessError::insufficient_balance(arg.path[0].token.0, balance_in));
        }

        // transfer first
        guard.token_transfer(TransferToken {
            token: arg.path[0].token.0,
            from: arg.from,
            amount: amounts[0].clone(),
            to: pool_accounts[0],
            fee: None,
        })?;

        // do swap
        let arg = &guard.arg.arg;
        self.swap(guard, &amounts, &pool_accounts, arg.from, amounts[0].clone(), arg.to)?;

        Ok(TokenPairSwapTokensSuccess { amounts })
    }

    // pair swap by loan
    pub fn swap_by_loan(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairSwapByLoanArg>,
        pas: Vec<TokenPairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        let arg = &guard.arg.arg;
        let (amounts, pool_accounts) = self.get_amounts_out(
            &arg.self_canister,
            &arg.loan, // Enter the number of loans
            &arg.loan, // The output must be no less than the loan quantity
            &arg.path,
            &pas,
        )?;

        // The lender, that is, the canister itself
        let loaner = Account {
            owner: arg.self_canister.id(),
            subaccount: None,
        };

        // ! loan token // transfer first
        guard.token_loan(DepositToken {
            token: arg.path[0].token.0,
            from: loaner,
            amount: arg.loan.clone(),
            to: pool_accounts[0],
        })?;

        // do swap
        let arg = &guard.arg.arg;
        self.swap(guard, &amounts, &pool_accounts, loaner, arg.loan.clone(), arg.to)?;

        // ! return loan
        let arg = &guard.arg.arg;
        guard.token_repay(WithdrawToken {
            token: arg.path[0].token.0,
            from: arg.to,
            amount: arg.loan.clone(),
            to: loaner,
        })?;

        Ok(TokenPairSwapTokensSuccess { amounts })
    }
}
