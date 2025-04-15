#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

#[ic_cdk::update(guard = "has_pause_replace")]
async fn request_trace_index_get() -> (RequestIndex, u64) {
    with_state(|s| s.business_request_index_get())
}

#[ic_cdk::update(guard = "has_pause_replace")]
async fn request_trace_get(index: RequestIndex) -> Option<RequestTrace> {
    with_state(|s| s.business_request_trace_get(&index))
}

#[ic_cdk::update(guard = "has_pause_replace")]
async fn request_trace_remove(index: RequestIndex) -> Option<RequestTrace> {
    with_mut_state_without_record(|s| s.business_request_trace_remove(&index))
}
