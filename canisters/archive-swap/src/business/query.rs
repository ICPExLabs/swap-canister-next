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
