use crate::types::{Business, CheckArgs, TokenAccount, TokenPairPool, with_state};

use super::{Amm, BusinessError, PairAmm, SelfCanister, TokenPair};

// for pool exist checking and lock accounts
pub fn check_pool(
    pool: &TokenPairPool,
    self_canister: &SelfCanister,
) -> Result<(PairAmm, Vec<TokenAccount>), BusinessError> {
    let TokenPairPool {
        pair: (token_a, token_b),
        amm,
    } = pool;
    let pair = TokenPair::new(*token_a, *token_b);
    pair.check_args()?; // check supported token
    let amm: Amm = amm.try_into()?; // parse amm
    let pa = PairAmm { pair, amm };
    // check pool exist
    let accounts = with_state(|s| {
        s.business_token_pair_pool_maker_get(&pa)
            .map(|maker| maker.accounts(self_canister))
    })
    .ok_or(pa.not_exist())?;

    let accounts = accounts
        .into_iter()
        .flat_map(|account| {
            vec![
                TokenAccount::new(pa.pair.token0, account),
                TokenAccount::new(pa.pair.token1, account),
            ]
        })
        .collect();

    Ok((pa, accounts))
}
