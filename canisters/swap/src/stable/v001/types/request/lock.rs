use candid::CandidType;
use common::types::TokenAccount;
use serde::{Deserialize, Serialize};

use super::{SwapBlockChainGuard, TokenBalancesGuard, TokenBlockChainGuard};

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
    pub fn new(
        token: Option<&TokenBlockChainGuard<'_>>,
        swap: Option<&SwapBlockChainGuard<'_>>,
        balances: Option<&TokenBalancesGuard<'_>>,
    ) -> Self {
        Self {
            token: token.map(|_| true),
            swap: swap.map(|_| true),
            balances: balances.map(|g| g.get_locked_balances()),
        }
    }
}
