//! https://github.com/dfinity/pocketic

use candid::{Principal, encode_one};
use pocket_ic::PocketIcBuilder;

mod archive_swap;
mod archive_token;
mod icrc2;
mod swap;

const WASM_MODULE_1_0_1: &[u8] = include_bytes!("../sources/source_opt_1_0_1.wasm");
const WASM_MODULE_1_0_4: &[u8] = include_bytes!("../sources/source_opt_1_0_4.wasm.gz");
const WASM_MODULE_1_0_5: &[u8] = include_bytes!("../sources/source_opt_1_0_5.wasm.gz");
const WASM_MODULE_NEXT: &[u8] = include_bytes!("../sources/source_opt.wasm.gz");

#[ignore]
#[test]
#[rustfmt::skip]
fn test_swap_upgrade() {
    // let pic = PocketIc::new();
    let pic = PocketIcBuilder::new().with_nns_subnet().build();

    let default_identity = Principal::from_text("2ibo7-dia").unwrap();

    let canister_id = Principal::from_text("arfmo-qqaaa-aaaaj-az7ta-cai").unwrap();

    pic.create_canister_with_id(Some(default_identity), None, canister_id).unwrap();
    pic.add_cycles(canister_id, 20_000_000_000_000);
    // ! v1.0.1
    pic.install_canister(canister_id, WASM_MODULE_1_0_1.to_vec(), encode_one(Some(InitArgs::V1(InitArgV1 { maintainers: None, schedule: None, current_archiving_token: None, current_archiving_swap: None }))).unwrap(), Some(default_identity));

    use swap::*;

    let pocketed_canister_id = PocketedCanisterId::new(canister_id, &pic);
    #[allow(unused)] let default = pocketed_canister_id.sender(default_identity);

    // ! v1.0.4
    for _ in 0..6 { pic.tick(); } // 🕰︎
    default.pause_replace(Some("test".to_string())).unwrap();
    let arg: Vec<u8> = encode_one(None::<()>).unwrap();
    assert_eq!(arg, vec![68, 73, 68, 76, 1, 110, 127, 1, 0, 0]); // 4449444c016e7f010000
    pic.upgrade_canister(canister_id, WASM_MODULE_1_0_4.to_vec(), arg, Some(default_identity)).unwrap();
    default.pause_replace(None).unwrap();

    // ! v1.0.5
    for _ in 0..6 { pic.tick(); } // 🕰︎
    default.pause_replace(Some("test".to_string())).unwrap();
    let arg: Vec<u8> = encode_one(None::<()>).unwrap();
    assert_eq!(arg, vec![68, 73, 68, 76, 1, 110, 127, 1, 0, 0]); // 4449444c016e7f010000
    pic.upgrade_canister(canister_id, WASM_MODULE_1_0_5.to_vec(), arg, Some(default_identity)).unwrap();
    default.pause_replace(None).unwrap();

    // ! next
    for _ in 0..6 { pic.tick(); } // 🕰︎
    let arg: Vec<u8> = encode_one(None::<()>).unwrap();
    assert_eq!(arg, vec![68, 73, 68, 76, 1, 110, 127, 1, 0, 0]); // 4449444c016e7f010000
    pic.upgrade_canister(canister_id, WASM_MODULE_NEXT.to_vec(), arg, Some(default_identity)).unwrap();

    // ! next
    for _ in 0..6 { pic.tick(); } // 🕰︎
    let arg: Vec<u8> = encode_one(None::<()>).unwrap();
    assert_eq!(arg, vec![68, 73, 68, 76, 1, 110, 127, 1, 0, 0]); // 4449444c016e7f010000
    pic.upgrade_canister(canister_id, WASM_MODULE_NEXT.to_vec(), arg, Some(default_identity)).unwrap();
}

fn print_backtrace() {
    let backtraces = format!("{}", std::backtrace::Backtrace::force_capture());
    let backtraces = backtraces.split('\n').collect::<Vec<_>>();
    let position = backtraces.iter().position(|b| b.contains("5: ")).unwrap();
    eprintln!("{}", backtraces[position + 1]);
}

#[allow(unused)]
fn assert_swap_block_pair_v2_mint_fee(block: archive_swap::SwapBlock, fee: archive_swap::SwapV2MintFeeToken) {
    print_backtrace();
    if let archive_swap::SwapOperation::Pair(archive_swap::PairOperation::SwapV2(
        archive_swap::SwapV2Operation::MintFee(f),
    )) = block.transaction.operation
    {
        return assert_eq!(f, fee);
    }
    panic!("Expected SwapV2MintFeeToken, got {:?}", block);
}
#[allow(unused)]
fn assert_swap_block_pair_v2_transfer(block: archive_swap::SwapBlock, transfer: archive_swap::SwapV2TransferToken) {
    print_backtrace();
    if let archive_swap::SwapOperation::Pair(archive_swap::PairOperation::SwapV2(
        archive_swap::SwapV2Operation::Transfer(t),
    )) = block.transaction.operation
    {
        return assert_eq!(t, transfer);
    }
    panic!("Expected SwapV2TransferToken, got {:?}", block);
}
