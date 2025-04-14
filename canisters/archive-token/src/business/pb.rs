#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

/// arg: GetBlockRequest
/// ret: GetBlockResponse
#[ic_cdk::query(guard = "has_business_queryable")]
fn get_block_pb(arg: Vec<u8>) -> Vec<u8> {
    let proto::GetBlockRequest { block_height } =
        trap(from_proto_bytes(&arg[..]).map_err(|_| "failed to decode get_block_pb argument"));
    let block = with_state(|s| s.business_block_query(block_height));
    let response = proto::GetBlockResponse {
        block: block.map(|block| block.into()),
    };
    trap(to_proto_bytes(&response).map_err(|_| "failed to encode get_block_pb response"))
}

/// 本罐子存储的 Block，参数不是对应的 height
/// arg: IterBlocksRequest
/// ret: IterBlocksResponse
#[ic_cdk::query(guard = "has_business_queryable")]
fn iter_blocks_pb(arg: Vec<u8>) -> Vec<u8> {
    let proto::IterBlocksRequest {
        start: index_start,
        length,
    } = trap(from_proto_bytes(&arg[..]).map_err(|_| "failed to decode iter_blocks_pb argument"));
    let blocks = with_state(|s| s.business_blocks_iter(index_start, length));
    let response = proto::IterBlocksResponse {
        blocks: blocks.into_iter().map(|block| block.into()).collect(),
    };
    trap(to_proto_bytes(&response).map_err(|_| "failed to encode iter_blocks_pb response"))
}

/// 本罐子存储的 Block，参数是对应的 height
/// arg: GetBlocksRequest
/// ret: GetBlocksResponse
#[ic_cdk::query(guard = "has_business_queryable")]
fn get_blocks_pb(arg: Vec<u8>) -> Vec<u8> {
    let proto::GetBlocksRequest {
        start: height_start,
        length,
    } = trap(from_proto_bytes(&arg[..]).map_err(|_| "failed to decode get_blocks_pb argument"));
    let response = with_state(|s| s.business_blocks_query(height_start, length));
    let response = match response {
        Ok(blocks) => proto::get_blocks_response::GetBlocksContent::Blocks(proto::EncodedBlocks {
            blocks: blocks.into_iter().map(|block| block.into()).collect(),
        }),
        Err(message) => proto::get_blocks_response::GetBlocksContent::Error(message),
    };
    let response = proto::GetBlocksResponse {
        get_blocks_content: Some(response),
    };
    trap(to_proto_bytes(&response).map_err(|_| "failed to encode get_blocks_pb response"))
}
