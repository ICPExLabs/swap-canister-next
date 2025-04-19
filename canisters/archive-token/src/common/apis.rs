use crate::stable::*;

// ================== 通用接口 ==================

#[ic_cdk::query]
pub fn wallet_balance() -> candid::Nat {
    ic_canister_kit::canister::cycles::wallet_balance()
}

#[ic_cdk::update]
pub fn wallet_receive() -> candid::Nat {
    ic_canister_kit::canister::cycles::wallet_receive(|_accepted| {})
}

// ================== 数据版本 ==================

// 当前数据库版本
#[ic_cdk::query]
fn version() -> u32 {
    with_state(|s| s.version())
}
