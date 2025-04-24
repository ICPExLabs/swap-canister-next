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

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, CandidType)]
pub struct FeeToView {
    pub token_fee_to: bool,
    pub swap_fee_to: bool,
}

impl From<FeeTo> for FeeToView {
    fn from(fee_to: FeeTo) -> Self {
        Self {
            token_fee_to: fee_to.token_fee_to.is_some(),
            swap_fee_to: fee_to.swap_fee_to.is_some(),
        }
    }
}
