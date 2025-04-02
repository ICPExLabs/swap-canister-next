#[allow(unused)]
pub use serde::{Deserialize, Serialize};

#[allow(unused)]
pub use candid::{CandidType, Nat};

#[allow(unused)]
pub use ic_canister_kit::types::*;

#[allow(unused)]
pub use crate::stable::*;

#[allow(unused)]
pub use icrc_ledger_types::icrc1::account::Account;

// ==================== business ====================

// token
mod token;
#[allow(unused)]
pub use token::*;

// ==================== types ====================

mod error;
#[allow(unused)]
pub use error::*;

pub trait CheckArgs {
    type Result;
    fn check_args(&self) -> Result<Self::Result, BusinessError>;
}
