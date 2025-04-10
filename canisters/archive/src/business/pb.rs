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
#[ic_cdk::query]
fn get_block_pb(arg: Vec<u8>) -> Vec<u8> {
    let get_block_request: GetBlockRequest =
        trap(from_proto_bytes(&arg[..]).map_err(|_| "failed to decode get_block_pb argument"));
    let response = with_state(|s| s.business_block_query(get_block_request.block_height));
    let response = GetBlockResponse {
        block: response.map(|bytes| EncodedBlock {
            block: bytes.into(),
        }),
    };
    trap(to_proto_bytes(&response).map_err(|_| "failed to encode get_block_pb response"))
}

/// 本罐子存储的 Block，参数不是对应的 height
/// arg: IterBlocksRequest
/// ret: IterBlocksResponse
#[ic_cdk::query]
fn iter_blocks_pb(arg: Vec<u8>) -> Vec<u8> {
    let IterBlocksRequest { start, length } =
        trap(from_proto_bytes(&arg[..]).map_err(|_| "failed to decode iter_blocks_pb argument"));
    let response = with_state(|s| s.business_blocks_iter(start, length));
    let response = IterBlocksResponse {
        blocks: response
            .into_iter()
            .map(|bytes| EncodedBlock {
                block: bytes.into(),
            })
            .collect(),
    };
    trap(to_proto_bytes(&response).map_err(|_| "failed to encode iter_blocks_pb response"))
}

/// 本罐子存储的 Block，参数是对应的 height
/// arg: GetBlocksRequest
/// ret: GetBlocksResponse
#[ic_cdk::query]
fn get_blocks_pb(arg: Vec<u8>) -> Vec<u8> {
    let IterBlocksRequest { start, length } =
        trap(from_proto_bytes(&arg[..]).map_err(|_| "failed to decode get_blocks_pb argument"));
    let response = with_state(|s| s.business_blocks_query(start, length));
    let response = match response {
        Ok(blocks) => get_blocks_response::GetBlocksContent::Blocks(EncodedBlocks {
            blocks: blocks
                .into_iter()
                .map(|bytes| EncodedBlock {
                    block: bytes.into(),
                })
                .collect(),
        }),
        Err(message) => get_blocks_response::GetBlocksContent::Error(message),
    };
    let response = GetBlocksResponse {
        get_blocks_content: Some(response),
    };
    trap(to_proto_bytes(&response).map_err(|_| "failed to encode get_blocks_pb response"))
}
