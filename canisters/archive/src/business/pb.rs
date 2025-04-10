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
    let get_block_request: GetBlockRequest = trap(from_proto_bytes(&arg[..]));
    let response = with_state(|s| s.business_block_query(get_block_request.block_height));
    let response = GetBlockResponse {
        block: response.map(|bytes| EncodedBlock {
            block: bytes.into(),
        }),
    };
    trap(to_proto_bytes(&response))
}
