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
/// to retrieve blocks for swapping. These arguments could include information such as block height,
/// block timestamp, or any other parameters necessary to query and retrieve the desired blocks for
/// swapping.
///
/// Returns:
///
/// The function `get_blocks` is returning a `GetSwapBlocksResult` value.
#[ic_cdk::query(guard = "has_business_queryable")]
fn get_blocks(args: GetBlocksArgs) -> GetTokenBlocksResult {
    inner_get_blocks(args).into()
}
fn inner_get_blocks(args: GetBlocksArgs) -> Result<TokenBlockRange, GetBlocksError> {
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
    Ok(TokenBlockRange { blocks })
}

/// The function `get_encoded_blocks` retrieves encoded blocks based on the provided arguments.
///
/// Arguments:
///
/// * `args`: The `args` parameter in the `get_encoded_blocks` function likely represents the arguments
/// needed to retrieve encoded blocks. These arguments could include information such as block height,
/// timestamps, or any other parameters required to fetch the encoded blocks from the underlying data
/// source. The specific structure and content of the `Get
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
        archive::token::{DepositToken, TokenBlock, TokenOperation, TokenTransaction},
        types::{CandidBlock, HashOf, TimestampNanos},
    };
    use ic_canister_kit::types::CanisterId;
    use icrc_ledger_types::icrc1::account::Account;

    use super::*;

    #[test]
    fn test() {
        let user_id = Principal::from_text("aaaaa-aa").unwrap();
        let block = TokenBlock(CandidBlock {
            parent_hash: HashOf::default(),
            timestamp: TimestampNanos::from_inner(0),
            transaction: TokenTransaction {
                operation: TokenOperation::Deposit(DepositToken {
                    from: Account {
                        owner: user_id,
                        subaccount: None,
                    },
                    token: CanisterId::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
                    amount: candid::Nat::from(100_u64),
                    to: Account {
                        owner: user_id,
                        subaccount: None,
                    },
                }),
                memo: None,
                created: None,
            },
        });
        let proto_block: proto::TokenBlock = block.clone().try_into().unwrap();
        let bytes = to_proto_bytes(&proto_block).unwrap();
        assert_eq!(hex::encode(&bytes), "0a220a2000000000000000000000000000000000000000000000000000000000000000001a1b0a190a170a0c0a0a0000000000000002010112001a030a01642200".to_string());
        let proto_block2: proto::TokenBlock = from_proto_bytes(&bytes).unwrap();
        assert_eq!(proto_block, proto_block2);
        let block2: TokenBlock = proto_block2.try_into().unwrap();
        assert_eq!(block, block2);
    }
}
