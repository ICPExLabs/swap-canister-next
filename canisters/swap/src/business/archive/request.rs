#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

#[ic_cdk::query(guard = "has_business_config_maintaining")]
async fn request_trace_index_get() -> (RequestIndex, u64) {
    with_state(|s| s.business_request_index_get())
}

#[ic_cdk::query(guard = "has_business_config_maintaining")]
async fn request_trace_get(index: RequestIndex) -> Option<RequestTrace> {
    with_state(|s| s.business_request_trace_get(&index))
}

#[ic_cdk::update(guard = "has_business_config_maintaining")]
async fn request_trace_remove(index: RequestIndex) -> Option<RequestTrace> {
    with_mut_state(|s| s.business_request_trace_remove(&index))
}

#[ic_cdk::query(guard = "has_business_config_maintaining")]
async fn request_traces_get(start: RequestIndex, length: u64) -> Vec<Option<RequestTrace>> {
    with_state(|s| {
        let mut list = Vec::with_capacity(length as usize);
        for i in 0..length {
            let index = start + i;
            list.push(s.business_request_trace_get(&index));
        }
        list
    })
}

#[ic_cdk::update(guard = "has_business_config_maintaining")]
async fn request_traces_remove(start: RequestIndex, length: u64) -> Vec<Option<RequestTrace>> {
    with_mut_state(|s| {
        let mut list = Vec::with_capacity(length as usize);
        for i in 0..length {
            let index = start + i;
            list.push(s.business_request_trace_remove(&index));
        }
        list
    })
}
