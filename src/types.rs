#[allow(unused)]
pub use candid::{CandidType, Nat};

#[allow(unused)]
pub use ic_canister_kit::types::*;

#[allow(unused)]
pub use crate::stable::*;

// ==================== common types ====================

pub mod common;
#[allow(unused)]
pub use common::*;

// ==================== business ====================

// business
pub mod business;
#[allow(unused)]
pub use business::*;

pub trait CheckArgs {
    type Result;
    fn check_args(&self) -> Result<Self::Result, BusinessError>;
}
