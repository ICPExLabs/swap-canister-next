#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

/// 本罐子存储的 Block，参数是对应的 height
#[ic_cdk::query(guard = "has_business_token_queryable")]
fn block_token_get(block_height: BlockIndex) -> QueryTokenBlockResult {
    inner_block_token_get(block_height).into()
}
fn inner_block_token_get(block_height: BlockIndex) -> QueryBlockResult<TokenBlock> {
    use QueryBlockResult::*;
    let response = with_state(|s| s.business_token_block_get(block_height));
    match response {
        Block(block) => {
            let block: TokenBlock = trap(block.try_into());
            Block(block)
        }
        Archive(canister_id) => Archive(canister_id),
    }
}
