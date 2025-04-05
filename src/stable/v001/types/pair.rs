use std::collections::HashMap;

use candid::Nat;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::{Deserialize, Serialize};

use crate::utils::{math::zero, principal::sort_tokens};

use super::{
    Amm, BusinessError, DummyCanisterId, MarketMaker, PairAmm, SelfCanister, TokenBalances,
    TokenInfo, TokenPair, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess,
    TokenPairLiquidityRemoveArg, TokenPairLiquidityRemoveSuccess, TokenPairPool,
    TokenPairSwapExactTokensForTokensArgs, TokenPairSwapTokensForExactTokensArgs,
    TokenPairSwapTokensSuccess,
};

#[derive(Serialize, Deserialize, Default)]
pub struct TokenPairs(HashMap<TokenPair, HashMap<Amm, MarketMaker>>);

impl TokenPairs {
    pub fn query_token_pair_pools(&self) -> Vec<(&TokenPair, &Amm, &MarketMaker)> {
        self.0
            .iter()
            .flat_map(|(pair, makers)| makers.iter().map(move |(amm, maker)| (pair, amm, maker)))
            .collect()
    }

    pub fn business_dummy_tokens_query(
        &self,
        tokens: &HashMap<CanisterId, TokenInfo>,
    ) -> HashMap<CanisterId, TokenInfo> {
        self.query_token_pair_pools()
            .into_iter()
            .flat_map(|(pair, amm, maker)| maker.dummy_tokens(tokens, pair, amm.into()))
            .map(|info| (info.canister_id, info))
            .collect()
    }

    /// 查询该币对池子涉及的账户
    pub fn get_token_pair_pool_maker(&self, pa: &PairAmm) -> Option<&MarketMaker> {
        let PairAmm { pair, amm } = pa;
        self.0.get(pair).and_then(|makers| makers.get(amm))
    }

    // ============================= create pair pool =============================

    pub fn create_token_pair_pool(
        &mut self,
        pa: PairAmm,
        subaccount: Subaccount,
        dummy_canister_id: DummyCanisterId,
        token0: &TokenInfo,
        token1: &TokenInfo,
    ) -> Result<(), BusinessError> {
        if self.get_token_pair_pool_maker(&pa).is_some() {
            return Err(BusinessError::TokenPairAmmExist((
                pa.pair,
                (&pa.amm).into(),
            )));
        }

        let PairAmm { pair, amm } = pa;

        let maker = MarketMaker::new_by_pair(&amm, subaccount, dummy_canister_id, token0, token1);

        let makers = self.0.entry(pair).or_default();
        makers.entry(amm).or_insert(maker);

        Ok(())
    }

    // ============================= liquidity =============================

    pub fn add_liquidity(
        &mut self,
        fee_to: Option<Account>,
        token_balances: &mut TokenBalances,
        self_canister: &SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        let maker = self
            .0
            .get_mut(&pa.pair)
            .and_then(|makers| makers.get_mut(&pa.amm))
            .ok_or_else(|| pa.not_exist())?;

        maker.add_liquidity(fee_to, token_balances, self_canister, arg)
    }

    pub fn check_liquidity_removable(
        &self,
        token_balances: &TokenBalances,
        pa: &PairAmm,
        from: &Account,
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        let maker = self
            .0
            .get(&pa.pair)
            .and_then(|makers| makers.get(&pa.amm))
            .ok_or_else(|| pa.not_exist())?;

        maker.check_liquidity_removable(token_balances, from, liquidity)
    }

    pub fn remove_liquidity(
        &mut self,
        fee_to: Option<Account>,
        token_balances: &mut TokenBalances,
        self_canister: &SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        let maker = self
            .0
            .get_mut(&pa.pair)
            .and_then(|makers| makers.get_mut(&pa.amm))
            .ok_or_else(|| pa.not_exist())?;

        maker.remove_liquidity(fee_to, token_balances, self_canister, arg)
    }

    // ============================= swap =============================

    #[allow(clippy::too_many_arguments)]
    fn swap(
        &mut self,
        token_balances: &mut TokenBalances,
        self_canister: &SelfCanister,
        amounts: &[Nat],
        path: &[TokenPairPool],
        pas: &[PairAmm],
        pool_accounts: &[Account],
        _to: Account,
    ) -> Result<(), BusinessError> {
        for (i, (pool, pa)) in path.iter().zip(pas.iter()).enumerate() {
            let (input, output) = pool.pair;
            let (token0, _) = sort_tokens(input, output);
            let amount_out = amounts[i + 1].clone();
            let (amount0_out, amount1_out) = if input == token0 {
                (zero(), amount_out)
            } else {
                (amount_out, zero())
            };
            let to = if i < path.len() - 1 {
                pool_accounts[i + 1]
            } else {
                _to
            };

            let maker = self
                .0
                .get_mut(&pa.pair)
                .and_then(|makers| makers.get_mut(&pa.amm))
                .ok_or_else(|| pa.not_exist())?;
            maker.swap(token_balances, self_canister, amount0_out, amount1_out, to)?;
        }
        Ok(())
    }

