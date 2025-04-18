#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

#[ic_cdk::update(guard = "has_business_blocks_append")]
fn set_maintainers(maintainers: Option<Vec<UserId>>) {
    with_mut_state_without_record(|s| s.business_config_maintainers_set(maintainers))
}

#[ic_cdk::update(guard = "has_business_blocks_append")]
fn set_max_memory_size_bytes(max_memory_size_bytes: u64) {
    with_mut_state_without_record(|s| {
        s.business_config_max_memory_size_bytes_set(max_memory_size_bytes)
    })
}
