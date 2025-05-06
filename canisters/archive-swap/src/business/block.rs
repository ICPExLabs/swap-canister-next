#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

/// This Rust function `get_blocks` queries swap blocks based on the provided arguments with a guard
/// condition `has_business_queryable`.
///
/// Arguments:
///
/// * `args`: The `args` parameter in the `get_blocks` function likely represents the arguments needed
///   to retrieve blocks for swapping. These arguments could include information such as block height,
///   block timestamp, or any other parameters necessary to query and retrieve the desired blocks for
///   swapping.
///
/// Returns:
///
/// The function `get_blocks` is returning a `GetSwapBlocksResult` value.
#[ic_cdk::query(guard = "has_business_queryable")]
fn get_blocks(args: GetBlocksArgs) -> GetSwapBlocksResult {
    inner_get_blocks(args).into()
}
fn inner_get_blocks(args: GetBlocksArgs) -> Result<SwapBlockRange, GetBlocksError> {
    let GetBlocksArgs {
        start: height_start,
        length,
    } = args;
    let response = with_state(|s| s.business_blocks_get(height_start, length))?;
    let blocks = trap(
        response
            .into_iter()
            .map(|block| block.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "failed to decode a block"),
    );
    Ok(SwapBlockRange { blocks })
}

/// The function `get_encoded_blocks` retrieves encoded blocks based on the provided arguments.
///
/// Arguments:
///
/// * `args`: The `args` parameter in the `get_encoded_blocks` function likely represents the arguments
///   needed to retrieve encoded blocks. These arguments could include information such as block height,
///   timestamps, or any other parameters required to fetch the encoded blocks from the underlying data
///   source. The specific structure and content of the `Get
///
/// Returns:
///
/// The function `get_encoded_blocks` is returning a `GetEncodedBlocksResult` which is the result of
/// calling the `inner_get_encoded_blocks` function with the provided arguments `args`.
#[ic_cdk::query(guard = "has_business_queryable")]
fn get_encoded_blocks(args: GetBlocksArgs) -> GetEncodedBlocksResult {
    inner_get_encoded_blocks(args).into()
}
fn inner_get_encoded_blocks(args: GetBlocksArgs) -> Result<Vec<EncodedBlock>, GetBlocksError> {
    let GetBlocksArgs {
        start: height_start,
        length,
    } = args;
    let response = with_state(|s| s.business_blocks_get(height_start, length))?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use candid::Principal;
    use common::{
        archive::swap::{PairCreate, PairOperation, SwapBlock, SwapOperation, SwapTransaction},
        types::{CandidBlock, HashOf, TimestampNanos, TokenPair, TokenPairAmm},
    };

    use super::*;

    #[test]
    fn test() {
        let user_id = Principal::from_text("aaaaa-aa").unwrap();
        let block = SwapBlock(CandidBlock {
            parent_hash: HashOf::default(),
            timestamp: TimestampNanos::from_inner(0),
            transaction: SwapTransaction {
                operation: SwapOperation::Pair(PairOperation::Create(PairCreate {
                    pa: TokenPairAmm {
                        pair: TokenPair {
                            token0: Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
                            token1: Principal::from_text("lvfsa-2aaaa-aaaaq-aaeyq-cai").unwrap(),
                        },
                        amm: "swap_v2_0.3%".try_into().unwrap(),
                    },
                    creator: user_id,
                })),
                memo: None,
                created: None,
            },
        });
        let proto_block: proto::SwapBlock = block.clone().try_into().unwrap();
        let bytes = to_proto_bytes(&proto_block).unwrap();
        assert_eq!(hex::encode(&bytes), "0a220a2000000000000000000000000000000000000000000000000000000000000000001a360a340a320a300a2c0a1c0a0c0a0a00000000000000020101120c0a0a00000000020001310101120c737761705f76325f302e33251200".to_string());
        let proto_block2: proto::SwapBlock = from_proto_bytes(&bytes).unwrap();
        assert_eq!(proto_block, proto_block2);
        let block2: SwapBlock = proto_block2.try_into().unwrap();
        assert_eq!(block, block2);
    }
}
