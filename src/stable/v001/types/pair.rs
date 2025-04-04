use std::collections::HashMap;

use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};

use super::{Amm, BusinessError, MarketMaker, TokenInfo, TokenPair};

#[derive(Serialize, Deserialize, Default)]
pub struct TokenPairs(HashMap<TokenPair, HashMap<Amm, MarketMaker>>);

impl TokenPairs {
    pub fn query_token_pair_pools(&self) -> Vec<(&TokenPair, &Amm, &MarketMaker)> {
        self.0
            .iter()
            .flat_map(|(pool, makers)| makers.iter().map(move |(amm, maker)| (pool, amm, maker)))
            .collect()
    }

    pub fn is_token_pair_pool_exist(&self, pool: &TokenPair, amm: &Amm) -> bool {
        self.0
            .get(pool)
            .is_some_and(|makers| makers.contains_key(amm))
    }

    pub fn create_token_pair_pool(
        &mut self,
        pool: TokenPair,
        amm: Amm,
        subaccount: Subaccount,
        token0: &TokenInfo,
        token1: &TokenInfo,
    ) -> Result<(), BusinessError> {
        if self.is_token_pair_pool_exist(&pool, &amm) {
            return Err(BusinessError::TokenPairAmmExist((pool, (&amm).into())));
        }

        let maker = MarketMaker::new_by_pair(&amm, subaccount, token0, token1);

        let makers = self.0.entry(pool).or_default();
        makers.insert(amm, maker);

        Ok(())
    }
}
