//! https://github.com/dfinity/pocketic
use candid::{Principal, encode_one};
use pocket_ic::PocketIc;

mod archive_swap;

// 2T cycles
const INIT_CYCLES: u128 = 2_000_000_000_000;

const WASM_MODULE: &[u8] = include_bytes!("../../archive-swap/sources/source_opt.wasm");

#[ignore]
#[test]
#[rustfmt::skip]
fn test_archive_swap_business_apis() {
    let pic = PocketIc::new();

    let default_identity = Principal::from_text("2ibo7-dia").unwrap();
    let alice_identity = Principal::from_text("uuc56-gyb").unwrap();
    let bob_identity = Principal::from_text("hqgi5-iic").unwrap(); // cspell: disable-line
    let carol_identity = Principal::from_text("jmf34-nyd").unwrap();
    let anonymous_identity = Principal::from_text("2vxsx-fae").unwrap();

    let canister_id = Principal::from_text("hcnys-xiaaa-aaaai-q3w4q-cai").unwrap();

    pic.create_canister_with_id(Some(default_identity), None, canister_id).unwrap();
    pic.add_cycles(canister_id, INIT_CYCLES);

    pic.install_canister(canister_id, WASM_MODULE.to_vec(), encode_one(Some(InitArgs::V1(InitArgV1 { maintainers: Some(vec![default_identity]), block_offset: None, host_canister_id: None, max_memory_size_bytes: None }))).unwrap(), Some(default_identity));

    use archive_swap::*;

    let pocketed_canister_id = PocketedCanisterId::new(canister_id, &pic);
    #[allow(unused)] let default = pocketed_canister_id.sender(default_identity);
    #[allow(unused)] let alice = pocketed_canister_id.sender(alice_identity);
    #[allow(unused)] let bob = pocketed_canister_id.sender(bob_identity);
    #[allow(unused)] let carol = pocketed_canister_id.sender(carol_identity);
    #[allow(unused)] let anonymous = pocketed_canister_id.sender(anonymous_identity);

    // 🚩 1 business
    assert_eq!(alice.get_block_pb(vec![].into()).unwrap_err().reject_message, "Only Maintainers are allowed to query data".to_string());
    assert_eq!(default.get_block_pb(vec![].into()).unwrap(), serde_bytes::ByteBuf::new());
    assert_eq!(alice.remaining_capacity().unwrap(), 32_212_254_720);
    assert_eq!(alice.append_blocks(vec![vec![0].into()]).unwrap_err().reject_message, "'Only Swap Canister is allowed to append blocks to an Archive Node'".to_string());
    assert_eq!(default.append_blocks(vec![hex::decode("0a220a2000000000000000000000000000000000000000000000000000000000000000001a360a340a320a300a2c0a1c0a0c0a0a00000000000000020101120c0a0a00000000020001310101120c737761705f76325f302e33251200").unwrap().into()]).unwrap(), ());
    assert_eq!(default.get_block_pb(vec![].into()).unwrap(), hex::decode("0a5e0a5c0a220a2000000000000000000000000000000000000000000000000000000000000000001a360a340a320a300a2c0a1c0a0c0a0a00000000000000020101120c0a0a00000000020001310101120c737761705f76325f302e33251200").unwrap());
    assert_eq!(alice.remaining_capacity().unwrap(), 32_212_254_628);
    assert_eq!(default.iter_blocks_pb(hex::decode("1064").unwrap().into()).unwrap(), hex::decode("0a5e0a5c0a220a2000000000000000000000000000000000000000000000000000000000000000001a360a340a320a300a2c0a1c0a0c0a0a00000000000000020101120c0a0a00000000020001310101120c737761705f76325f302e33251200").unwrap());
    assert_eq!(default.get_blocks_pb(hex::decode("1064").unwrap().into()).unwrap(), hex::decode("126852657175657374656420626c6f636b73206f757473696465207468652072616e67652073746f72656420696e207468652061726368697665206e6f64652e20526571756573746564205b30202e2e203130305d2e20417661696c61626c65205b30202e2e20315d2e").unwrap());
    assert_eq!(default.get_blocks(GetBlocksArgs { start: 1, length: 100 }).unwrap(), GetSwapBlocksResult::Ok(SwapBlockRange { blocks: vec![] }));
    assert_eq!(default.get_blocks(GetBlocksArgs { start: 11, length: 100 }).unwrap(), GetSwapBlocksResult::Ok(SwapBlockRange { blocks: vec![] }));
    assert_eq!(default.get_blocks(GetBlocksArgs { start: 0, length: 100 }).unwrap(), GetSwapBlocksResult::Ok(SwapBlockRange { blocks: vec![SwapBlock  {
        parent_hash: vec![0; 32].into(),
        timestamp: 0,
        transaction: SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::Create(PairCreate {
                pa: TokenPairAmm {
                    pair: TokenPair {
                        token0: Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
                        token1: Principal::from_text("lvfsa-2aaaa-aaaaq-aaeyq-cai").unwrap(),
                    },
                    amm: Amm::SwapV2T3, // swap_v2_0.3%
                },
                creator: Principal::from_text("aaaaa-aa").unwrap(),
            })),
            memo: None,
            created: None,
        },
    }] }));
    assert_eq!(default.http_request(CustomHttpRequest { url: "/metrics".to_string(), method: "GET".to_string(), body: vec![].into(), headers: vec![] }).unwrap(), CustomHttpResponse { body: r##"# HELP archive_node_block_height_offset Block height offset assigned to this instance of the archive canister.
# TYPE archive_node_block_height_offset gauge
archive_node_block_height_offset 0 1620328630000
# HELP archive_node_max_memory_size_bytes Maximum amount of memory this canister is allowed to use for blocks.
# TYPE archive_node_max_memory_size_bytes gauge
archive_node_max_memory_size_bytes 32212254720 1620328630000
# HELP archive_node_blocks Number of blocks stored by this canister.
# TYPE archive_node_blocks gauge
archive_node_blocks 1 1620328630000
# HELP archive_node_blocks_bytes Total amount of memory consumed by the blocks stored by this canister.
# TYPE archive_node_blocks_bytes gauge
archive_node_blocks_bytes 92 1620328630000
# HELP archive_node_stable_memory_pages Size of the stable memory allocated by this canister measured in 64K Wasm pages.
# TYPE archive_node_stable_memory_pages gauge
archive_node_stable_memory_pages 257 1620328630000
# HELP stable_memory_bytes Size of the stable memory allocated by this canister measured in bytes.
# TYPE stable_memory_bytes gauge
stable_memory_bytes 16842752 1620328630000
# HELP heap_memory_bytes Size of the heap memory allocated by this canister measured in bytes.
# TYPE heap_memory_bytes gauge
heap_memory_bytes 1245184 1620328630000
# HELP archive_node_last_upgrade_time_seconds IC timestamp of the last upgrade performed on this canister.
# TYPE archive_node_last_upgrade_time_seconds gauge
archive_node_last_upgrade_time_seconds 0 1620328630000
"##.as_bytes().to_vec().into(), headers: vec![("Content-Type".to_string(), "text/plain".to_string())], upgrade: None, streaming_strategy: None, status_code: 200 });
    assert_eq!(default.get_encoded_blocks(GetBlocksArgs { start: 0, length: 100 }).unwrap(), GetEncodedBlocksResult::Ok(vec![hex::decode("0a220a2000000000000000000000000000000000000000000000000000000000000000001a360a340a320a300a2c0a1c0a0c0a0a00000000000000020101120c0a0a00000000020001310101120c737761705f76325f302e33251200").unwrap().into()]));

    // 🚩 2 business set_maintainers
    assert_eq!(alice.set_maintainers(None).unwrap_err().reject_message, "'Only Swap Canister is allowed to append blocks to an Archive Node'".to_string());
    assert_eq!(alice.get_block_pb(vec![].into()).unwrap_err().reject_message, "Only Maintainers are allowed to query data".to_string());
    assert_eq!(default.set_maintainers(None).unwrap(), ());
    assert_eq!(alice.get_block_pb(vec![].into()).unwrap(), hex::decode("0a5e0a5c0a220a2000000000000000000000000000000000000000000000000000000000000000001a360a340a320a300a2c0a1c0a0c0a0a00000000000000020101120c0a0a00000000020001310101120c737761705f76325f302e33251200").unwrap());
    assert_eq!(default.set_maintainers(Some(vec![default_identity])).unwrap(), ());
    assert_eq!(alice.get_block_pb(vec![].into()).unwrap_err().reject_message, "Only Maintainers are allowed to query data".to_string());

    // 🚩 3 business query
    assert_eq!(alice.query_latest_block_index().unwrap(), Some(0));
    assert_eq!(alice.query_metrics().unwrap(), CustomMetrics { stable_memory_pages: 257, stable_memory_bytes: 16_842_752, heap_memory_bytes: 1_245_184, last_upgrade_time_seconds: 0, max_memory_size_bytes: 32_212_254_720, blocks: 1, blocks_bytes: 92, block_height_offset: 0 });

    // 🚩 4 business set_max_memory_size_bytes
    assert_eq!(default.query_metrics().unwrap(), CustomMetrics { stable_memory_pages: 257, stable_memory_bytes: 16_842_752, heap_memory_bytes: 1_245_184, last_upgrade_time_seconds: 0, max_memory_size_bytes: 32_212_254_720, blocks: 1, blocks_bytes: 92, block_height_offset: 0 });
    assert_eq!(alice.set_max_memory_size_bytes(10).unwrap_err().reject_message, "'Only Swap Canister is allowed to append blocks to an Archive Node'".to_string());
    assert_eq!(default.set_max_memory_size_bytes(10).unwrap_err().reject_message.contains("Cannot set max_memory_size_bytes to 10, because it is lower than total_block_size 92."), true);
    assert_eq!(default.set_max_memory_size_bytes(100).unwrap(), ());
    assert_eq!(default.query_metrics().unwrap(), CustomMetrics { stable_memory_pages: 257, stable_memory_bytes: 16_842_752, heap_memory_bytes: 1_245_184, last_upgrade_time_seconds: 0, max_memory_size_bytes: 100, blocks: 1, blocks_bytes: 92, block_height_offset: 0 });
}
