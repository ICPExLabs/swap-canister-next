#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

/// 本罐子存储的 Block，参数是对应的 height
#[ic_cdk::query(guard = "has_business_swap_queryable")]
fn block_swap_get(block_height: BlockIndex) -> QuerySwapBlockResult {
    inner_block_swap_get(block_height).into()
}
fn inner_block_swap_get(block_height: BlockIndex) -> QueryBlockResult<SwapBlock> {
    use QueryBlockResult::*;
    let response = with_state(|s| s.business_swap_block_get(block_height));
    match response {
        Block(block) => {
            let block: SwapBlock = trap(block.try_into());
            Block(block)
        }
        Archive(canister_id) => Archive(canister_id),
    }
}
