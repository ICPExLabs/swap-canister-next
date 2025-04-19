use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, CandidType)]
pub struct FeeTo {
    /// The handling fee charged for preventing witch attacks
    /// token's transfer usage
    /// burn usage of liquidity
    pub token_fee_to: Option<Account>,
    /// Swap agreement fee is charged, 1/6
    pub swap_fee_to: Option<Account>,
}
