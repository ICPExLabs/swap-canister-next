use crate::stable::*;

// ================== general apis ==================

#[ic_cdk::query]
pub fn wallet_balance() -> candid::Nat {
    ic_canister_kit::canister::cycles::wallet_balance()
}

#[ic_cdk::update]
pub fn wallet_receive() -> candid::Nat {
    ic_canister_kit::canister::cycles::wallet_receive(|_accepted| {})
}

// ================== data version ==================

// current data version
#[ic_cdk::query]
fn version() -> u32 {
    with_state(|s| s.version())
}
