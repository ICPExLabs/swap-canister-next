use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

/// amm
mod amm;
pub use amm::*;

/// pair
mod pair;
pub use pair::*;

// =================== token pair pool ===================

/// (token0, token1, amm)
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairPool {
    /// token 0
    pub token0: CanisterId,
    /// token 1
    pub token1: CanisterId,
    /// amm algorithm
    pub amm: AmmText,
}

impl From<TokenPairAmm> for TokenPairPool {
    fn from(value: TokenPairAmm) -> Self {
        Self {
            token0: value.pair.token0,
            token1: value.pair.token1,
            amm: value.amm.into(),
        }
    }
}

// =================== token pair swap ===================

/// Pay attention to the redemption direction
/// token_a -> token_b
/// (token_a, token_b, amm)
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SwapTokenPair {
    /// token a, token b
    pub token: (CanisterId, CanisterId),
    /// amm algorithm
    pub amm: AmmText,
}
