use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::types::TokenAccount;

#[derive(Debug, Clone, Default, Serialize, Deserialize, CandidType)]
pub struct BusinessLocks {
    /// Is there a token lock
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<bool>,
    /// Is there a swap lock
    #[serde(skip_serializing_if = "Option::is_none")]
    swap: Option<bool>,
    /// Is there a balance lock
    #[serde(skip_serializing_if = "Option::is_none")]
    balances: Option<Vec<TokenAccount>>,
}

impl BusinessLocks {
    pub fn new(token: Option<bool>, swap: Option<bool>, balances: Option<Vec<TokenAccount>>) -> Self {
        Self { token, swap, balances }
    }
}