    fn get_amounts_out(
        &self,
        self_canister: &SelfCanister,
        amount_in: &Nat,
        amount_out_min: &Nat,
        path: &[TokenPairPool],
        pas: &[PairAmm],
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        let mut amounts = Vec::with_capacity(path.len() + 1);
        amounts.push(amount_in.clone());
        let mut last_amount_in = amount_in.clone();

        #[allow(clippy::panic)] // ? SAFETY
        if path.len() != pas.len() {
            panic!("path.len() != pas.len()");
        }

        let mut pool_accounts = vec![];
        for (pool, pa) in path.iter().zip(pas.iter()) {
            let maker = self
                .0
                .get(&pa.pair)
                .and_then(|makers| makers.get(&pa.amm))
                .ok_or_else(|| pa.not_exist())?;

            let (pool_account, amount) =
                maker.get_amount_out(self_canister, &last_amount_in, pool.pair.0, pool.pair.1)?;
            last_amount_in = amount.clone();

            amounts.push(amount);
            pool_accounts.push(pool_account);
        }

        if amounts[amounts.len() - 1] < *amount_out_min {
            return Err(BusinessError::Swap(format!(
                "INSUFFICIENT_OUTPUT_AMOUNT: {}",
                amounts[amounts.len() - 1]
            )));
        }

        Ok((amounts, pool_accounts))
    }

    fn get_amounts_in(
        &self,
        self_canister: &SelfCanister,
        amount_out: &Nat,
        amount_in_max: &Nat,
        path: &[TokenPairPool],
        pas: &[PairAmm],
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        let mut amounts = Vec::with_capacity(path.len() + 1);
        amounts.push(amount_out.clone());
        let mut last_amount_out = amount_out.clone();

        #[allow(clippy::panic)] // ? SAFETY
        if path.len() != pas.len() {
            panic!("path.len() != pas.len()");
        }

        let mut pool_accounts = vec![];
        for (pool, pa) in path.iter().zip(pas.iter()).rev() {
            let maker = self
                .0
                .get(&pa.pair)
                .and_then(|makers| makers.get(&pa.amm))
                .ok_or_else(|| pa.not_exist())?;

            let (pool_account, amount) =
                maker.get_amount_in(self_canister, &last_amount_out, pool.pair.0, pool.pair.1)?;
            last_amount_out = amount.clone();

            amounts.push(amount);
            pool_accounts.push(pool_account);
        }

        // 逆序
        amounts.reverse();
        pool_accounts.reverse();

        // check amount in
        if *amount_in_max < amounts[0] {
            return Err(BusinessError::Swap(format!(
                "EXCESSIVE_INPUT_AMOUNT: {}",
                amounts[0]
            )));
        }

        Ok((amounts, pool_accounts))
    }

    // pair swap pay extra tokens
    pub fn swap_exact_tokens_for_tokens(
        &mut self,
        token_balances: &mut TokenBalances,
        self_canister: &SelfCanister,
        args: TokenPairSwapExactTokensForTokensArgs,
        pas: Vec<PairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        let (amounts, pool_accounts) = self.get_amounts_out(
            self_canister,
            &args.amount_in,
            &args.amount_out_min,
            &args.path,
            &pas,
        )?;

        // transfer first
        token_balances.token_transfer(
            args.path[0].pair.0,
            args.from,
            pool_accounts[0],
            amounts[0].clone(),
        );

        self.swap(
            token_balances,
            self_canister,
            &amounts,
            &args.path,
            &pas,
            &pool_accounts,
            args.to,
        )?;

        Ok(TokenPairSwapTokensSuccess { amounts })
    }

    // pair swap got extra tokens
    pub fn swap_tokens_for_exact_tokens(
        &mut self,
        token_balances: &mut TokenBalances,
        self_canister: &SelfCanister,
        args: TokenPairSwapTokensForExactTokensArgs,
        pas: Vec<PairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        let (amounts, pool_accounts) = self.get_amounts_in(
            self_canister,
            &args.amount_out,
            &args.amount_in_max,
            &args.path,
            &pas,
        )?;

        // check balance in
        let balance_in = token_balances.token_balance_of(args.path[0].pair.0, args.from);
        if balance_in < amounts[0] {
            return Err(BusinessError::InsufficientBalance((
                args.path[0].pair.0,
                balance_in,
            )));
        }

        // transfer first
        token_balances.token_transfer(
            args.path[0].pair.0,
            args.from,
            pool_accounts[0],
            amounts[0].clone(),
        );

        self.swap(
            token_balances,
            self_canister,
            &amounts,
            &args.path,
            &pas,
            &pool_accounts,
            args.to,
        )?;

        Ok(TokenPairSwapTokensSuccess { amounts })
    }
}
