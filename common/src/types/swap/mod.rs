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
    /// 代币 0
    pub token0: CanisterId,
    /// 代币 1
    pub token1: CanisterId,
    /// amm 算法
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

/// 注意兑换方向
/// token_a -> token_b
/// (token_a, token_b, amm)
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwap {
    /// 代币 a, 代币 b
    pub token: (CanisterId, CanisterId),
    /// amm 算法
    pub amm: AmmText,
}
