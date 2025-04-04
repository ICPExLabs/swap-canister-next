#[allow(unused)]
pub use serde::{Deserialize, Serialize};

#[allow(unused)]
pub use candid::{CandidType, Nat};

#[allow(unused)]
pub use ic_canister_kit::types::*;

#[allow(unused)]
pub use crate::stable::*;

#[allow(unused)]
pub use icrc_ledger_types::icrc1::account::{Account, Subaccount};

// ==================== common types ====================

mod common;
#[allow(unused)]
pub use common::*;

// ==================== business ====================

// business
mod business;
#[allow(unused)]
pub use business::*;

pub trait CheckArgs {
    type Result;
    fn check_args(&self) -> Result<Self::Result, BusinessError>;
}
