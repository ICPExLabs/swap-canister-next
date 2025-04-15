use candid::CandidType;
use common::types::TokenAccount;
use serde::{Deserialize, Serialize};

use super::{SwapBlockChainGuard, TokenBalancesGuard, TokenBlockChainGuard};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct BusinessLocks {
    /// 是否有余额锁
    balances: Option<Vec<TokenAccount>>,
    /// 是否有 token 锁
    token: Option<bool>,
    /// 是否有 swap 锁
    swap: Option<bool>,
}

impl BusinessLocks {
    pub fn new(
        balances: Option<&TokenBalancesGuard<'_>>,
        token: Option<&TokenBlockChainGuard<'_>>,
        swap: Option<&SwapBlockChainGuard<'_>>,
    ) -> Self {
        Self {
            balances: balances.map(|g| g.get_locked_balances()),
            token: token.map(|_| true),
            swap: swap.map(|_| true),
        }
    }
}
