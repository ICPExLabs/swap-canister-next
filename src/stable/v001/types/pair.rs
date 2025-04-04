use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Amm, MarketMaker, TokenPair};

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
}
