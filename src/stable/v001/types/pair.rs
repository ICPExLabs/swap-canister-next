use std::collections::HashMap;

use candid::Nat;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::{Deserialize, Serialize};

use super::{
    Amm, BusinessError, DummyCanisterId, MarketMaker, PairAmm, SelfCanister, TokenBalances,
    TokenInfo, TokenPair, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess,
    TokenPairLiquidityRemoveArg, TokenPairLiquidityRemoveSuccess,
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

    pub fn add_liquidity(
        &mut self,
        fee_to: Option<Account>,
        token_balances: &mut TokenBalances,
        self_canister: SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        let maker = self
            .0
            .get_mut(&pa.pair)
            .and_then(|makers| makers.get_mut(&pa.amm))
            .ok_or_else(|| pa.not_exist())?;

        maker.add_liquidity(fee_to, token_balances, self_canister, pa, arg)
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
        self_canister: SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        let maker = self
            .0
            .get_mut(&pa.pair)
            .and_then(|makers| makers.get_mut(&pa.amm))
            .ok_or_else(|| pa.not_exist())?;

        maker.remove_liquidity(fee_to, token_balances, self_canister, pa, arg)
    }
}
