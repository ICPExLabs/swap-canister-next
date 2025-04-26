mod transform;

mod common;
pub use common::*;

mod swap;
pub use swap::*;

mod block;
pub use block::*;

#[allow(missing_docs)]
mod request;
pub use request::*;

pub use candid::Nat;
/// canister id, alias of principal
pub type CanisterId = candid::Principal;
/// user id, alias of principal
pub type UserId = candid::Principal;

#[allow(missing_docs)]
pub trait CheckArgs {
    type Result;
    fn check_args(&self) -> Result<Self::Result, BusinessError>;
}
