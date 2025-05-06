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

/// The `iter_blocks_pb` function in Rust processes a request to iterate through blocks stored in a
/// container based on specified parameters.
///
/// Arguments:
///
/// * `arg`: The `arg` parameter for the `iter_blocks_pb` function is of type `Vec<u8>`, which
///   represents a vector of bytes. This parameter is used to pass the request data in the form of a
///   serialized protobuf message of type `IterBlocksRequest`. note: start means the index at this canister.
///
/// Returns:
///
/// The function `iter_blocks_pb` returns a `Vec<u8>` which contains the response data encoded in the
/// protocol buffer format. The response data is generated based on the input argument `arg` of type
/// `Vec<u8>` which is decoded into an `IterBlocksRequest` struct. The function processes this request,
/// retrieves blocks from the state, constructs an `IterBlocksResponse` struct with the
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

/// This Rust function retrieves blocks based on a given height range and returns the response in a
/// protobuf format.
///
/// Arguments:
///
/// * `arg`: The `arg` parameter in the `get_blocks_pb` function represents the input argument for
///   retrieving blocks. It is of type `Vec<u8>`, which is a vector of unsigned bytes. The function
///   decodes this argument into a `GetBlocksRequest` struct. note: start means block height.
///
/// Returns:
///
/// The function `get_blocks_pb` returns a serialized `GetBlocksResponse` message in the form of a byte
/// vector (`Vec<u8>`). The response contains either a list of blocks or an error message, depending on
/// the outcome of the `business_blocks_query` function call.
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
