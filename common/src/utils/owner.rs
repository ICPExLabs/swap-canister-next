#[cfg(feature = "cdk")]
use ic_canister_kit::{
    identity::{caller, self_canister_id},
    types::UserId,
};

/// check owner
#[cfg(feature = "cdk")]
pub fn check_owner_for_token_balance_of(owner: &UserId) {
    if *owner != caller() && *owner != self_canister_id() {
        ic_cdk::trap("You can only query your own balance")
    }
}
