use crate::utils::pb::to_proto_bytes;

use super::*;

#[test]
fn test_block_height() {
    let arg = GetBlockRequest { block_height: 0 };
    let bytes = to_proto_bytes(&arg).unwrap();
    assert_eq!(hex::encode(bytes), "".to_string());

    let arg = GetBlockRequest { block_height: 1 };
    let bytes = to_proto_bytes(&arg).unwrap();
    assert_eq!(hex::encode(bytes), "0801".to_string());
}

#[test]
fn test_blocks() {
    let arg = GetBlocksRequest { start: 0, length: 100 };
    let bytes = to_proto_bytes(&arg).unwrap();
    assert_eq!(hex::encode(bytes), "1064".to_string());
}
