use ::common::types::{Amm, AmmText, TokenPair};

mod create;
pub use create::*;

mod liquidity;
pub use liquidity::*;

mod swap;
pub use swap::*;

mod loan;
pub use loan::*;

use super::*;

/// (token0, token1, amm)
#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairPool {
    pub pair: (CanisterId, CanisterId),
    pub amm: AmmText,
}

pub fn token_pair_to_pool(pair: &TokenPair, amm: Amm) -> TokenPairPool {
    TokenPairPool {
        pair: (pair.token0, pair.token1),
        amm: amm.into(),
    }
}
