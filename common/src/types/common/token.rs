use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::types::CanisterId;

// =================================== token info ===================================

/// token info
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct TokenInfo {
    /// canister id
    pub canister_id: CanisterId,
    /// token name
    #[allow(unused)]
    pub name: String,
    /// token symbol
    #[allow(unused)]
    pub symbol: String,
    /// token decimals
    #[allow(unused)]
    pub decimals: u8,
    /// token fee
    pub fee: Nat,
    /// is lp token or not
    pub is_lp_token: bool,
}

impl TokenInfo {
    /// create a new token info
    #[cfg(feature = "cdk")]
    pub fn new(
        canister_id: &'static str,
        name: &'static str,
        symbol: &'static str,
        decimals: u8,
        fee: u128,
        is_lp_token: bool,
    ) -> Self {
        use ic_canister_kit::common::trap;
        Self {
            canister_id: trap(CanisterId::from_text(canister_id)),
            name: name.into(),
            symbol: symbol.into(),
            decimals,
            fee: Nat::from(fee),
            is_lp_token,
        }
    }
}
