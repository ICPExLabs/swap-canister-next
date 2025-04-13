#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

/// 本罐子存储的 Block，参数是对应的 height
#[ic_cdk::query(guard = "has_business_maintaining")]
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

/// 本罐子存储的 Block，参数是对应的 height
#[ic_cdk::query(guard = "has_business_maintaining")]
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
        common::{CandidBlock, HashOf, TimestampNanos},
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
                }),
                memo: None,
                created: None,
            },
        });
        let proto_block: proto::TokenBlock = block.clone().try_into().unwrap();
        let bytes = to_proto_bytes(&proto_block).unwrap();
        assert_eq!(hex::encode(&bytes), "0a220a2000000000000000000000000000000000000000000000000000000000000000001a190a170a150a0c0a0a0000000000000002010112001a030a0164".to_string());
        let proto_block2: proto::TokenBlock = from_proto_bytes(&bytes).unwrap();
        assert_eq!(proto_block, proto_block2);
        let block2: TokenBlock = proto_block2.try_into().unwrap();
        assert_eq!(block, block2);
    }
}
