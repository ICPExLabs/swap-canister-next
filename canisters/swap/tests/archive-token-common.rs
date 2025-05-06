//! https://github.com/dfinity/pocketic
use candid::{Principal, encode_one};
use pocket_ic::PocketIc;

mod archive_token;

// 2T cycles
const INIT_CYCLES: u128 = 2_000_000_000_000;

const WASM_MODULE: &[u8] = include_bytes!("../../archive-token/sources/source_opt.wasm");

#[ignore]
#[test]
#[rustfmt::skip]
fn test_archive_token_common_apis() {
    let pic = PocketIc::new();

    let default_identity = Principal::from_text("2ibo7-dia").unwrap();
    let alice_identity = Principal::from_text("uuc56-gyb").unwrap();
    let bob_identity = Principal::from_text("hqgi5-iic").unwrap(); // cspell: disable-line
    let carol_identity = Principal::from_text("jmf34-nyd").unwrap();
    let anonymous_identity = Principal::from_text("2vxsx-fae").unwrap();

    let canister_id = Principal::from_text("ykio2-paaaa-aaaaj-az5ka-cai").unwrap();

    pic.create_canister_with_id(Some(default_identity), None, canister_id).unwrap();
    pic.add_cycles(canister_id, INIT_CYCLES);

    pic.install_canister(canister_id, WASM_MODULE.to_vec(), encode_one(None::<()>).unwrap(), Some(default_identity));

    use archive_token::*;

    let pocketed_canister_id = PocketedCanisterId::new(canister_id, &pic);
    #[allow(unused)] let default = pocketed_canister_id.sender(default_identity);
    #[allow(unused)] let alice = pocketed_canister_id.sender(alice_identity);
    #[allow(unused)] let bob = pocketed_canister_id.sender(bob_identity);
    #[allow(unused)] let carol = pocketed_canister_id.sender(carol_identity);
    #[allow(unused)] let anonymous = pocketed_canister_id.sender(anonymous_identity);

    // 🚩 1 version
    assert_eq!(alice.version().unwrap(), 1_u32, "version");
}
