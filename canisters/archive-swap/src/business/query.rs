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

#[ic_cdk::query(guard = "has_business_queryable")]
fn get_block(block_height: BlockIndex) -> Option<SwapBlock> {
    with_state(|s| s.business_blocks_get(block_height, 1))
        .ok()
        .and_then(|mut r| r.pop())
        .map(|b| trap(b.try_into()))
}
