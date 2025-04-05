use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

use crate::types::{Amm, TokenPair};

use super::super::AmmText;

mod create;
pub use create::*;

mod liquidity;
pub use liquidity::*;

mod swap;
pub use swap::*;

/// (token0, token1, amm)
#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairPool {
    pub pair: (CanisterId, CanisterId),
    pub amm: AmmText,
}

impl TokenPair {
    pub fn to_pool(self, amm: &Amm) -> TokenPairPool {
        TokenPairPool {
            pair: (self.token0, self.token1),
            amm: amm.into(),
        }
    }
}
