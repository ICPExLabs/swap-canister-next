#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== query pairs ==========================

// anyone can query
#[ic_cdk::query]
fn pairs_query() -> Vec<(TokenPairPool, MarketMakerView)> {
    with_state(|s| {
        s.business_token_pair_pools_query()
            .into_iter()
            .map(|(pair, amm, maker)| (pair.to_pool(amm), maker.clone().into()))
            .collect()
    })
}

// anyone can query
#[ic_cdk::query]
fn pair_query(pool: TokenPairPool) -> Option<MarketMakerView> {
    let pair = TokenPair::new(pool.pair.0, pool.pair.1);
    let amm: Amm = (&pool.amm).try_into().ok()?; // parse amm

    with_state(|s| {
        s.business_token_pair_pools_query()
            .into_iter()
            .find(|(p, a, _maker)| **p == pair && **a == amm)
            .map(|(_p, _a, maker)| maker.clone().into())
    })
}
