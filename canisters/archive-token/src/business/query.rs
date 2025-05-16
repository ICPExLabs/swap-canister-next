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
fn get_block(block_height: BlockIndex) -> Option<TokenBlock> {
    with_state(|s| s.business_blocks_get(block_height, 1))
        .ok()
        .and_then(|mut r| r.pop())
        .map(|b| trap(b.try_into()))
}

#[ic_cdk::query(guard = "has_business_queryable")]
fn get_blocks_by(block_height: BlockIndex, length: u64) -> Vec<(BlockIndex, Option<TokenBlock>)> {
    assert!(length <= MAX_BLOCKS_PER_REQUEST, "length too large");
    with_state(|s| {
        (block_height..(block_height + length))
            .map(|block_height| {
                let block = s.business_blocks_get(block_height, 1);
                (
                    block_height,
                    block.ok().and_then(|mut r| r.pop()).and_then(|b| b.try_into().ok()),
                )
            })
            .collect()
    })
}
