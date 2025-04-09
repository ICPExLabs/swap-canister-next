use ic_canister_kit::{
    identity::{caller, self_canister_id},
    types::UserId,
};

pub fn check_owner_for_token_balance_of(owner: &UserId) {
    #[allow(clippy::panic)] // ? SAFETY
    if *owner != caller() && *owner != self_canister_id() {
        panic!("You can only query your own balance")
    }
}
