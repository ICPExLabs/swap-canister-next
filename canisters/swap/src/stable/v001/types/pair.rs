use std::collections::HashMap;

use ::common::utils::math::zero;
use ::common::{types::SwapTokenPair, utils::principal::sort_tokens};

use super::*;

use super::{
    BusinessError, InnerTokenPairSwapGuard, MarketMaker, PairSwapToken, SelfCanister, TokenBalances, TokenInfo,
    TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess, TokenPairLiquidityRemoveArg,
    TokenPairLiquidityRemoveSuccess, TokenPairSwapTokensSuccess,
};

#[derive(Serialize, Deserialize)]
pub struct TokenPairs {
    #[serde(skip, default = "init_token_pairs")]
    pairs: StableBTreeMap<TokenPairAmm, MarketMaker>,
}

impl Default for TokenPairs {
    fn default() -> Self {
        Self {
            pairs: init_token_pairs(),
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

    pub fn query_dummy_tokens(&self, tokens: &HashMap<CanisterId, TokenInfo>) -> HashMap<CanisterId, TokenInfo> {
        self.query_all_token_pair_pools()
            .into_iter()
            .flat_map(|(pa, maker)| maker.dummy_tokens(tokens, &pa))
            .map(|info| (info.canister_id, info))
            .collect()
    }

    pub fn query_dummy_token_info(
        &self,
        tokens: &HashMap<CanisterId, TokenInfo>,
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
            ic_cdk::trap(&format!("too many dummy tokens for: {}", pa))
        }
        tokens.pop()
    }

    /// Query the accounts involved in this coin pair pool
    pub fn get_token_pair_pool(&self, pa: &TokenPairAmm) -> Option<MarketMaker> {
        self.pairs.get(pa)
    }

    // ============================= create pair pool =============================

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
                        "*CreateTokenPair* `token0:[{}], token1:[{}], amm:{}, subaccount:({}), dummyCanisterId:[{}]`",
                        arg.arg.pair.token0.to_text(),
                        arg.arg.pair.token1.to_text(),
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
        let mut maker = self.pairs.get(&pa).ok_or_else(|| pa.not_exist())?;
        let result = handle(&mut maker);
        self.pairs.insert(pa, maker);
        result
    }

    pub fn add_liquidity(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg>,
        pa: TokenPairAmm,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        self.handle_maker(pa, |maker| super::common::add_liquidity(maker, guard))
    }

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
                "*Pair Swap Token* `swap_pair:([{}],[{}],{}), from:({}), to:({}), pay_amount:{}, got_amount:{}`",
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

    // Fixed input to calculate the intermediate number of each coin pair
    pub fn get_amounts_out(
        &self,
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
            let maker = self.pairs.get(pa).ok_or_else(|| pa.not_exist())?;
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

    // Fixed output to calculate the intermediate number of each coin pair
    pub fn get_amounts_in(
        &self,
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
            let maker = self.pairs.get(pa).ok_or_else(|| pa.not_exist())?;
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
