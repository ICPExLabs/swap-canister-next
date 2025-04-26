use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::types::TokenAccount;

#[derive(Debug, Default, Serialize, Deserialize, CandidType)]
pub struct BusinessLocks {
    /// Is there a token lock
    token: Option<bool>,
    /// Is there a swap lock
    swap: Option<bool>,
    /// Is there a balance lock
    balances: Option<Vec<TokenAccount>>,
}

impl BusinessLocks {
    pub fn new(token: Option<bool>, swap: Option<bool>, balances: Option<Vec<TokenAccount>>) -> Self {
        Self { token, swap, balances }
    }
}
