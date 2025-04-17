#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

#[ic_cdk::query]
fn query_latest_block_index() -> Option<BlockIndex> {
    with_state(|s| s.business_latest_block_index_query())
}

#[ic_cdk::query]
fn query_metrics() -> CustomMetrics {
    with_state(|s| s.business_metrics_query())
}
