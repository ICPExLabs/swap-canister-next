#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

#[ic_cdk::query]
fn remaining_capacity() -> u64 {
    with_state(|s| s.business_remaining_capacity())
}
