use std::collections::HashMap;

use ::common::utils::principal::sort_tokens;

use super::*;

use crate::utils::math::zero;

use super::{
    Amm, BusinessError, InnerTokenPairSwapGuard, MarketMaker, SelfCanister, TokenBalances,
    TokenInfo, TokenPair, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess,
    TokenPairLiquidityRemoveArg, TokenPairLiquidityRemoveSuccess, TokenPairPool,
    TokenPairSwapByLoanArgs, TokenPairSwapExactTokensForTokensArgs,
    TokenPairSwapTokensForExactTokensArgs, TokenPairSwapTokensSuccess,
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

    pub fn query_dummy_tokens(
        &self,
        tokens: &HashMap<CanisterId, TokenInfo>,
    ) -> HashMap<CanisterId, TokenInfo> {
        self.query_all_token_pair_pools()
            .into_iter()
            .flat_map(|(pa, maker)| maker.dummy_tokens(tokens, &pa))
            .map(|info| (info.canister_id, info))
            .collect()
    }

    /// 查询该币对池子涉及的账户
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
            return Err(BusinessError::TokenPairAmmExist((
                arg.arg.pair,
                arg.arg.amm.into(),
            )));
        }

        // 1. get token block
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::Create(PairCreate {
                pa: arg.arg.clone(),
                creator: arg.caller.id(),
            })),
            memo: arg.memo,
            created: arg.created,
        };
        // 2. do create and mint block
        let maker = swap_guard.mint_block(arg.now, transaction, || {
            let TokenPairAmm { amm, .. } = &arg.arg;
            let (subaccount, dummy_canister_id) = arg.arg.get_subaccount_and_dummy_canister_id();
            let maker =
                MarketMaker::new_by_pair(amm, subaccount, dummy_canister_id, token0, token1);
            let maker = trace_guard.handle(
                |trace| {
                    self.pairs.insert(arg.arg.clone(), maker.clone()); // do insert token pair pool
                    trace.trace(format!(
                        "Token0: [{}] Token1: [{}] Amm: {} Subaccount: ({}) DummyCanisterId: [{}]",
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
        self.handle_maker(pa, |maker| maker.add_liquidity(guard))
    }

    pub fn check_liquidity_removable(
        &self,
        token_balances: &TokenBalances,
        pa: &TokenPairAmm,
        from: &Account,
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        let maker = self.pairs.get(pa).ok_or_else(|| pa.not_exist())?;
        maker.check_liquidity_removable(token_balances, from, liquidity)
    }

    pub fn remove_liquidity(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityRemoveArg>,
        pa: TokenPairAmm,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        self.handle_maker(pa, |maker| maker.remove_liquidity(guard))
    }

    // // ============================= swap =============================

    // #[allow(clippy::too_many_arguments)]
    // fn swap(
    //     &mut self,
    //     guard: &mut TokenBalancesGuard,
    //     self_canister: &SelfCanister,
    //     amounts: &[Nat],
    //     path: &[TokenPairPool],
    //     pas: &[TokenPairAmm],
    //     pool_accounts: &[Account],
    //     _to: Account,
    // ) -> Result<(), BusinessError> {
    //     for (i, (pool, pa)) in path.iter().zip(pas.iter()).enumerate() {
    //         let (input, output) = pool.pair;
    //         let (token0, _) = sort_tokens(input, output);
    //         let amount_out = amounts[i + 1].clone();
    //         let (amount0_out, amount1_out) = if input == token0 {
    //             (zero(), amount_out)
    //         } else {
    //             (amount_out, zero())
    //         };
    //         let to = if i < path.len() - 1 {
    //             pool_accounts[i + 1]
    //         } else {
    //             _to
    //         };

    //         let maker = self
    //             .0
    //             .get_mut(&pa.pair)
    //             .and_then(|makers| makers.get_mut(&pa.amm))
    //             .ok_or_else(|| pa.not_exist())?;
    //         maker.swap(guard, self_canister, amount0_out, amount1_out, to)?;
    //     }
    //     Ok(())
    // }

    // fn get_amounts_out(
    //     &self,
    //     self_canister: &SelfCanister,
    //     amount_in: &Nat,
    //     amount_out_min: &Nat,
    //     path: &[TokenPairPool],
    //     pas: &[TokenPairAmm],
    // ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
    //     let mut amounts = Vec::with_capacity(path.len() + 1);
    //     amounts.push(amount_in.clone());
    //     let mut last_amount_in = amount_in.clone();

    //     assert_eq!(path.len(), pas.len(), "path.len() != pas.len()");

    //     let mut pool_accounts = vec![];
    //     for (pool, pa) in path.iter().zip(pas.iter()) {
    //         let maker = self
    //             .0
    //             .get(&pa.pair)
    //             .and_then(|makers| makers.get(&pa.amm))
    //             .ok_or_else(|| pa.not_exist())?;

    //         let (pool_account, amount) =
    //             maker.get_amount_out(self_canister, &last_amount_in, pool.pair.0, pool.pair.1)?;
    //         last_amount_in = amount.clone();

    //         amounts.push(amount);
    //         pool_accounts.push(pool_account);
    //     }

    //     if amounts[amounts.len() - 1] < *amount_out_min {
    //         return Err(BusinessError::Swap(format!(
    //             "INSUFFICIENT_OUTPUT_AMOUNT: {}",
    //             amounts[amounts.len() - 1]
    //         )));
    //     }

    //     Ok((amounts, pool_accounts))
    // }

    // fn get_amounts_in(
    //     &self,
    //     self_canister: &SelfCanister,
    //     amount_out: &Nat,
    //     amount_in_max: &Nat,
    //     path: &[TokenPairPool],
    //     pas: &[TokenPairAmm],
    // ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
    //     let mut amounts = Vec::with_capacity(path.len() + 1);
    //     amounts.push(amount_out.clone());
    //     let mut last_amount_out = amount_out.clone();

    //     assert_eq!(path.len(), pas.len(), "path.len() != pas.len()");

    //     let mut pool_accounts = vec![];
    //     for (pool, pa) in path.iter().zip(pas.iter()).rev() {
    //         let maker = self
    //             .0
    //             .get(&pa.pair)
    //             .and_then(|makers| makers.get(&pa.amm))
    //             .ok_or_else(|| pa.not_exist())?;

    //         let (pool_account, amount) =
    //             maker.get_amount_in(self_canister, &last_amount_out, pool.pair.0, pool.pair.1)?;
    //         last_amount_out = amount.clone();

    //         amounts.push(amount);
    //         pool_accounts.push(pool_account);
    //     }

    //     // 逆序
    //     amounts.reverse();
    //     pool_accounts.reverse();

    //     // check amount in
    //     if *amount_in_max < amounts[0] {
    //         return Err(BusinessError::Swap(format!(
    //             "EXCESSIVE_INPUT_AMOUNT: {}",
    //             amounts[0]
    //         )));
    //     }

    //     Ok((amounts, pool_accounts))
    // }

    // // pair swap pay extra tokens
    // pub fn swap_exact_tokens_for_tokens(
    //     &mut self,
    //     guard: &mut TokenBalancesGuard,
    //     self_canister: &SelfCanister,
    //     args: TokenPairSwapExactTokensForTokensArgs,
    //     pas: Vec<TokenPairAmm>,
    // ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    //     todo!()

    //     // let (amounts, pool_accounts) = self.get_amounts_out(
    //     //     self_canister,
    //     //     &args.amount_in,
    //     //     &args.amount_out_min,
    //     //     &args.path,
    //     //     &pas,
    //     // )?;

    //     // // transfer first
    //     // guard.token_transfer(
    //     //     args.path[0].pair.0,
    //     //     args.from,
    //     //     pool_accounts[0],
    //     //     amounts[0].clone(),
    //     // )?;

    //     // self.swap(
    //     //     guard,
    //     //     self_canister,
    //     //     &amounts,
    //     //     &args.path,
    //     //     &pas,
    //     //     &pool_accounts,
    //     //     args.to,
    //     // )?;

    //     // Ok(TokenPairSwapTokensSuccess { amounts })
    // }

    // // pair swap got extra tokens
    // pub fn swap_tokens_for_exact_tokens(
    //     &mut self,
    //     guard: &mut TokenBalancesGuard,
    //     self_canister: &SelfCanister,
    //     args: TokenPairSwapTokensForExactTokensArgs,
    //     pas: Vec<TokenPairAmm>,
    // ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    //     todo!()

    //     // let (amounts, pool_accounts) = self.get_amounts_in(
    //     //     self_canister,
    //     //     &args.amount_out,
    //     //     &args.amount_in_max,
    //     //     &args.path,
    //     //     &pas,
    //     // )?;

    //     // // check balance in
    //     // let balance_in = guard.token_balance_of(args.path[0].pair.0, args.from)?;
    //     // if balance_in < amounts[0] {
    //     //     return Err(BusinessError::InsufficientBalance((
    //     //         args.path[0].pair.0,
    //     //         balance_in,
    //     //     )));
    //     // }

    //     // // transfer first
    //     // guard.token_transfer(
    //     //     args.path[0].pair.0,
    //     //     args.from,
    //     //     pool_accounts[0],
    //     //     amounts[0].clone(),
    //     // )?;

    //     // self.swap(
    //     //     guard,
    //     //     self_canister,
    //     //     &amounts,
    //     //     &args.path,
    //     //     &pas,
    //     //     &pool_accounts,
    //     //     args.to,
    //     // )?;

    //     // Ok(TokenPairSwapTokensSuccess { amounts })
    // }

    // // pair swap by loan
    // pub fn swap_by_loan(
    //     &mut self,
    //     guard: &mut TokenBalancesGuard,
    //     self_canister: &SelfCanister,
    //     args: TokenPairSwapByLoanArgs,
    //     pas: Vec<TokenPairAmm>,
    // ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    //     todo!()

    //     // let (amounts, pool_accounts) =
    //     //     self.get_amounts_out(self_canister, &args.loan, &args.loan, &args.path, &pas)?;

    //     // // ! loan token // transfer first
    //     // guard.token_deposit(args.path[0].pair.0, pool_accounts[0], args.loan.clone())?;

    //     // self.swap(
    //     //     guard,
    //     //     self_canister,
    //     //     &amounts,
    //     //     &args.path,
    //     //     &pas,
    //     //     &pool_accounts,
    //     //     args.to,
    //     // )?;

    //     // // ! return loan
    //     // guard.token_withdraw(args.path[0].pair.0, args.to, args.loan.clone())?;

    //     // Ok(TokenPairSwapTokensSuccess { amounts })
    // }
}
