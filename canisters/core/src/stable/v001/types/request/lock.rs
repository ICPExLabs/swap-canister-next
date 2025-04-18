use candid::CandidType;
use common::types::TokenAccount;
use serde::{Deserialize, Serialize};

use super::{SwapBlockChainGuard, TokenBalancesGuard, TokenBlockChainGuard};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct BusinessLocks {
    /// 是否有 token 锁
    token: Option<bool>,
    /// 是否有 swap 锁
    swap: Option<bool>,
    /// 是否有余额锁
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
