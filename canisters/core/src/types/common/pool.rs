use common::types::{Amm, BusinessError, SelfCanister, TokenAccount, TokenPair, TokenPairAmm};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;

use crate::types::{Business, CheckArgs, TokenPairPool, with_state};

// for pool exist checking and lock accounts
pub fn check_pool(
    pool: &TokenPairPool,
    self_canister: &SelfCanister,
    liquidity: Option<&Account>,
) -> Result<(TokenPairAmm, Vec<CanisterId>, Vec<TokenAccount>), BusinessError> {
    let TokenPairPool {
        pair: (token_a, token_b),
        amm,
    } = pool;
    let pair = TokenPair::new(*token_a, *token_b);
    pair.check_args()?; // check supported token
    let amm: Amm = amm.as_ref().try_into()?; // parse amm
    let pa = TokenPairAmm { pair, amm };
    // check pool exist
    let (required, dummy_tokens) = with_state(|s| {
        s.business_token_pair_pool_maker_get(&pa)
            .map(|maker| (maker.accounts(self_canister), maker.dummy_canisters()))
    })
    .ok_or(pa.not_exist())?;

    let mut required: Vec<TokenAccount> = required
        .into_iter()
        .flat_map(|account| {
            vec![
                TokenAccount::new(pa.pair.token0, account), // self pool account of token0
                TokenAccount::new(pa.pair.token1, account), // self pool account of token1
            ]
        })
        .collect();

    if let Some(liquidity) = liquidity {
        for token in &dummy_tokens {
            required.push(TokenAccount::new(*token, *liquidity)); // liquidity of account would be changed
        }
    }

    Ok((pa, dummy_tokens, required))
}
