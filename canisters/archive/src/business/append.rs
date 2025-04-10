#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

#[ic_cdk::update(guard = "has_business_blocks_append")]
fn append_blocks(args: Vec<Vec<u8>>) {
    with_mut_state_without_record(|s| s.business_blocks_append(args))
}
