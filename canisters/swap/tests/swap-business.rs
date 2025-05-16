//! https://github.com/dfinity/pocketic
use std::str::FromStr;

use candid::{Nat, Principal, encode_one};
use ic_cdk::management_canister::CanisterSettings;
use pocket_ic::PocketIc;

mod archive_swap;
mod archive_token;
mod icrc2;
mod swap;

// 2T cycles
const INIT_CYCLES: u128 = 2_000_000_000_000;

const WASM_MODULE: &[u8] = include_bytes!("../sources/source_opt.wasm");
const ICRC2_WASM_MODULE: &[u8] = include_bytes!("../../../ledger/ic-icrc1-ledger.wasm");
const ARCHIVE_TOKEN_WASM_MODULE: &[u8] = include_bytes!("../../archive-token/sources/source_opt.wasm");
const ARCHIVE_SWAP_WASM_MODULE: &[u8] = include_bytes!("../../archive-swap/sources/source_opt.wasm");

#[ignore]
#[test]
#[rustfmt::skip]
fn test_swap_business_apis() {
    let pic = PocketIc::new();

    let default_identity = Principal::from_text("2ibo7-dia").unwrap();
    let alice_identity = Principal::from_text("uuc56-gyb").unwrap();
    let bob_identity = Principal::from_text("hqgi5-iic").unwrap(); // cspell: disable-line
    let carol_identity = Principal::from_text("jmf34-nyd").unwrap();
    let anonymous_identity = Principal::from_text("2vxsx-fae").unwrap();

    let canister_id = Principal::from_text("piwiu-wiaaa-aaaaj-azzka-cai").unwrap();
    let token_icp_canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let token_sns_icx_canister_id = Principal::from_text("lvfsa-2aaaa-aaaaq-aaeyq-cai").unwrap();
    let token_ck_btc_canister_id = Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap();
    let token_ck_eth_canister_id = Principal::from_text("ss2fx-dyaaa-aaaar-qacoq-cai").unwrap();
    let token_ck_usdt_canister_id = Principal::from_text("cngnf-vqaaa-aaaar-qag4q-cai").unwrap();
    let token_sns_chat_canister_id = Principal::from_text("2ouva-viaaa-aaaaq-aaamq-cai").unwrap();
    let archive_token_canister_id = Principal::from_text("ykio2-paaaa-aaaaj-az5ka-cai").unwrap();
    let archive_swap_canister_id = Principal::from_text("hcnys-xiaaa-aaaai-q3w4q-cai").unwrap();

    fn account(owner: Principal) -> Account {
        Account { owner, subaccount: None }
    }
    fn icrc2_account(owner: Principal) -> icrc2::Account {
        icrc2::Account { owner, subaccount: None }
    }
    fn archive_token_account(owner: Principal) -> archive_token::Account {
        archive_token::Account { owner, subaccount: None }
    }
    fn archive_swap_account(owner: Principal) -> archive_swap::Account {
        archive_swap::Account { owner, subaccount: None }
    }
    fn nat(value: u64) -> candid::Nat {
        Nat::from(value)
    }
    fn principal(text: &str) -> Principal {
        Principal::from_text(text).unwrap()
    }

    pic.create_canister_with_id(Some(default_identity), None, canister_id).unwrap();
    pic.add_cycles(canister_id, 20_000_000_000_000);
    pic.install_canister(canister_id, WASM_MODULE.to_vec(), encode_one(None::<()>).unwrap(), Some(default_identity));

    pic.create_canister_with_id(Some(default_identity), Some(CanisterSettings { controllers: Some(vec![default_identity, canister_id]), ..CanisterSettings::default() }), archive_token_canister_id).unwrap();
    pic.add_cycles(archive_token_canister_id, INIT_CYCLES);
    pic.install_canister(archive_token_canister_id, ARCHIVE_TOKEN_WASM_MODULE.to_vec(), encode_one(Some(archive_token::InitArgs::V1(archive_token::InitArgV1{ maintainers: Some(vec![default_identity]), block_offset: None, host_canister_id: Some(canister_id), max_memory_size_bytes: None }))).unwrap(), Some(default_identity));
    #[allow(unused)] let archive_token = archive_token::PocketedCanisterId::new(archive_token_canister_id, &pic);

    pic.create_canister_with_id(Some(default_identity), Some(CanisterSettings { controllers: Some(vec![default_identity, canister_id]), ..CanisterSettings::default() }), archive_swap_canister_id).unwrap();
    pic.add_cycles(archive_swap_canister_id, INIT_CYCLES);
    pic.install_canister(archive_swap_canister_id, ARCHIVE_SWAP_WASM_MODULE.to_vec(), encode_one(Some(archive_swap::InitArgs::V1(archive_swap::InitArgV1{ maintainers: None, block_offset: None, host_canister_id: Some(canister_id), max_memory_size_bytes: None }))).unwrap(), Some(default_identity));
    #[allow(unused)] let archive_swap = archive_swap::PocketedCanisterId::new(archive_swap_canister_id, &pic);

    // ! 0. deploy tokens
    #[allow(unused)] let token_sns_icx = deploy_icrc2(&pic, default_identity, token_sns_icx_canister_id, "snsICX", 8, 100000, vec![(default_identity, 1_000_000_000_000)], alice_identity);
    #[allow(unused)] let token_ck_btc = deploy_icrc2(&pic, default_identity, token_ck_btc_canister_id, "ckBTC", 8, 10, vec![(default_identity, 100_000_000)], alice_identity);
    #[allow(unused)] let token_ck_eth = deploy_icrc2(&pic, default_identity, token_ck_eth_canister_id, "ckETH", 18, 2_000_000_000_000, vec![(default_identity, 10_000_000_000_000_000_000), (alice_identity, 8_000_000_000_000_000_000)], alice_identity);
    #[allow(unused)] let token_ck_usdt = deploy_icrc2(&pic, default_identity, token_ck_usdt_canister_id, "ckUSDT", 6, 10000, vec![(default_identity, 100_000_000_000_000)], alice_identity);
    #[allow(unused)] let token_sns_chat = deploy_icrc2(&pic, default_identity, token_sns_chat_canister_id, "snsCHAT", 8, 100000, vec![(default_identity, 1_000_000_000_000)], alice_identity);

    use swap::*;

    let pocketed_canister_id = PocketedCanisterId::new(canister_id, &pic);
    #[allow(unused)] let default = pocketed_canister_id.sender(default_identity);
    #[allow(unused)] let alice = pocketed_canister_id.sender(alice_identity);
    #[allow(unused)] let bob = pocketed_canister_id.sender(bob_identity);
    #[allow(unused)] let carol = pocketed_canister_id.sender(carol_identity);
    #[allow(unused)] let anonymous = pocketed_canister_id.sender(anonymous_identity);

    default.test_config_token_current_archiving_replace(CurrentArchiving { canister_id: archive_token_canister_id, length: 0, max_length: 100_000_000, block_height_offset: 0 }).unwrap();
    default.test_config_swap_current_archiving_replace(CurrentArchiving { canister_id: archive_swap_canister_id, length: 0, max_length: 100_000_000, block_height_offset: 0 }).unwrap();

    // 🚩 0 query balances
    assert_eq!(token_sns_icx.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(1_000_000_000_000));
    assert_eq!(token_sns_icx.sender(alice_identity).icrc1_balance_of(icrc2_account(alice_identity)).unwrap(), nat(0));
    assert_eq!(token_sns_icx.sender(bob_identity).icrc1_balance_of(icrc2_account(bob_identity)).unwrap(), nat(0));
    assert_eq!(token_ck_btc.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(100_000_000));
    assert_eq!(token_ck_btc.sender(alice_identity).icrc1_balance_of(icrc2_account(alice_identity)).unwrap(), nat(0));
    assert_eq!(token_ck_btc.sender(bob_identity).icrc1_balance_of(icrc2_account(bob_identity)).unwrap(), nat(0));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(10_000_000_000_000_000_000));
    assert_eq!(token_ck_eth.sender(alice_identity).icrc1_balance_of(icrc2_account(alice_identity)).unwrap(), nat(8_000_000_000_000_000_000));
    assert_eq!(token_ck_eth.sender(bob_identity).icrc1_balance_of(icrc2_account(bob_identity)).unwrap(), nat(0));
    assert_eq!(token_ck_usdt.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(100_000_000_000_000));
    assert_eq!(token_ck_usdt.sender(alice_identity).icrc1_balance_of(icrc2_account(alice_identity)).unwrap(), nat(0));
    assert_eq!(token_ck_usdt.sender(bob_identity).icrc1_balance_of(icrc2_account(bob_identity)).unwrap(), nat(0));
    assert_eq!(token_sns_chat.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(1_000_000_000_000));
    assert_eq!(token_sns_chat.sender(alice_identity).icrc1_balance_of(icrc2_account(alice_identity)).unwrap(), nat(0));
    assert_eq!(token_sns_chat.sender(bob_identity).icrc1_balance_of(icrc2_account(bob_identity)).unwrap(), nat(0));

    assert_eq!(alice.permission_query().unwrap(), ["PauseQuery", "PermissionQuery", "BusinessTokenDeposit", "BusinessTokenWithdraw", "BusinessTokenTransfer", "BusinessTokenPairLiquidityAdd", "BusinessTokenPairLiquidityRemove", "BusinessTokenPairSwap"].iter().map(|p| p.to_string()).collect::<Vec<_>>());
    assert_eq!(default.permission_query().unwrap(), ["PauseQuery", "PauseReplace", "PermissionQuery", "PermissionFind", "PermissionUpdate", "ScheduleFind", "ScheduleReplace", "ScheduleTrigger", "BusinessConfigFeeTo", "BusinessConfigCustomToken", "BusinessConfigMaintaining", "BusinessTokenBalanceBy", "BusinessTokenDeposit", "BusinessTokenWithdraw", "BusinessTokenTransfer", "BusinessTokenPairCreateOrRemove", "BusinessTokenPairLiquidityAdd", "BusinessTokenPairLiquidityRemove", "BusinessTokenPairSwap"].iter().map(|p| p.to_string()).collect::<Vec<_>>());

    // 🚩 0 business config fee to update
    assert_eq!(bob.config_fee_to_query().unwrap_err().reject_message, "Permission 'BusinessConfigFeeTo' is required".to_string());
    assert_eq!(default.config_fee_to_query().unwrap(), FeeTo { token_fee_to: None, swap_fee_to: None });
    assert_eq!(bob.config_fee_to_replace(FeeTo { token_fee_to: Some(account(alice_identity)), swap_fee_to: Some(account(bob_identity)) }).unwrap_err().reject_message, "Permission 'BusinessConfigFeeTo' is required".to_string());
    assert_eq!(default.config_fee_to_replace(FeeTo { token_fee_to: Some(account(alice_identity)), swap_fee_to: Some(account(bob_identity)) }).unwrap(), FeeTo { token_fee_to: None, swap_fee_to: None });
    assert_eq!(default.config_fee_to_query().unwrap(), FeeTo { token_fee_to: Some(account(alice_identity)), swap_fee_to: Some(account(bob_identity)) });
    assert_eq!(default.config_fee_to_replace(FeeTo { token_fee_to: None, swap_fee_to: None }).unwrap(), FeeTo { token_fee_to: Some(account(alice_identity)), swap_fee_to: Some(account(bob_identity)) });
    assert_eq!(default.config_fee_to_query().unwrap(), FeeTo { token_fee_to: None, swap_fee_to: None });

    // 🚩 1 business tokens
    let symbols = alice.tokens_query().unwrap().iter().map(|t| t.symbol.clone()).collect::<Vec<_>>();
    assert_eq!(symbols.contains(&"ICP".to_string()), true);
    assert_eq!(symbols.contains(&"ckUSDT".to_string()), true);
    assert_eq!(alice.token_query(token_icp_canister_id).unwrap().unwrap().name, "Internet Computer".to_string());
    assert_eq!(alice.token_balance_of(token_sns_icx_canister_id, account(default_identity)).unwrap_err().reject_message.contains("You can only query your own balance"), true);
    assert_eq!(default.token_balance_by(token_sns_icx_canister_id, account(alice_identity)).unwrap(), nat(0));
    assert_eq!(default.token_balance_of(token_sns_icx_canister_id, account(default_identity)).unwrap(), nat(0));
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_sns_icx_canister_id, nat(0))]);

    // 🚩 1.1 business tokens deposit
    assert_eq!(default.token_deposit(TokenDepositArgs { token: token_ck_eth_canister_id, from: account(default_identity), deposit_amount_without_fee: nat(5_000_000_000_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, Some(100)).unwrap_err().reject_message.contains("Too many retries"), true);
    assert_eq!(default.token_deposit(TokenDepositArgs { token: token_ck_eth_canister_id, from: account(default_identity), deposit_amount_without_fee: nat(5_000_000_000_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Err(BusinessError::TransferFromError(TransferFromError::InsufficientAllowance { allowance: nat(0) })));
    assert_eq!(token_ck_eth.sender(default_identity).icrc2_approve(icrc2::ApproveArgs::new(icrc2_account(canister_id), nat(1_000_000_000_000_000_000))).unwrap(), icrc2::Result2::Ok(nat(2)));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(9_999_998_000_000_000_000));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(alice_identity)).unwrap(), nat(8_000_000_000_000_000_000));
    assert_eq!(default.token_deposit(TokenDepositArgs { token: token_ck_eth_canister_id, from: account(default_identity), deposit_amount_without_fee: nat(5_000_000_000_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Err(BusinessError::TransferFromError(TransferFromError::InsufficientAllowance { allowance: nat(1_000_000_000_000_000_000) })));
    assert_eq!(token_ck_eth.sender(default_identity).icrc2_approve(icrc2::ApproveArgs::new(icrc2_account(canister_id), nat(10_000_000_000_000_000_000))).unwrap(), icrc2::Result2::Ok(nat(3)));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(9_999_996_000_000_000_000));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(alice_identity)).unwrap(), nat(8_000_000_000_000_000_000));
    assert_eq!(alice.request_trace_index_get().unwrap_err().reject_message, "Permission 'BusinessConfigMaintaining' is required".to_string());
    assert_eq!(default.request_trace_index_get().unwrap(), (0, 0));
    assert_eq!(default.request_trace_get(0).unwrap(), None); // 👁︎
    assert_eq!(alice.block_token_get(1).unwrap_err().reject_message, "Only Maintainers are allowed to query data".to_string()); // 👁︎
    assert_eq!(default.block_token_get(1).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.block_token_get(0).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(0).unwrap(), None); // 👁︎
    assert_eq!(default.token_deposit(TokenDepositArgs { token: token_ck_eth_canister_id, from: account(default_identity), deposit_amount_without_fee: nat(5_000_000_000_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(4)));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    assert_token_block_deposit(archive_token.sender(default_identity).get_block(0).unwrap().unwrap(), archive_token::DepositToken { token: token_ck_eth_canister_id, from: archive_token_account(default_identity), amount: nat(5_000_000_000_000_000_000), to: archive_token_account(default_identity) }); // 👁︎
    assert_eq!(default.block_token_get(0).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.request_trace_get(0).unwrap().unwrap().traces[0].1, format!("*Deposit* `token:[{}], from:({}.), to:({}.), amount:5_000_000_000_000_000_000, height:4`", token_ck_eth_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎

    // 🚩 1.2 business tokens balance of
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(4_999_994_000_000_000_000));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(alice_identity)).unwrap(), nat(8_000_002_000_000_000_000));
    assert_eq!(default.token_balance_of(token_ck_eth_canister_id, account(default_identity)).unwrap(), nat(5_000_000_000_000_000_000));
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(5_000_000_000_000_000_000))]);

    // 🚩 1.3 business tokens withdraw
    assert_eq!(default.token_withdraw(TokenWithdrawArgs { token: token_ck_eth_canister_id, from: account(default_identity), withdraw_amount_without_fee: nat(15_000_000_000_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Err(BusinessError::InsufficientBalance { token: token_ck_eth_canister_id, balance: nat(5_000_000_000_000_000_000) }));
    assert_eq!(default.request_trace_get(1).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(1).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(1).unwrap(), None); // 👁︎
    assert_eq!(default.token_withdraw(TokenWithdrawArgs { token: token_ck_eth_canister_id, from: account(default_identity), withdraw_amount_without_fee: nat(999_998_000_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(5)));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    assert_token_block_withdraw(archive_token.sender(default_identity).get_block(1).unwrap().unwrap(), archive_token::DepositToken { token: token_ck_eth_canister_id, from: archive_token_account(default_identity), amount: nat(1_000_000_000_000_000_000), to: archive_token_account(default_identity) }); // 👁︎
    assert_eq!(default.block_token_get(1).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.request_trace_get(1).unwrap().unwrap().traces[0].1, format!("*Withdraw* `token:[{}], from:({}.), to:({}.), amount:1_000_000_000_000_000_000, height:5`", token_ck_eth_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(4_000_000_000_000_000_000))]);

    // 🚩 1.4 business tokens balance of
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(5_999_992_000_000_000_000));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(alice_identity)).unwrap(), nat(8_000_004_000_000_000_000));
    assert_eq!(default.token_balance_of(token_ck_eth_canister_id, account(default_identity)).unwrap(), nat(4_000_000_000_000_000_000));
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(4_000_000_000_000_000_000))]);

    // 🚩 1.5 business tokens transfer
    assert_eq!(alice.token_balance_of(token_ck_eth_canister_id, account(alice_identity)).unwrap(), nat(0));
    assert_eq!(default.token_transfer(TokenTransferArgs { token: token_ck_eth_canister_id, from: account(default_identity), transfer_amount_without_fee: Nat::from_str("1_000_000_000_000_000_000_000").unwrap(), to: account(alice_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Err(BusinessError::InsufficientBalance { token: token_ck_eth_canister_id, balance: nat(4_000_000_000_000_000_000) }));
    assert_eq!(default.request_trace_get(2).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(2).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(2).unwrap(), None); // 👁︎
    assert_eq!(default.token_transfer(TokenTransferArgs { token: token_ck_eth_canister_id, from: account(default_identity), transfer_amount_without_fee: nat(1_000_000_000_000_000_000), to: account(alice_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(1_000_000_000_000_000_000)));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(2).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_eth_canister_id, from: archive_token_account(default_identity), amount: nat(1_000_000_000_000_000_000), to: archive_token_account(alice_identity), fee: None }); // 👁︎
    assert_eq!(default.block_token_get(2).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.request_trace_get(2).unwrap().unwrap().traces[0].1, format!("*Transfer* `token:[{}], from:({}.), to:({}.), amount:1_000_000_000_000_000_000, fee:None`", token_ck_eth_canister_id.to_text(), default_identity.to_text(), alice_identity.to_text())); // 👁︎
    assert_eq!(default.token_balance_of(token_ck_eth_canister_id, account(default_identity)).unwrap(), nat(3_000_000_000_000_000_000));
    assert_eq!(alice.token_balance_of(token_ck_eth_canister_id, account(alice_identity)).unwrap(), nat(1_000_000_000_000_000_000));

    // 🚩 2 business pair fee_to
    // assert_eq!(default.config_fee_to_replace(FeeTo { token_fee_to: Some(account(alice_identity)), swap_fee_to: Some(account(bob_identity)) }).unwrap(), FeeTo { token_fee_to: None, swap_fee_to: None });

    // 🚩 2.1 business pairs
    let token_ck_eth_token_ck_usdt_dummy_canister_id = principal("vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4");
    let token_ck_eth_token_ck_usdt_subaccount = hex::decode("11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c").unwrap();
    assert_eq!(alice.pairs_query().unwrap(), Vec::new());
    assert_eq!(alice.pair_query(TokenPairPool { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_0.3%".to_string() }).unwrap(), None);
    assert_eq!(alice.pair_create(TokenPairCreateOrRemoveArgs { pool: TokenPairPool { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_0.3%".to_string() }, memo: None, created: None }).unwrap_err().reject_message, "Permission 'BusinessTokenPairCreateOrRemove' is required".to_string());
    assert_eq!(default.tokens_balance_of(account(default_identity)).unwrap().contains(&(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))), false);
    assert_eq!(alice.block_swap_get(1).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.request_trace_get(3).unwrap(), None); // 👁︎
    assert_eq!(default.block_swap_get(0).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(0).unwrap(), None); // 👁︎
    assert_eq!(default.pair_create(TokenPairCreateOrRemoveArgs { pool: TokenPairPool { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_0.3%".to_string() }, memo: None, created: None }).unwrap(), TokenPairCreateOrRemoveResult::Ok(MarketMakerView::SwapV2(SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "0".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "0".to_string(), reserve1: "0".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() })));
    // assert_eq!(default.config_protocol_fee_replace(token_ck_eth_token_ck_usdt_subaccount.clone().into(), Some(SwapRatio { numerator: 1, denominator: 6 })).unwrap(), None);
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_eq!(default.block_swap_get(0).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_swap_block_pair_create(archive_swap.sender(default_identity).get_block(0).unwrap().unwrap(), archive_swap::PairCreate { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id  }, amm: archive_swap::Amm::SwapV2T3 }, creator: default_identity  }); // 👁︎
    assert_eq!(default.block_swap_get(0).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.request_trace_get(3).unwrap().unwrap().traces[0].1, format!("*TokenPairCreate* `token0:[{}], token1:[{}], amm:swap_v2_0.3%, subaccount:({}), dummyCanisterId:[{}]`", token_ck_eth_canister_id.to_text(), token_ck_usdt_canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount), token_ck_eth_token_ck_usdt_dummy_canister_id.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(4).unwrap(), None); // 👁︎
    assert_eq!(alice.pairs_query().unwrap()[0], (TokenPairPool { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_0.3%".to_string() }, MarketMakerView::SwapV2(SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "0".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "0".to_string(), reserve1: "0".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() })));
    assert_eq!(alice.pair_query(TokenPairPool { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_0.3%".to_string() }).unwrap().unwrap(), MarketMakerView::SwapV2(SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "0".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "0".to_string(), reserve1: "0".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() }));
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(3_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(0)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_tokens_balance(alice.tokens_balance_of(Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_ck_eth_canister_id, nat(0)), (token_ck_usdt_canister_id, nat(0)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);

    // 🚩 2.2 business pair liquidity add
    assert_eq!(alice.pair_liquidity_add(TokenPairLiquidityAddArgs { swap_pair: SwapTokenPair { token: (token_ck_eth_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_0.3%".to_string() }, from: account(default_identity), to: account(default_identity), amount_desired: (nat(1), nat(1)), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityAddResult::Err(BusinessError::NotOwner(default_identity)));
    assert_eq!(default.pair_liquidity_add(TokenPairLiquidityAddArgs { swap_pair: SwapTokenPair { token: (token_ck_eth_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_0.3%".to_string() }, from: account(default_identity), to: account(default_identity), amount_desired: (nat(2_000_000_000_000_000_000), nat(200_000_000_000)), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityAddResult::Err(BusinessError::InsufficientBalance{ token: token_ck_usdt_canister_id, balance: nat(0) }));
    assert_eq!(token_ck_usdt.sender(default_identity).icrc2_approve(icrc2::ApproveArgs::new(icrc2_account(canister_id), nat(900_000_000_000))).unwrap(), icrc2::Result2::Ok(nat(1)));
    assert_eq!(default.token_deposit(TokenDepositArgs { token: token_ck_usdt_canister_id, from: account(default_identity), deposit_amount_without_fee: nat(800_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(2)));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    assert_token_block_deposit(archive_token.sender(default_identity).get_block(3).unwrap().unwrap(), archive_token::DepositToken { token: token_ck_usdt_canister_id, from: archive_token_account(default_identity), amount: nat(800_000_000_000), to: archive_token_account(default_identity) }); // 👁︎
    assert_eq!(default.block_token_get(3).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.request_trace_get(4).unwrap().unwrap().traces[0].1, format!("*Deposit* `token:[{}], from:({}.), to:({}.), amount:800_000_000_000, height:2`", token_ck_usdt_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(5).unwrap(), None); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(4).unwrap(), None); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(5).unwrap(), None); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(6).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(4).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.block_token_get(5).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.block_token_get(6).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(1).unwrap(), None); // 👁︎
    assert_eq!(default.block_swap_get(1).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.pair_liquidity_add(TokenPairLiquidityAddArgs { swap_pair: SwapTokenPair { token: (token_ck_eth_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_0.3%".to_string() }, from: account(default_identity), to: account(default_identity), amount_desired: (nat(2_000_000_000_000_000_000), nat(400_000_000_000)), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityAddResult::Ok(TokenPairLiquidityAddSuccess { liquidity: nat(894_427_190_999_915), amount: (nat(2_000_000_000_000_000_000), nat(400_000_000_000)) }));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_eq!(default.block_swap_get(1).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(2).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(3).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_swap_block_pair_v2_mint(archive_swap.sender(default_identity).get_block(1).unwrap().unwrap(), archive_swap::SwapV2MintToken { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2T3 }, from: archive_swap_account(default_identity), to: archive_swap_account(default_identity), token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amount0: nat(2_000_000_000_000_000_000), amount1: nat(400_000_000_000), token: token_ck_eth_token_ck_usdt_dummy_canister_id, amount: nat(894_427_190_999_915) }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(2).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2T3 }, block_timestamp: 0, supply: nat(894_427_190_999_915), reserve0: nat(2_000_000_000_000_000_000), reserve1: nat(400_000_000_000), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(4).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_eth_canister_id, from: archive_token_account(default_identity), amount: nat(2_000_000_000_000_000_000), to: archive_token::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(5).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_usdt_canister_id, from: archive_token_account(default_identity), amount: nat(400_000_000_000), to: archive_token::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_deposit(archive_token.sender(default_identity).get_block(6).unwrap().unwrap(), archive_token::DepositToken { token: token_ck_eth_token_ck_usdt_dummy_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }, amount: nat(894_427_190_999_915), to: archive_token_account(default_identity) }); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(7).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(4).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(5).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(6).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(7).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.request_trace_get(5).unwrap().unwrap().traces[1].1, format!("*TokenTransfer* `token:[{}], from:({}.), to:({}.{}), amount:2_000_000_000_000_000_000, done:2_000_000_000_000_000_000`", token_ck_eth_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(5).unwrap().unwrap().traces[2].1, format!("*TokenTransfer* `token:[{}], from:({}.), to:({}.{}), amount:400_000_000_000, done:400_000_000_000`", token_ck_usdt_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(5).unwrap().unwrap().traces[3].1, format!("*PairLiquidityAdd* `amount_a:2_000_000_000_000_000_000, amount_b:400_000_000_000`{}", "")); // 👁︎
    assert_eq!(default.request_trace_get(5).unwrap().unwrap().traces[4].1, format!("*PairLiquidityMint(Deposit)* `token:[{}], from[transferred 2 tokens]:({}.{}), to[minted liquidity]:({}.), amount:894_427_190_999_915`", token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(5).unwrap().unwrap().traces[5].1, format!("*PairLiquidityMint* `token:[{}], to:({}.), amount:894_427_190_999_915`", token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(6).unwrap(), None); // 👁︎
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[0].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "894_427_190_999_915".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "2_000_000_000_000_000_000".to_string(), reserve1: "400_000_000_000".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_swap_v2_market_maker(&alice.pair_query(TokenPairPool { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_0.3%".to_string() }).unwrap().unwrap(), SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "894_427_190_999_915".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "2_000_000_000_000_000_000".to_string(), reserve1: "400_000_000_000".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(1_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(400_000_000_000)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(894_427_190_999_915))]);
    assert_tokens_balance(default.tokens_balance_of(swap::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_ck_eth_canister_id, nat(2_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(400_000_000_000)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);

    // 🚩 2.3 business pair swap extra tokens for tokens
    let token_sns_icx_token_ck_usdt_dummy_canister_id = principal("cqghx-dyrhn-uthyh-esofu-7ydwb-butty-mn7ac-4hxww-5d7gh-ubdbs-f7o");
    let token_sns_icx_token_ck_usdt_subaccount = hex::decode("113b6933e0e4938b4fe076086939e18df805c3ded6e8fe63d0230c8bf7f03cf4").unwrap();
    assert_eq!(default.pair_swap_exact_tokens_for_tokens(TokenPairSwapExactTokensForTokensArgs { from: account(default_identity), amount_in: nat(100_000_000), amount_out_min: nat(100_000_000_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Err(BusinessError::TokenPairAmmNotExist(TokenPairAmm { pair: TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id }, amm: Amm::SwapV2H1 })));
    assert_eq!(default.pair_create(TokenPairCreateOrRemoveArgs { pool: TokenPairPool { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_1%".to_string() }, memo: None, created: None }).unwrap(), TokenPairCreateOrRemoveResult::Ok(MarketMakerView::SwapV2(SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000".to_string(), decimals: 7, dummy_canister_id: token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000".to_string(), total_supply: "0".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "0".to_string(), reserve1: "0".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "1/100".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() })));
    // assert_eq!(default.config_protocol_fee_replace(token_sns_icx_token_ck_usdt_subaccount.clone().into(), Some(SwapRatio { numerator: 1, denominator: 6 })).unwrap(), None);
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_eq!(default.block_swap_get(3).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_swap_block_pair_create(archive_swap.sender(default_identity).get_block(3).unwrap().unwrap(), archive_swap::PairCreate { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id  }, amm: archive_swap::Amm::SwapV2H1 }, creator: default_identity  }); // 👁︎
    assert_eq!(default.block_swap_get(3).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.request_trace_get(6).unwrap().unwrap().traces[0].1, format!("*TokenPairCreate* `token0:[{}], token1:[{}], amm:swap_v2_1%, subaccount:({}), dummyCanisterId:[{}]`", token_sns_icx_canister_id.to_text(), token_ck_usdt_canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount), token_sns_icx_token_ck_usdt_dummy_canister_id.to_text())); // 👁︎
    assert_swap_v2_market_maker(&alice.pair_query(TokenPairPool { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_1%".to_string() }).unwrap().unwrap(), SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000".to_string(), decimals: 7, dummy_canister_id: token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000".to_string(), total_supply: "0".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "0".to_string(), reserve1: "0".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "1/100".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_eq!(default.pair_swap_exact_tokens_for_tokens(TokenPairSwapExactTokensForTokensArgs { from: account(default_identity), amount_in: nat(100_000_000), amount_out_min: nat(100_000_000_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Err(BusinessError::InsufficientBalance { token: token_sns_icx_canister_id, balance: nat(0) }));
    assert_eq!(token_sns_icx.sender(default_identity).icrc2_approve(icrc2::ApproveArgs::new(icrc2_account(canister_id), nat(100_000_000_000))).unwrap(), icrc2::Result2::Ok(nat(1)));
    assert_eq!(default.token_deposit(TokenDepositArgs { token: token_sns_icx_canister_id, from: account(default_identity), deposit_amount_without_fee: nat(20_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(2)));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    assert_token_block_deposit(archive_token.sender(default_identity).get_block(7).unwrap().unwrap(), archive_token::DepositToken { token: token_sns_icx_canister_id, from: archive_token_account(default_identity), amount: nat(20_000_000_000), to: archive_token_account(default_identity) }); // 👁︎
    assert_eq!(default.block_token_get(7).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.request_trace_get(7).unwrap().unwrap().traces[0].1, format!("*Deposit* `token:[{}], from:({}.), to:({}.), amount:20_000_000_000, height:2`", token_sns_icx_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(8).unwrap(), None); // 👁︎
    assert_eq!(default.pair_swap_exact_tokens_for_tokens(TokenPairSwapExactTokensForTokensArgs { from: account(default_identity), amount_in: nat(100_000_000), amount_out_min: nat(100_000_000_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Err(BusinessError::Swap("INSUFFICIENT_LIQUIDITY".to_string())));
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_sns_icx_canister_id, nat(20_000_000_000)), (token_ck_usdt_canister_id, nat(4_000_00_000_000)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_tokens_balance(default.tokens_balance_of(swap::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_sns_icx_canister_id, nat(0)), (token_ck_usdt_canister_id, nat(0)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_eq!(default.pair_liquidity_add(TokenPairLiquidityAddArgs { swap_pair: SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }, from: account(default_identity), to: account(default_identity), amount_desired: (nat(10_000_000_000), nat(200_000_000_000)), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityAddResult::Ok(TokenPairLiquidityAddSuccess { liquidity: nat(44_721_359_549), amount: (nat(10_000_000_000), nat(200_000_000_000)) }));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_eq!(default.block_swap_get(4).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(5).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(6).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_swap_block_pair_v2_mint(archive_swap.sender(default_identity).get_block(4).unwrap().unwrap(), archive_swap::SwapV2MintToken { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2H1 }, from: archive_swap_account(default_identity), to: archive_swap_account(default_identity), token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id, amount0: nat(10_000_000_000), amount1: nat(200_000_000_000), token: token_sns_icx_token_ck_usdt_dummy_canister_id, amount: nat(44_721_359_549) }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(5).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2H1 }, block_timestamp: 0, supply: nat(44_721_359_549), reserve0: nat(10_000_000_000), reserve1: nat(200_000_000_000), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(8).unwrap().unwrap(), archive_token::TransferToken { token: token_sns_icx_canister_id, from: archive_token_account(default_identity), amount: nat(10_000_000_000), to: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(9).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_usdt_canister_id, from: archive_token_account(default_identity), amount: nat(200_000_000_000), to: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_deposit(archive_token.sender(default_identity).get_block(10).unwrap().unwrap(), archive_token::DepositToken { token: token_sns_icx_token_ck_usdt_dummy_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, amount: nat(44_721_359_549), to: archive_token_account(default_identity) }); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(11).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(8).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(9).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(10).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(11).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.request_trace_get(8).unwrap().unwrap().traces[1].1, format!("*TokenTransfer* `token:[{}], from:({}.), to:({}.{}), amount:10_000_000_000, done:10_000_000_000`", token_sns_icx_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(8).unwrap().unwrap().traces[2].1, format!("*TokenTransfer* `token:[{}], from:({}.), to:({}.{}), amount:200_000_000_000, done:200_000_000_000`", token_ck_usdt_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(8).unwrap().unwrap().traces[3].1, format!("*PairLiquidityAdd* `amount_a:10_000_000_000, amount_b:200_000_000_000`{}", "")); // 👁︎
    assert_eq!(default.request_trace_get(8).unwrap().unwrap().traces[4].1, format!("*PairLiquidityMint(Deposit)* `token:[{}], from[transferred 2 tokens]:({}.{}), to[minted liquidity]:({}.), amount:44_721_359_549`", token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(8).unwrap().unwrap().traces[5].1, format!("*PairLiquidityMint* `token:[{}], to:({}.), amount:44_721_359_549`", token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(9).unwrap(), None); // 👁︎
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[0].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000".to_string(), decimals: 7, dummy_canister_id: token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000".to_string(), total_supply: "44_721_359_549".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "10_000_000_000".to_string(), reserve1: "200_000_000_000".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "1/100".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_swap_v2_market_maker(&alice.pair_query(TokenPairPool { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_1%".to_string() }).unwrap().unwrap(), SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000".to_string(), decimals: 7, dummy_canister_id: token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000".to_string(), total_supply: "44_721_359_549".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "10_000_000_000".to_string(), reserve1: "200_000_000_000".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "1/100".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_sns_icx_canister_id, nat(10_000_000_000)), (token_ck_usdt_canister_id, nat(200_000_000_000)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(44_721_359_549))]);
    assert_tokens_balance(default.tokens_balance_of(swap::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_sns_icx_canister_id, nat(10_000_000_000)), (token_ck_usdt_canister_id, nat(200_000_000_000)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_eq!(default.pair_swap_exact_tokens_for_tokens(TokenPairSwapExactTokensForTokensArgs { from: account(default_identity), amount_in: nat(100_000_000), amount_out_min: nat(100_000_000_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Err(BusinessError::Swap("INSUFFICIENT_OUTPUT_AMOUNT: 1_960_590_157".to_string())));
    assert_eq!(default.request_trace_get(9).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(11).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.block_swap_get(6).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.pair_swap_exact_tokens_for_tokens(TokenPairSwapExactTokensForTokensArgs { from: account(default_identity), amount_in: nat(100_000_000), amount_out_min: nat(1_000_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Ok(TokenPairSwapTokensSuccess { amounts: vec![nat(100_000_000), nat(1_960_590_157)] }));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_swap_block_pair_swap(archive_swap.sender(default_identity).get_block(6).unwrap().unwrap(), archive_swap::PairSwapToken { token_a: token_sns_icx_canister_id, token_b: token_ck_usdt_canister_id, amm: archive_swap::Amm::SwapV2H1, from: archive_swap_account(default_identity), to: archive_swap_account(default_identity), amount_a: nat(100_000_000), amount_b: nat(1_960_590_157) }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(7).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2H1 }, block_timestamp: 0, supply: nat(44_721_359_549), reserve0: nat(10_100_000_000), reserve1: nat(198_039_409_843), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(8).unwrap(), None); // 👁︎
    assert_eq!(default.block_swap_get(6).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(7).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(8).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(11).unwrap().unwrap(), archive_token::TransferToken { token: token_sns_icx_canister_id, from: archive_token_account(default_identity), amount: nat(100_000_000), to: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(12).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_usdt_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, amount: nat(1_960_590_157), to: archive_token_account(default_identity), fee: None }); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(13).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(11).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(12).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(13).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.request_trace_get(9).unwrap().unwrap().traces[0].1, format!("*TokenTransfer* `token:[{}], from:({}.), to:({}.{}), amount:100_000_000, done:100_000_000`", token_sns_icx_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(9).unwrap().unwrap().traces[1].1, format!("*TokenTransfer* `token:[{}], from:({}.{}), to:({}.), amount:1_960_590_157, done:1_960_590_157`", token_ck_usdt_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(9).unwrap().unwrap().traces[2].1, format!("*TokenPairSwap* `swap_pair:([{}],[{}],swap_v2_1%), from:({}.), to:({}.), pay_amount:100_000_000, got_amount:1_960_590_157`", token_sns_icx_canister_id.to_text(), token_ck_usdt_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(10).unwrap(), None); // 👁︎
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_sns_icx_canister_id, nat(9_900_000_000)), (token_ck_usdt_canister_id, nat(201_960_590_157)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(44_721_359_549))]);
    assert_tokens_balance(default.tokens_balance_of(swap::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_sns_icx_canister_id, nat(10_100_000_000)), (token_ck_usdt_canister_id, nat(198_039_409_843)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_swap_v2_market_maker(&alice.pair_query(TokenPairPool { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_1%".to_string() }).unwrap().unwrap(), SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000".to_string(), decimals: 7, dummy_canister_id: token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000".to_string(), total_supply: "44_721_359_549".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "10_100_000_000".to_string(), reserve1: "198_039_409_843".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "1/100".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });

    // 🚩 2.4 business pair swap tokens for extra tokens
    assert_eq!(default.pair_swap_tokens_for_exact_tokens(TokenPairSwapTokensForExactTokensArgs { from: account(default_identity), amount_out: nat(39_409_843), amount_in_max: nat(1_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Err(BusinessError::Swap("EXCESSIVE_INPUT_AMOUNT: 2_030_607".to_string())));
    assert_eq!(default.pair_swap_tokens_for_exact_tokens(TokenPairSwapTokensForExactTokensArgs { from: account(default_identity), amount_out: nat(39_409_843), amount_in_max: nat(2_030_606), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Err(BusinessError::Swap("EXCESSIVE_INPUT_AMOUNT: 2_030_607".to_string())));
    assert_eq!(default.request_trace_get(10).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(16).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.block_swap_get(10).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.pair_swap_tokens_for_exact_tokens(TokenPairSwapTokensForExactTokensArgs { from: account(default_identity), amount_out: nat(39_409_843), amount_in_max: nat(2_030_607), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Ok(TokenPairSwapTokensSuccess { amounts: vec![nat(2_030_607), nat(39_409_843)] }));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_swap_block_pair_swap(archive_swap.sender(default_identity).get_block(8).unwrap().unwrap(), archive_swap::PairSwapToken { token_a: token_sns_icx_canister_id, token_b: token_ck_usdt_canister_id, amm: archive_swap::Amm::SwapV2H1, from: archive_swap_account(default_identity), to: archive_swap_account(default_identity), amount_a: nat(2_030_607), amount_b: nat(39_409_843) }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(9).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2H1 }, block_timestamp: 0, supply: nat(44_721_359_549), reserve0: nat(10_102_030_607), reserve1: nat(198_000_000_000), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(10).unwrap(), None); // 👁︎
    assert_eq!(default.block_swap_get(8).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(9).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(10).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(13).unwrap().unwrap(), archive_token::TransferToken { token: token_sns_icx_canister_id, from: archive_token_account(default_identity), amount: nat(2_030_607), to: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(14).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_usdt_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, amount: nat(39_409_843), to: archive_token_account(default_identity), fee: None }); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(15).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(13).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(14).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(15).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.request_trace_get(10).unwrap().unwrap().traces[0].1, format!("*TokenTransfer* `token:[{}], from:({}.), to:({}.{}), amount:2_030_607, done:2_030_607`", token_sns_icx_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(10).unwrap().unwrap().traces[1].1, format!("*TokenTransfer* `token:[{}], from:({}.{}), to:({}.), amount:39_409_843, done:39_409_843`", token_ck_usdt_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(10).unwrap().unwrap().traces[2].1, format!("*TokenPairSwap* `swap_pair:([{}],[{}],swap_v2_1%), from:({}.), to:({}.), pay_amount:2_030_607, got_amount:39_409_843`", token_sns_icx_canister_id.to_text(), token_ck_usdt_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(11).unwrap(), None); // 👁︎
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_sns_icx_canister_id, nat(9_897_969_393)), (token_ck_usdt_canister_id, nat(202_000_000_000)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(44_721_359_549))]);
    assert_tokens_balance(default.tokens_balance_of(swap::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_sns_icx_canister_id, nat(10_102_030_607)), (token_ck_usdt_canister_id, nat(198_000_000_000)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_swap_v2_market_maker(&alice.pair_query(TokenPairPool { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_1%".to_string() }).unwrap().unwrap(), SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000".to_string(), decimals: 7, dummy_canister_id: token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000".to_string(), total_supply: "44_721_359_549".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "10_102_030_607".to_string(), reserve1: "198_000_000_000".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "1/100".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });

    // 🚩 2.5 business pair swap by loan
    let token_sns_icx_token_ck_eth_dummy_canister_id = principal("ayn3r-qthap-u2k6w-4avsn-wwxu3-mowpf-5c42n-p77ru-jb5bn-6srs4-2vq");
    let token_sns_icx_token_ck_eth_subaccount = hex::decode("6703e9a57adc0564db5af4db1d6797a2e69afffe34487a16fa51973558c00185").unwrap();
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[0].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000".to_string(), decimals: 7, dummy_canister_id: token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000".to_string(), total_supply: "44_721_359_549".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "10_102_030_607".to_string(), reserve1: "198_000_000_000".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "1/100".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[1].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "894_427_190_999_915".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "2_000_000_000_000_000_000".to_string(), reserve1: "400_000_000_000".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    // ICX -> USDT 10_102_030_607 -> 198_000_000_000
    // USDT -> ETH 400_000_000_000 -> 2_000_000_000_000_000_000
    assert_eq!(default.pair_create(TokenPairCreateOrRemoveArgs { pool: TokenPairPool { token0: token_sns_icx_canister_id, token1: token_ck_eth_canister_id, amm: "swap_v2_0.3%".to_string() }, memo: None, created: None }).unwrap(), TokenPairCreateOrRemoveResult::Ok(MarketMakerView::SwapV2(SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "1_000_000_000".to_string(), decimals: 13, dummy_canister_id: token_sns_icx_token_ck_eth_dummy_canister_id.to_text(), minimum_liquidity: "1_000_000_000_000".to_string(), total_supply: "0".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "0".to_string(), reserve1: "0".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_eth_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_eth_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() })));
    // assert_eq!(default.config_protocol_fee_replace(token_sns_icx_token_ck_eth_subaccount.clone().into(), Some(SwapRatio { numerator: 1, denominator: 6 })).unwrap(), None);
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_eq!(default.block_swap_get(10).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_swap_block_pair_create(archive_swap.sender(default_identity).get_block(10).unwrap().unwrap(), archive_swap::PairCreate { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_eth_canister_id  }, amm: archive_swap::Amm::SwapV2T3 }, creator: default_identity  }); // 👁︎
    assert_eq!(default.block_swap_get(10).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.request_trace_get(11).unwrap().unwrap().traces[0].1, format!("*TokenPairCreate* `token0:[{}], token1:[{}], amm:swap_v2_0.3%, subaccount:({}), dummyCanisterId:[{}]`", token_sns_icx_canister_id.to_text(), token_ck_eth_canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_eth_subaccount), token_sns_icx_token_ck_eth_dummy_canister_id.to_text())); // 👁︎
    assert_swap_v2_market_maker(&alice.pair_query(TokenPairPool { token0: token_sns_icx_canister_id, token1: token_ck_eth_canister_id, amm: "swap_v2_0.3%".to_string() }).unwrap().unwrap(), SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "1_000_000_000".to_string(), decimals: 13, dummy_canister_id: token_sns_icx_token_ck_eth_dummy_canister_id.to_text(), minimum_liquidity: "1_000_000_000_000".to_string(), total_supply: "0".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "0".to_string(), reserve1: "0".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_eth_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_eth_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_sns_icx_canister_id, nat(9_897_969_393)), (token_ck_eth_canister_id, nat(1_000_000_000_000_000_000)), (token_sns_icx_token_ck_eth_dummy_canister_id, nat(0))]);
    assert_eq!(default.pair_liquidity_add(TokenPairLiquidityAddArgs { swap_pair: SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_eth_canister_id), amm: "swap_v2_0.3%".to_string() }, from: account(default_identity), to: account(default_identity), amount_desired: (nat(2_000_000_000), nat(100_000_000_000_000_000)), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityAddResult::Ok(TokenPairLiquidityAddSuccess { liquidity: nat(14_142_135_623_730), amount: (nat(2_000_000_000), nat(100_000_000_000_000_000)) }));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_eq!(default.block_swap_get(11).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(12).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(13).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_swap_block_pair_v2_mint(archive_swap.sender(default_identity).get_block(11).unwrap().unwrap(), archive_swap::SwapV2MintToken { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_eth_canister_id }, amm: archive_swap::Amm::SwapV2T3 }, from: archive_swap_account(default_identity), to: archive_swap_account(default_identity), token0: token_sns_icx_canister_id, token1: token_ck_eth_canister_id, amount0: nat(2_000_000_000), amount1: nat(100_000_000_000_000_000), token: token_sns_icx_token_ck_eth_dummy_canister_id, amount: nat(14_142_135_623_730) }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(12).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_eth_canister_id }, amm: archive_swap::Amm::SwapV2T3 }, block_timestamp: 0, supply: nat(14_142_135_623_730), reserve0: nat(2_000_000_000), reserve1: nat(100_000_000_000_000_000), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(15).unwrap().unwrap(), archive_token::TransferToken { token: token_sns_icx_canister_id, from: archive_token_account(default_identity), amount: nat(2_000_000_000), to: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(16).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_eth_canister_id, from: archive_token_account(default_identity), amount: nat(100_000_000_000_000_000), to: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_deposit(archive_token.sender(default_identity).get_block(17).unwrap().unwrap(), archive_token::DepositToken { token: token_sns_icx_token_ck_eth_dummy_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }, amount: nat(14_142_135_623_730), to: archive_token_account(default_identity) }); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(18).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(15).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(16).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(17).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(18).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.request_trace_get(12).unwrap().unwrap().traces[1].1, format!("*TokenTransfer* `token:[{}], from:({}.), to:({}.{}), amount:2_000_000_000, done:2_000_000_000`", token_sns_icx_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_eth_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(12).unwrap().unwrap().traces[2].1, format!("*TokenTransfer* `token:[{}], from:({}.), to:({}.{}), amount:100_000_000_000_000_000, done:100_000_000_000_000_000`", token_ck_eth_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_eth_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(12).unwrap().unwrap().traces[3].1, format!("*PairLiquidityAdd* `amount_a:2_000_000_000, amount_b:100_000_000_000_000_000`{}", "")); // 👁︎
    assert_eq!(default.request_trace_get(12).unwrap().unwrap().traces[4].1, format!("*PairLiquidityMint(Deposit)* `token:[{}], from[transferred 2 tokens]:({}.{}), to[minted liquidity]:({}.), amount:14_142_135_623_730`", token_sns_icx_token_ck_eth_dummy_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_eth_subaccount), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(12).unwrap().unwrap().traces[5].1, format!("*PairLiquidityMint* `token:[{}], to:({}.), amount:14_142_135_623_730`", token_sns_icx_token_ck_eth_dummy_canister_id.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(13).unwrap(), None); // 👁︎
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[0].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "1_000_000_000".to_string(), decimals: 13, dummy_canister_id: token_sns_icx_token_ck_eth_dummy_canister_id.to_text(), minimum_liquidity: "1_000_000_000_000".to_string(), total_supply: "14_142_135_623_730".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "2_000_000_000".to_string(), reserve1: "100_000_000_000_000_000".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_eth_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_eth_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_swap_v2_market_maker(&alice.pair_query(TokenPairPool { token0: token_sns_icx_canister_id, token1: token_ck_eth_canister_id, amm: "swap_v2_0.3%".to_string() }).unwrap().unwrap(), SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "1_000_000_000".to_string(), decimals: 13, dummy_canister_id: token_sns_icx_token_ck_eth_dummy_canister_id.to_text(), minimum_liquidity: "1_000_000_000_000".to_string(), total_supply: "14_142_135_623_730".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "2_000_000_000".to_string(), reserve1: "100_000_000_000_000_000".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_eth_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_eth_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_sns_icx_canister_id, nat(7_897_969_393)), (token_ck_eth_canister_id, nat(900_000_000_000_000_000)), (token_sns_icx_token_ck_eth_dummy_canister_id, nat(14_142_135_623_730))]);
    assert_tokens_balance(default.tokens_balance_of(swap::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }).unwrap(), vec![(token_sns_icx_canister_id, nat(2_000_000_000)), (token_ck_eth_canister_id, nat(100_000_000_000_000_000)), (token_sns_icx_token_ck_eth_dummy_canister_id, nat(0))]);
    // ETH -> ICX 100_000_000_000_000_000 -> 20_000_000_000
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(),                                                                               vec![(token_sns_icx_canister_id, nat( 7_897_969_393)), (token_ck_eth_canister_id, nat(  900_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(202_000_000_000))]);
    assert_tokens_balance(default.tokens_balance_by(Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_sns_icx_canister_id, nat(10_102_030_607)), (token_ck_eth_canister_id, nat(                        0)), (token_ck_usdt_canister_id, nat(198_000_000_000))]);
    assert_tokens_balance(default.tokens_balance_by(Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }).unwrap(),  vec![(token_sns_icx_canister_id, nat(             0)), (token_ck_eth_canister_id, nat(2_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(400_000_000_000))]);
    assert_tokens_balance(default.tokens_balance_by(Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }).unwrap(),  vec![(token_sns_icx_canister_id, nat( 2_000_000_000)), (token_ck_eth_canister_id, nat(  100_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(              0))]);
    assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(),                                                                               vec![(token_sns_icx_canister_id, nat(             0)), (token_ck_eth_canister_id, nat(                        0)), (token_ck_usdt_canister_id, nat(              0))]);
    assert_eq!(default.pair_swap_by_loan(TokenPairSwapByLoanArgs { from: account(default_identity), loan: nat(2_000_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }, SwapTokenPair { token: (token_ck_usdt_canister_id, token_ck_eth_canister_id), amm: "swap_v2_0.3%".to_string() }, SwapTokenPair { token: (token_ck_eth_canister_id, token_sns_icx_canister_id), amm: "swap_v2_0.3%".to_string() }, ], to: account(bob_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Err(BusinessError::Swap("INSUFFICIENT_OUTPUT_AMOUNT: 1_197_438_008".to_string())));
    assert_eq!(default.request_trace_get(13).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(21).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(18).unwrap(), None); // 👁︎
    assert_eq!(default.block_swap_get(15).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(13).unwrap(), None); // 👁︎
    assert_eq!(default.pair_swap_by_loan(TokenPairSwapByLoanArgs { from: account(default_identity), loan: nat(20_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }, SwapTokenPair { token: (token_ck_usdt_canister_id, token_ck_eth_canister_id), amm: "swap_v2_0.3%".to_string() }, SwapTokenPair { token: (token_ck_eth_canister_id, token_sns_icx_canister_id), amm: "swap_v2_0.3%".to_string() }, ], to: account(bob_identity), deadline: None, created: None, memo: None }, None).unwrap(), TokenPairSwapTokensResult::Ok(TokenPairSwapTokensSuccess { amounts: vec![nat(20_000), nat(388_079), nat(1_934_571_943_713), nat(38_574)] }));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_swap_block_pair_swap(archive_swap.sender(default_identity).get_block(13).unwrap().unwrap(), archive_swap::PairSwapToken { token_a: token_sns_icx_canister_id, token_b: token_ck_usdt_canister_id, amm: archive_swap::Amm::SwapV2H1, from: archive_swap_account(canister_id), to: archive_swap::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }, amount_a: nat(20_000), amount_b: nat(388_079) }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(14).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2H1 }, block_timestamp: 0, supply: nat(44_721_359_549), reserve0: nat(10_102_050_607), reserve1: nat(197_999_611_921), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_swap_block_pair_swap(archive_swap.sender(default_identity).get_block(15).unwrap().unwrap(), archive_swap::PairSwapToken { token_a: token_ck_usdt_canister_id, token_b: token_ck_eth_canister_id, amm: archive_swap::Amm::SwapV2T3, from: archive_swap::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }, to: archive_swap::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }, amount_a: nat(388_079), amount_b: nat(1_934_571_943_713) }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(16).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2T3 }, block_timestamp: 0, supply: nat(894_427_190_999_915), reserve0: nat(1_999_998_065_428_056_287), reserve1: nat(400_000_388_079), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_swap_block_pair_swap(archive_swap.sender(default_identity).get_block(17).unwrap().unwrap(), archive_swap::PairSwapToken { token_a: token_ck_eth_canister_id, token_b: token_sns_icx_canister_id, amm: archive_swap::Amm::SwapV2T3, from: archive_swap::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }, to: archive_swap_account(bob_identity), amount_a: nat(1_934_571_943_713), amount_b: nat(38_574) }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(18).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_eth_canister_id }, amm: archive_swap::Amm::SwapV2T3 }, block_timestamp: 0, supply: nat(14_142_135_623_730), reserve0: nat(1_999_961_426), reserve1: nat(100_001_934_571_943_713), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(19).unwrap(), None); // 👁︎
    assert_eq!(default.block_swap_get(13).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(14).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(15).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(16).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(17).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(18).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(19).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_token_block_deposit(archive_token.sender(default_identity).get_block(18).unwrap().unwrap(), archive_token::DepositToken { token: token_sns_icx_canister_id, from: archive_token_account(canister_id), amount: nat(20_000), to: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) } }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(19).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_usdt_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, amount: nat(388_079), to: archive_token::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(20).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_eth_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }, amount: nat(1_934_571_943_713), to: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(21).unwrap().unwrap(), archive_token::TransferToken { token: token_sns_icx_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }, amount: nat(38_574), to: archive_token_account(bob_identity), fee: None }); // 👁︎
    assert_token_block_withdraw(archive_token.sender(default_identity).get_block(22).unwrap().unwrap(), archive_token::DepositToken { token: token_sns_icx_canister_id, from: archive_token_account(bob_identity), amount: nat(20_000), to: archive_token_account(canister_id) }); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(23).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(18).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(19).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(20).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(21).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(22).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(23).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.request_trace_get(13).unwrap().unwrap().traces[0].1, format!("*TokenLoan* `token:[{}], from:({}.), to:({}.{}), amount:20_000`", token_sns_icx_canister_id.to_text(), canister_id.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(13).unwrap().unwrap().traces[1].1, format!("*TokenTransfer* `token:[{}], from:({}.{}), to:({}.{}), amount:388_079, done:388_079`", token_ck_usdt_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(13).unwrap().unwrap().traces[2].1, format!("*TokenPairSwap* `swap_pair:([{}],[{}],swap_v2_1%), from:({}.), to:({}.{}), pay_amount:20_000, got_amount:388_079`", token_sns_icx_canister_id.to_text(), token_ck_usdt_canister_id.to_text(), canister_id.to_text(), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(13).unwrap().unwrap().traces[4].1, format!("*TokenTransfer* `token:[{}], from:({}.{}), to:({}.{}), amount:1_934_571_943_713, done:1_934_571_943_713`", token_ck_eth_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_eth_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(13).unwrap().unwrap().traces[5].1, format!("*TokenPairSwap* `swap_pair:([{}],[{}],swap_v2_0.3%), from:({}.{}), to:({}.{}), pay_amount:388_079, got_amount:1_934_571_943_713`", token_ck_usdt_canister_id.to_text(), token_ck_eth_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_eth_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(13).unwrap().unwrap().traces[7].1, format!("*TokenTransfer* `token:[{}], from:({}.{}), to:({}.), amount:38_574, done:38_574`", token_sns_icx_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_eth_subaccount), bob_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(13).unwrap().unwrap().traces[8].1, format!("*TokenPairSwap* `swap_pair:([{}],[{}],swap_v2_0.3%), from:({}.{}), to:({}.), pay_amount:1_934_571_943_713, got_amount:38_574`", token_ck_eth_canister_id.to_text(), token_sns_icx_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_eth_subaccount), bob_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(13).unwrap().unwrap().traces[10].1, format!("*TokenRepay* `token:[{}], from:({}.), to:({}.), amount:20_000`", token_sns_icx_canister_id.to_text(), bob_identity.to_text(), canister_id.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(14).unwrap(), None); // 👁︎
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(),                                                                               vec![(token_sns_icx_canister_id, nat( 7_897_969_393)), (token_ck_eth_canister_id, nat(  900_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(202_000_000_000))]);
  //assert_tokens_balance(default.tokens_balance_by(Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_sns_icx_canister_id, nat(10_102_030_607)), (token_ck_eth_canister_id, nat(                        0)), (token_ck_usdt_canister_id, nat(198_000_000_000))]);
    assert_tokens_balance(default.tokens_balance_by(Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_sns_icx_canister_id, nat(10_102_050_607)), (token_ck_eth_canister_id, nat(                        0)), (token_ck_usdt_canister_id, nat(197_999_611_921))]); // +20_000,                   , -388_079
  //assert_tokens_balance(default.tokens_balance_by(Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }).unwrap(),  vec![(token_sns_icx_canister_id, nat(             0)), (token_ck_eth_canister_id, nat(2_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(400_000_000_000))]);
    assert_tokens_balance(default.tokens_balance_by(Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }).unwrap(),  vec![(token_sns_icx_canister_id, nat(             0)), (token_ck_eth_canister_id, nat(1_999_998_065_428_056_287)), (token_ck_usdt_canister_id, nat(400_000_388_079))]); //        , -1_934_571_943_713, +388_079
  //assert_tokens_balance(default.tokens_balance_by(Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }).unwrap(),  vec![(token_sns_icx_canister_id, nat( 2_000_000_000)), (token_ck_eth_canister_id, nat(  100_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(              0))]);
    assert_tokens_balance(default.tokens_balance_by(Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_eth_subaccount.clone().into()) }).unwrap(),  vec![(token_sns_icx_canister_id, nat( 1_999_961_426)), (token_ck_eth_canister_id, nat(  100_001_934_571_943_713)), (token_ck_usdt_canister_id, nat(              0))]); // -38_574, +1_934_571_943_713,
  //assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(),                                                                               vec![(token_sns_icx_canister_id, nat(             0)), (token_ck_eth_canister_id, nat(                        0)), (token_ck_usdt_canister_id, nat(              0))]);
    assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(),                                                                               vec![(token_sns_icx_canister_id, nat(        18_574)), (token_ck_eth_canister_id, nat(                        0)), (token_ck_usdt_canister_id, nat(              0))]); // +38_574-20_000,

    // 🚩 2.6 business pair swap with deposit and withdraw
    assert_eq!(default.pair_swap_with_deposit_and_withdraw(TokenPairSwapWithDepositAndWithdrawArgs { from: account(default_identity), deposit_amount_without_fee: nat(20_000_000_000_000), amount_out_min: nat(20_000_000_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), created: None, memo: None, deadline: None, withdraw_fee: None, deposit_fee: None }).unwrap().0, TokenChangedResult::Err(BusinessError::Swap("INSUFFICIENT_OUTPUT_AMOUNT: 197_898_643_127".to_string()))); // 👁︎
    assert_eq!(default.pair_swap_with_deposit_and_withdraw(TokenPairSwapWithDepositAndWithdrawArgs { from: account(default_identity), deposit_amount_without_fee: nat(2_000_000_000), amount_out_min: nat(20_000_000_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), created: None, memo: None, deadline: None, withdraw_fee: None, deposit_fee: None }).unwrap().0, TokenChangedResult::Err(BusinessError::Swap("INSUFFICIENT_OUTPUT_AMOUNT: 32_448_070_642".to_string()))); // 👁︎
    assert_eq!(default.pair_swap_with_deposit_and_withdraw(TokenPairSwapWithDepositAndWithdrawArgs { from: account(default_identity), deposit_amount_without_fee: nat(20_000_000_000_000), amount_out_min: nat(20_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), created: None, memo: None, deadline: None, withdraw_fee: None, deposit_fee: None }).unwrap().0, TokenChangedResult::Err(BusinessError::TransferFromError(TransferFromError::InsufficientAllowance{ allowance: nat(79_999_900_000) }))); // 👁︎
    assert_eq!(token_sns_icx.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(979_999_800_000));
    assert_eq!(token_ck_usdt.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(99_199_999_980_000));
    assert_eq!(default.pair_swap_with_deposit_and_withdraw(TokenPairSwapWithDepositAndWithdrawArgs { from: account(default_identity), deposit_amount_without_fee: nat(200_000_000), amount_out_min: nat(20_000_000), path: vec![SwapTokenPair { token: (token_sns_icx_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_1%".to_string() }], to: account(default_identity), created: None, memo: None, deadline: None, withdraw_fee: None, deposit_fee: None }).unwrap(), (TokenChangedResult::Ok(nat(3)), Some(TokenPairSwapTokensResult::Ok(TokenPairSwapTokensSuccess { amounts: vec![nat(200_000_000), nat(3_806_187_431)] })), Some(TokenChangedResult::Ok(nat(3))))); // 👁︎
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_swap_block_pair_swap(archive_swap.sender(default_identity).get_block(19).unwrap().unwrap(), archive_swap::PairSwapToken { token_a: token_sns_icx_canister_id, token_b: token_ck_usdt_canister_id, amm: archive_swap::Amm::SwapV2H1, from: archive_swap_account(default_identity), to: archive_swap_account(default_identity), amount_a: nat(200_000_000), amount_b: nat(3_806_187_431) }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(20).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_sns_icx_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2H1 }, block_timestamp: 0, supply: nat(44_721_359_549), reserve0: nat(10_302_050_607), reserve1: nat(194_193_424_490), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(21).unwrap(), None); // 👁︎
    assert_eq!(default.block_swap_get(19).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(20).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(21).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_token_block_deposit(archive_token.sender(default_identity).get_block(23).unwrap().unwrap(), archive_token::DepositToken { token: token_sns_icx_canister_id, from: archive_token_account(default_identity), amount: nat(200_000_000), to: archive_token_account(default_identity) }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(24).unwrap().unwrap(), archive_token::TransferToken { token: token_sns_icx_canister_id, from: archive_token_account(default_identity), amount: nat(200_000_000), to: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, fee: None }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(25).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_usdt_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_sns_icx_token_ck_usdt_subaccount.clone().into()) }, amount: nat(3_806_187_431), to: archive_token_account(default_identity), fee: None }); // 👁︎
    assert_token_block_withdraw(archive_token.sender(default_identity).get_block(26).unwrap().unwrap(), archive_token::DepositToken { token: token_ck_usdt_canister_id, from: archive_token_account(default_identity), amount: nat(3_806_187_431), to: archive_token_account(default_identity) }); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(27).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(23).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(24).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(25).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(26).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(27).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.request_trace_get(14).unwrap().unwrap().traces[0].1, format!("*Deposit* `token:[{}], from:({}.), to:({}.), amount:200_000_000, height:3`", token_sns_icx_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(15).unwrap().unwrap().traces[0].1, format!("*TokenTransfer* `token:[{}], from:({}.), to:({}.{}), amount:200_000_000, done:200_000_000`", token_sns_icx_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount))); // 👁︎
    assert_eq!(default.request_trace_get(15).unwrap().unwrap().traces[1].1, format!("*TokenTransfer* `token:[{}], from:({}.{}), to:({}.), amount:3_806_187_431, done:3_806_187_431`", token_ck_usdt_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_sns_icx_token_ck_usdt_subaccount), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(15).unwrap().unwrap().traces[2].1, format!("*TokenPairSwap* `swap_pair:([{}],[{}],swap_v2_1%), from:({}.), to:({}.), pay_amount:200_000_000, got_amount:3_806_187_431`", token_sns_icx_canister_id.to_text(), token_ck_usdt_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(16).unwrap().unwrap().traces[0].1, format!("*Withdraw* `token:[{}], from:({}.), to:({}.), amount:3_806_187_431, height:3`", token_ck_usdt_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(token_sns_icx.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(979_799_700_000));
    assert_eq!(token_ck_usdt.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(99_203_806_157_431));

    // 🚩 2.7 business pair liquidity remove
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[1].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000".to_string(), decimals: 7, dummy_canister_id: token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000".to_string(), total_supply: "44_721_359_549".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "10_302_050_607".to_string(), reserve1: "194_193_424_490".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "1/100".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[2].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "894_427_190_999_915".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "1_999_998_065_428_056_287".to_string(), reserve1: "400_000_388_079".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[0].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "1_000_000_000".to_string(), decimals: 13, dummy_canister_id: token_sns_icx_token_ck_eth_dummy_canister_id.to_text(), minimum_liquidity: "1_000_000_000_000".to_string(), total_supply: "14_142_135_623_730".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "1_999_961_426".to_string(), reserve1: "100_001_934_571_943_713".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_eth_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_eth_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(894_427_190_999_915)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(44_721_359_549)), (token_sns_icx_token_ck_eth_dummy_canister_id, nat(14_142_135_623_730))]);
    assert_tokens_balance(  alice.tokens_balance_of(account(  alice_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(                  0)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(             0)), (token_sns_icx_token_ck_eth_dummy_canister_id, nat(                 0))]);
    assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(                  0)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(             0)), (token_sns_icx_token_ck_eth_dummy_canister_id, nat(                 0))]);
    assert_eq!(default.pair_liquidity_remove(TokenPairLiquidityRemoveArgs { swap_pair: SwapTokenPair { token: (token_ck_eth_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_0.3%".to_string() }, from: account(default_identity), to: account(default_identity), liquidity_without_fee: nat(1_894_427_190_999_915), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityRemoveResult::Err(BusinessError::Liquidity("INSUFFICIENT_LIQUIDITY".to_string())));
    assert_eq!(default.pair_liquidity_remove(TokenPairLiquidityRemoveArgs { swap_pair: SwapTokenPair { token: (token_ck_eth_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_0.3%".to_string() }, from: account(default_identity), to: account(default_identity), liquidity_without_fee: nat(894_327_190_999_916), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityRemoveResult::Err(BusinessError::Liquidity("REMAIN_TOTAL_LIQUIDITY_TOO_SMALL".to_string())));
    assert_eq!(default.request_trace_get(17).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(27).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.block_token_get(28).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.block_token_get(29).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(27).unwrap(), None); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(28).unwrap(), None); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(29).unwrap(), None); // 👁︎
    assert_eq!(default.block_swap_get(21).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(21).unwrap(), None); // 👁︎
    assert_eq!(default.pair_liquidity_remove(TokenPairLiquidityRemoveArgs { swap_pair: SwapTokenPair { token: (token_ck_eth_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_0.3%".to_string() }, from: account(default_identity), to: account(default_identity), liquidity_without_fee: nat(494_427_190_999_915), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityRemoveResult::Ok(TokenPairLiquidityRemoveSuccess { amount: (nat(1_105_571_739_595_014_231), nat(221_114_776_324)) }));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    // mint fee if fee_to set up
    assert_swap_block_pair_v2_burn(archive_swap.sender(default_identity).get_block(21).unwrap().unwrap(), archive_swap::SwapV2BurnToken { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2T3 }, from: archive_swap_account(default_identity), to: archive_swap_account(default_identity), token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amount0: nat(1_105_571_739_595_014_231), amount1: nat(221_114_776_324), token: token_ck_eth_token_ck_usdt_dummy_canister_id, amount: nat(494_427_190_999_915), fee: None }); // 👁︎
    assert_swap_block_pair_v2_state(archive_swap.sender(default_identity).get_block(22).unwrap().unwrap(), archive_swap::SwapV2State { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2T3 }, block_timestamp: 0, supply: nat(400_000_000_000_000), reserve0: nat(894_426_325_833_042_056), reserve1: nat(178_885_611_755), price_cumulative_exponent: 64, price0_cumulative: nat(0), price1_cumulative: nat(0) }); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(23).unwrap(), None); // 👁︎
    // mint fee if fee_to set up
    assert_eq!(default.block_swap_get(21).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(22).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(23).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    // mint fee if fee_to set up
    assert_token_block_withdraw(archive_token.sender(default_identity).get_block(27).unwrap().unwrap(), archive_token::DepositToken { token: token_ck_eth_token_ck_usdt_dummy_canister_id, from: archive_token_account(default_identity), amount: nat(494_427_190_999_915), to: archive_token::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) } }); // 👁︎
    // deposit burn fee if fee_to set up
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(28).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_eth_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into())}, amount: nat(1_105_571_739_595_014_231), to: archive_token_account(default_identity), fee: None }); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(29).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_usdt_canister_id, from: archive_token::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into())}, amount: nat(221_114_776_324), to: archive_token_account(default_identity), fee: None }); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(30).unwrap(), None); // 👁︎
    // mint fee if fee_to set up
    assert_eq!(default.block_token_get(27).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    // deposit burn fee if fee_to set up
    assert_eq!(default.block_token_get(28).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(29).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(30).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    // mint fee if fee_to set up
    // mint fee if fee_to set up
    assert_eq!(default.request_trace_get(17).unwrap().unwrap().traces[1].1, format!("*PairLiquidityBurn(Withdraw)* `token:[{}], from[burned liquidity]:({}.), to[withdrawn 2 tokens]:({}.{}), amount:494_427_190_999_915, fee:None`", token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), default_identity.to_text(), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount))); // 👁︎
    // deposit burn fee if fee_to set up
    assert_eq!(default.request_trace_get(17).unwrap().unwrap().traces[2].1, format!("*PairLiquidityBurn* `token:[{}], from:({}.), amount:494_427_190_999_915`", token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(17).unwrap().unwrap().traces[3].1, format!("*TokenTransfer* `token:[{}], from:({}.{}), to:({}.), amount:1_105_571_739_595_014_231, done:1_105_571_739_595_014_231`", token_ck_eth_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(17).unwrap().unwrap().traces[4].1, format!("*TokenTransfer* `token:[{}], from:({}.{}), to:({}.), amount:221_114_776_324, done:221_114_776_324`", token_ck_usdt_canister_id.to_text(), canister_id.to_text(), hex::encode(&token_ck_eth_token_ck_usdt_subaccount), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(18).unwrap(), None); // 👁︎
    assert_swap_v2_market_maker(&alice.pair_query(TokenPairPool { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_0.3%".to_string() }).unwrap().unwrap(), SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "400_000_000_000_000".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "894_426_325_833_042_056".to_string(), reserve1: "178_885_611_755".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(2_005_571_739_595_014_231)), (token_ck_usdt_canister_id, nat(423_114_776_324)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(400_000_000_000_000))]);
    assert_tokens_balance(default.tokens_balance_of(swap::Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_ck_eth_canister_id, nat(894_426_325_833_042_056)), (token_ck_usdt_canister_id, nat(178_885_611_755)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[1].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000".to_string(), decimals: 7, dummy_canister_id: token_sns_icx_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000".to_string(), total_supply: "44_721_359_549".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "10_302_050_607".to_string(), reserve1: "194_193_424_490".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "1/100".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[2].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "400_000_000_000_000".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "894_426_325_833_042_056".to_string(), reserve1: "178_885_611_755".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_swap_v2_market_maker(&alice.pairs_query().unwrap()[0].1, SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "1_000_000_000".to_string(), decimals: 13, dummy_canister_id: token_sns_icx_token_ck_eth_dummy_canister_id.to_text(), minimum_liquidity: "1_000_000_000_000".to_string(), total_supply: "14_142_135_623_730".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "1_999_961_426".to_string(), reserve1: "100_001_934_571_943_713".to_string(), subaccount: hex::encode(&token_sns_icx_token_ck_eth_subaccount), price1_cumulative_last: "0".to_string(), token0: token_sns_icx_canister_id.to_text(), token1: token_ck_eth_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() });
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(400_000_000_000_000)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(44_721_359_549)), (token_sns_icx_token_ck_eth_dummy_canister_id, nat(14_142_135_623_730))]);
    assert_tokens_balance(  alice.tokens_balance_of(account(  alice_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(                  0)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(             0)), (token_sns_icx_token_ck_eth_dummy_canister_id, nat(                 0))]);
    assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(                  0)), (token_sns_icx_token_ck_usdt_dummy_canister_id, nat(             0)), (token_sns_icx_token_ck_eth_dummy_canister_id, nat(                 0))]);

    // 🚩 2.8 business pair liquidity transfer
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(400_000_000_000_000))]);
    assert_tokens_balance(  alice.tokens_balance_of(account(  alice_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(                  0))]);
    assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(                  0))]);
    assert_eq!(default.token_transfer(TokenTransferArgs { token: token_ck_eth_token_ck_usdt_dummy_canister_id, from: account(default_identity), transfer_amount_without_fee: nat(100_000_000_000_000), to: account(bob_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(100_000_000_000_000)));
    std::thread::sleep(std::time::Duration::from_secs(2)); // 🕰︎
    archive_swap.sender(canister_id).append_blocks(vec![]).unwrap(); // ! BUG
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(300_000_000_000_000))]);
    assert_tokens_balance(  alice.tokens_balance_of(account(  alice_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(                  0))]);
    assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(), vec![(token_ck_eth_token_ck_usdt_dummy_canister_id, nat(100_000_000_000_000))]);
    assert_swap_block_pair_v2_transfer(archive_swap.sender(default_identity).get_block(23).unwrap().unwrap(), archive_swap::SwapV2TransferToken { pa: archive_swap::TokenPairAmm { pair: archive_swap::TokenPair { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id }, amm: archive_swap::Amm::SwapV2T3 }, token: token_ck_eth_token_ck_usdt_dummy_canister_id, from: archive_swap_account(default_identity), to: archive_swap_account(bob_identity), amount: nat(100_000_000_000_000), fee: None}); // 👁︎
    assert_eq!(archive_swap.sender(default_identity).get_block(24).unwrap(), None); // 👁︎
    assert_eq!(default.block_swap_get(23).unwrap(), QuerySwapBlockResult::Archive(archive_swap_canister_id)); // 👁︎
    assert_eq!(default.block_swap_get(24).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_token_block_transfer(archive_token.sender(default_identity).get_block(30).unwrap().unwrap(), archive_token::TransferToken { token: token_ck_eth_token_ck_usdt_dummy_canister_id, from: archive_token_account(default_identity), amount: nat(100_000_000_000_000), to: archive_token_account(bob_identity), fee: None }); // 👁︎
    assert_eq!(archive_token.sender(default_identity).get_block(31).unwrap(), None); // 👁︎
    assert_eq!(default.block_token_get(30).unwrap(), QueryTokenBlockResult::Archive(archive_token_canister_id)); // 👁︎
    assert_eq!(default.block_token_get(31).unwrap_err().reject_message.contains("invalid block height"), true); // 👁︎
    assert_eq!(default.request_trace_get(18).unwrap().unwrap().traces[0].1, format!("*Transfer* `token:[{}], from:({}.), to:({}.), amount:100_000_000_000_000, fee:None`", token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), default_identity.to_text(), bob_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(18).unwrap().unwrap().traces[1].1, format!("*Transfer(Swap)* `token:[{}], from:({}.), to:({}.), amount:100_000_000_000_000, fee:None`", token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), default_identity.to_text(), bob_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(19).unwrap(), None); // 👁︎

    // 🚩 3 test stable data
    assert_eq!(default.pause_replace(Some("reason".to_string())).unwrap(), ());
    assert_eq!(default.pause_query().unwrap(), true);
    pic.upgrade_canister(canister_id, WASM_MODULE.to_vec(), encode_one(None::<()>).unwrap(), Some(default_identity)).unwrap();
    assert_eq!(default.pause_replace(None).unwrap(), ());
    assert_eq!(default.pause_query().unwrap(), false);

    // 🚩 4 test archive-token
    assert_token_block_chain_view(&default.config_token_block_chain_query(BlockChainArgs::BlockChainQuery).unwrap(), BlockChainView { current_archiving: Some(CurrentArchiving { canister_id: archive_token_canister_id, length: 31, max_length: 100_000_000, block_height_offset: 0 }), latest_block_hash: vec![].into(), archive_config: NextArchiveCanisterConfig { maintainers: Some(vec![default_identity]), max_memory_size_bytes: None, max_length: 100_000_000 }, next_block_index: 31, archived: vec![] });
    assert_token_archive_wasm_module(default.config_token_block_chain_query(BlockChainArgs::WasmModuleQuery).unwrap(), ARCHIVE_TOKEN_WASM_MODULE.to_vec());
    assert_token_archive_wasm_module(default.config_token_block_chain_update(BlockChainArgs::WasmModuleUpdate(serde_bytes::ByteBuf::from(vec![1, 2, 3]))).unwrap(), ARCHIVE_TOKEN_WASM_MODULE.to_vec());
    assert_token_archive_wasm_module(default.config_token_block_chain_query(BlockChainArgs::WasmModuleQuery).unwrap(), vec![1, 2, 3]);
    assert_token_current_archiving_max_length(default.config_token_block_chain_update(BlockChainArgs::CurrentArchivingMaxLengthUpdate(1000)).unwrap(), CurrentArchiving { canister_id: archive_token_canister_id, length: 31, max_length: 1_000, block_height_offset: 0 });
    assert_token_block_chain_view(&default.config_token_block_chain_query(BlockChainArgs::BlockChainQuery).unwrap(), BlockChainView { current_archiving: Some(CurrentArchiving { canister_id: archive_token_canister_id, length: 31, max_length: 1_000, block_height_offset: 0 }), latest_block_hash: vec![].into(), archive_config: NextArchiveCanisterConfig { maintainers: Some(vec![default_identity]), max_memory_size_bytes: None, max_length: 1_000 }, next_block_index: 31, archived: vec![] });
    assert_token_archive_config_replace(default.config_token_block_chain_update(BlockChainArgs::NextArchiveCanisterConfigUpdate(NextArchiveCanisterConfig{ maintainers: Some(vec![bob_identity]), max_memory_size_bytes: Some(2_000_000), max_length: 10000 })).unwrap(), NextArchiveCanisterConfig { maintainers: Some(vec![default_identity]), max_memory_size_bytes: None, max_length: 1_000 });
    assert_token_block_chain_view(&default.config_token_block_chain_query(BlockChainArgs::BlockChainQuery).unwrap(), BlockChainView { current_archiving: Some(CurrentArchiving { canister_id: archive_token_canister_id, length: 31, max_length: 1_000, block_height_offset: 0 }), latest_block_hash: vec![].into(), archive_config: NextArchiveCanisterConfig { maintainers: Some(vec![bob_identity]), max_memory_size_bytes: Some(2_000_000), max_length: 10_000 }, next_block_index: 31, archived: vec![] });
    assert_eq!(default.config_token_block_chain_update(BlockChainArgs::ArchivedCanisterMaintainersUpdate{ canister_id: archive_token_canister_id, maintainers: Some(vec![alice_identity]) }).unwrap(), swap::TokenBlockResult::Ok(swap::TokenBlockResponse::ArchivedCanisterMaintainers));
    assert_eq!(default.config_token_block_chain_update(BlockChainArgs::ArchivedCanisterMaxMemorySizeBytesUpdate{ canister_id: archive_token_canister_id, max_memory_size_bytes: 300 }).unwrap(), swap::TokenBlockResult::Err(BusinessError::CallCanisterError(format!("call rejected: 5 - IC0503: Error from Canister {}: Canister called `ic0.trap` with message: 'Cannot set max_memory_size_bytes to 300, because it is lower than total_block_size 3805.'.\nConsider gracefully handling failures from this canister or altering the canister to handle exceptions. See documentation: https://internetcomputer.org/docs/current/references/execution-errors#trapped-explicitly", archive_token_canister_id.to_text()))));
    assert_eq!(default.config_token_block_chain_update(BlockChainArgs::ArchivedCanisterMaxMemorySizeBytesUpdate{ canister_id: archive_token_canister_id, max_memory_size_bytes: 3_000_000 }).unwrap(), swap::TokenBlockResult::Ok(swap::TokenBlockResponse::ArchivedCanisterMaxMemorySizeBytes));

    // 🚩 4.1 test archive-token
    assert_eq!(archive_token.sender(default_identity).get_blocks(archive_token::GetBlocksArgs{ start: 0, length: 100 }).unwrap_err().reject_message, "Only Maintainers are allowed to query data".to_string());
    assert_eq!(archive_token.sender(bob_identity).get_blocks(archive_token::GetBlocksArgs{ start: 0, length: 100 }).unwrap_err().reject_message, "Only Maintainers are allowed to query data".to_string());
    assert_token_get_blocks(archive_token.sender(alice_identity).get_blocks(archive_token::GetBlocksArgs{ start: 0, length: 100 }).unwrap(), 31);

    // 🚩 5 test archive-swap
    assert_swap_block_chain_view(&default.config_swap_block_chain_query(BlockChainArgs::BlockChainQuery).unwrap(), BlockChainView { current_archiving: Some(CurrentArchiving { canister_id: archive_swap_canister_id, length: 24, max_length: 100_000_000, block_height_offset: 0 }), latest_block_hash: vec![].into(), archive_config: NextArchiveCanisterConfig { maintainers: None, max_memory_size_bytes: None, max_length: 100_000_000 }, next_block_index: 24, archived: vec![] });
    assert_swap_archive_wasm_module(default.config_swap_block_chain_query(BlockChainArgs::WasmModuleQuery).unwrap(), ARCHIVE_SWAP_WASM_MODULE.to_vec());
    assert_swap_archive_wasm_module(default.config_swap_block_chain_update(BlockChainArgs::WasmModuleUpdate(serde_bytes::ByteBuf::from(vec![1, 2, 3]))).unwrap(), ARCHIVE_SWAP_WASM_MODULE.to_vec());
    assert_swap_archive_wasm_module(default.config_swap_block_chain_query(BlockChainArgs::WasmModuleQuery).unwrap(), vec![1, 2, 3]);
    assert_swap_current_archiving_max_length(default.config_swap_block_chain_update(BlockChainArgs::CurrentArchivingMaxLengthUpdate(1000)).unwrap(), CurrentArchiving { canister_id: archive_swap_canister_id, length: 24, max_length: 1_000, block_height_offset: 0 });
    assert_swap_block_chain_view(&default.config_swap_block_chain_query(BlockChainArgs::BlockChainQuery).unwrap(), BlockChainView { current_archiving: Some(CurrentArchiving { canister_id: archive_swap_canister_id, length: 24, max_length: 1_000, block_height_offset: 0 }), latest_block_hash: vec![].into(), archive_config: NextArchiveCanisterConfig { maintainers: None, max_memory_size_bytes: None, max_length: 1_000 }, next_block_index: 24, archived: vec![] });
    assert_swap_archive_config_replace(default.config_swap_block_chain_update(BlockChainArgs::NextArchiveCanisterConfigUpdate(NextArchiveCanisterConfig{ maintainers: Some(vec![bob_identity]), max_memory_size_bytes: Some(2_000_000), max_length: 10000 })).unwrap(), NextArchiveCanisterConfig { maintainers: None, max_memory_size_bytes: None, max_length: 1_000 });
    assert_swap_block_chain_view(&default.config_swap_block_chain_query(BlockChainArgs::BlockChainQuery).unwrap(), BlockChainView { current_archiving: Some(CurrentArchiving { canister_id: archive_swap_canister_id, length: 24, max_length: 1_000, block_height_offset: 0 }), latest_block_hash: vec![].into(), archive_config: NextArchiveCanisterConfig { maintainers: Some(vec![bob_identity]), max_memory_size_bytes: Some(2_000_000), max_length: 10_000 }, next_block_index: 24, archived: vec![] });
    assert_eq!(default.config_swap_block_chain_update(BlockChainArgs::ArchivedCanisterMaintainersUpdate{ canister_id: archive_swap_canister_id, maintainers: Some(vec![alice_identity]) }).unwrap(), swap::SwapBlockResult::Ok(swap::SwapBlockResponse::ArchivedCanisterMaintainers));
    assert_eq!(default.config_swap_block_chain_update(BlockChainArgs::ArchivedCanisterMaxMemorySizeBytesUpdate{ canister_id: archive_swap_canister_id, max_memory_size_bytes: 300 }).unwrap(), swap::SwapBlockResult::Err(BusinessError::CallCanisterError(format!("call rejected: 5 - IC0503: Error from Canister {}: Canister called `ic0.trap` with message: 'Cannot set max_memory_size_bytes to 300, because it is lower than total_block_size 3919.'.\nConsider gracefully handling failures from this canister or altering the canister to handle exceptions. See documentation: https://internetcomputer.org/docs/current/references/execution-errors#trapped-explicitly", archive_swap_canister_id.to_text()))));
    assert_eq!(default.config_swap_block_chain_update(BlockChainArgs::ArchivedCanisterMaxMemorySizeBytesUpdate{ canister_id: archive_swap_canister_id, max_memory_size_bytes: 3_000_000 }).unwrap(), swap::SwapBlockResult::Ok(swap::SwapBlockResponse::ArchivedCanisterMaxMemorySizeBytes));

    // 🚩 5.1 test archive-swap
    assert_eq!(archive_swap.sender(bob_identity).get_blocks(archive_swap::GetBlocksArgs{ start: 0, length: 100 }).unwrap_err().reject_message, "Only Maintainers are allowed to query data".to_string());
    assert_eq!(alice.config_swap_block_chain_update(BlockChainArgs::ArchivedCanisterMaintainersUpdate{ canister_id: archive_swap_canister_id, maintainers: None }).unwrap_err().reject_message, "Permission 'BusinessConfigMaintaining' is required".to_string());
    assert_eq!(default.config_swap_block_chain_update(BlockChainArgs::ArchivedCanisterMaintainersUpdate{ canister_id: archive_swap_canister_id, maintainers: None }).unwrap(), swap::SwapBlockResult::Ok(swap::SwapBlockResponse::ArchivedCanisterMaintainers));
    assert_swap_get_blocks(archive_swap.sender(bob_identity).get_blocks(archive_swap::GetBlocksArgs{ start: 0, length: 100 }).unwrap(), 24);

    // 🚩 6 test request trace
    assert_eq!(bob.request_trace_index_get().unwrap_err().reject_message, "Permission 'BusinessConfigMaintaining' is required".to_string());
    assert_eq!(default.request_trace_index_get().unwrap(), (0, 19));
    assert_eq!(alice.request_trace_get(0).unwrap_err().reject_message, "Permission 'BusinessConfigMaintaining' is required".to_string());
    assert_eq!(default.request_trace_get(0).unwrap().unwrap().traces[0].1, format!("*Deposit* `token:[{}], from:({}.), to:({}.), amount:5_000_000_000_000_000_000, height:4`", token_ck_eth_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(bob.request_trace_remove(0).unwrap_err().reject_message, "Permission 'BusinessConfigMaintaining' is required".to_string());
    assert_eq!(default.request_trace_remove(10).unwrap_err().reject_message.contains("must remove min request index: 0"), true);
    assert_eq!(default.request_trace_remove(0).unwrap().unwrap().traces[0].1, format!("*Deposit* `token:[{}], from:({}.), to:({}.), amount:5_000_000_000_000_000_000, height:4`", token_ck_eth_canister_id.to_text(), default_identity.to_text(), default_identity.to_text())); // 👁︎
    assert_eq!(default.request_trace_get(0).unwrap(), None); // 👁︎
    assert_eq!(bob.request_trace_index_get().unwrap_err().reject_message, "Permission 'BusinessConfigMaintaining' is required".to_string());
    assert_eq!(default.request_trace_index_get().unwrap(), (1, 18));
    assert_eq!(default.request_trace_remove(10).unwrap_err().reject_message.contains("must remove min request index: 1"), true);

    // 🚩 7 test maintain cycles
    assert_eq!(archive_token.sender(default_identity).wallet_balance().unwrap() < nat(INIT_CYCLES as u64), true);
    assert_eq!(archive_swap.sender(default_identity).wallet_balance().unwrap() < nat(INIT_CYCLES as u64), true);
    assert_eq!(default.schedule_trigger().unwrap(), ());
    assert_eq!(archive_token.sender(default_identity).wallet_balance().unwrap() > nat(INIT_CYCLES as u64), true);
    assert_eq!(archive_swap.sender(default_identity).wallet_balance().unwrap() > nat(INIT_CYCLES as u64), true);
}

fn deploy_icrc2<'a>(
    pic: &'a PocketIc,
    sender: Principal,
    canister_id: Principal,
    symbol: &str,
    decimals: u8,
    transfer_fee: u64,
    initial_balances: Vec<(Principal, u64)>,
    fee_collector: Principal,
) -> icrc2::PocketedCanisterId<'a> {
    use icrc2::*;

    pic.create_canister_with_id(Some(sender), None, canister_id).unwrap();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        ICRC2_WASM_MODULE.to_vec(),
        encode_one(LedgerArgument::Init(InitArgs {
            token_name: symbol.to_string(),
            token_symbol: symbol.to_string(),
            decimals: Some(decimals),
            transfer_fee: Nat::from(transfer_fee),
            metadata: vec![],
            minting_account: Account {
                owner: Principal::from_text("aaaaa-aa").unwrap(),
                subaccount: None,
            },
            initial_balances: initial_balances
                .into_iter()
                .map(|(owner, balance)| {
                    (
                        Account {
                            owner,
                            subaccount: None,
                        },
                        Nat::from(balance),
                    )
                })
                .collect(),
            fee_collector_account: Some(Account {
                owner: fee_collector,
                subaccount: None,
            }),
            archive_options: ArchiveOptions {
                num_blocks_to_archive: 1000,
                max_transactions_per_response: None,
                trigger_threshold: 1000,
                more_controller_ids: None,
                max_message_size_bytes: None,
                cycles_for_archive_creation: None,
                node_max_memory_size_bytes: None,
                controller_id: Principal::from_text("aaaaa-aa").unwrap(),
            },
            max_memo_length: None,
            feature_flags: Some(FeatureFlags { icrc2: true }),
        }))
        .unwrap(),
        Some(sender),
    );

    icrc2::PocketedCanisterId::new(canister_id, &pic)
}

fn print_backtrace() {
    let backtraces = format!("{}", std::backtrace::Backtrace::force_capture());
    let backtraces = backtraces.split('\n').collect::<Vec<_>>();
    let position = backtraces.iter().position(|b| b.contains("5: ")).unwrap();
    eprintln!("{}", backtraces[position + 1]);
}

fn assert_token_block_deposit(block: archive_token::TokenBlock, deposit: archive_token::DepositToken) {
    print_backtrace();
    if let archive_token::TokenOperation::Deposit(deposit_token) = block.transaction.operation {
        return assert_eq!(deposit_token, deposit);
    }
    panic!("Expected DepositToken, got {:?}", block);
}
fn assert_token_block_withdraw(block: archive_token::TokenBlock, withdraw: archive_token::DepositToken) {
    print_backtrace();
    if let archive_token::TokenOperation::Withdraw(withdraw_token) = block.transaction.operation {
        return assert_eq!(withdraw_token, withdraw);
    }
    panic!("Expected WithdrawToken, got {:?}", block);
}
fn assert_token_block_transfer(block: archive_token::TokenBlock, transfer: archive_token::TransferToken) {
    print_backtrace();
    if let archive_token::TokenOperation::Transfer(transfer_token) = block.transaction.operation {
        return assert_eq!(transfer_token, transfer);
    }
    panic!("Expected TransferToken, got {:?}", block);
}

fn assert_tokens_balance(balances: Vec<(Principal, Nat)>, required: Vec<(Principal, Nat)>) {
    for r in required {
        if !balances.contains(&r) {
            print_backtrace();
            if let Some((p, b)) = balances.iter().find(|(p, _)| *p == r.0) {
                panic!(
                    "Expected balance ({}, {}) but got ({}, {b})",
                    r.0.to_text(),
                    r.1,
                    p.to_text(),
                );
            } else {
                panic!(
                    "Expected balance ({}, {}) but not found, got {balances:?}",
                    r.0.to_text(),
                    r.1,
                );
            }
        }
    }
}
#[allow(irrefutable_let_patterns)]
fn assert_swap_v2_market_maker(view: &swap::MarketMakerView, required: swap::SwapV2MarketMakerView) {
    print_backtrace();
    if let swap::MarketMakerView::SwapV2(s) = view {
        assert_eq!(s.subaccount, required.subaccount);
        assert_eq!(s.fee_rate, required.fee_rate);
        assert_eq!(s.token0, required.token0);
        assert_eq!(s.token1, required.token1);
        assert_eq!(s.reserve0, required.reserve0);
        assert_eq!(s.reserve1, required.reserve1);
        // assert_eq!(s.block_timestamp_last, required.block_timestamp_last);
        assert_eq!(s.price_cumulative_exponent, required.price_cumulative_exponent);
        // assert_eq!(s.price0_cumulative_last, required.price0_cumulative_last);
        // assert_eq!(s.price1_cumulative_last, required.price1_cumulative_last);
        assert_eq!(s.k_last, required.k_last);
        assert_eq!(s.lp, required.lp);
        assert_eq!(s.protocol_fee, required.protocol_fee);
    } else {
        panic!("Expected swap v2, got {view:?}");
    }
}

fn assert_swap_block_pair_create(block: archive_swap::SwapBlock, create: archive_swap::PairCreate) {
    print_backtrace();
    if let archive_swap::SwapOperation::Pair(archive_swap::PairOperation::Create(c)) = block.transaction.operation {
        return assert_eq!(c, create);
    }
    panic!("Expected PairCreate, got {:?}", block);
}
fn assert_swap_block_pair_swap(block: archive_swap::SwapBlock, swap: archive_swap::PairSwapToken) {
    print_backtrace();
    if let archive_swap::SwapOperation::Pair(archive_swap::PairOperation::Swap(s)) = block.transaction.operation {
        return assert_eq!(s, swap);
    }
    panic!("Expected PairSwapToken, got {:?}", block);
}
fn assert_swap_block_pair_v2_mint(block: archive_swap::SwapBlock, mint: archive_swap::SwapV2MintToken) {
    print_backtrace();
    if let archive_swap::SwapOperation::Pair(archive_swap::PairOperation::SwapV2(
        archive_swap::SwapV2Operation::Mint(m),
    )) = block.transaction.operation
    {
        return assert_eq!(m, mint);
    }
    panic!("Expected SwapV2MintToken, got {:?}", block);
}
fn assert_swap_block_pair_v2_state(block: archive_swap::SwapBlock, state: archive_swap::SwapV2State) {
    print_backtrace();
    if let archive_swap::SwapOperation::Pair(archive_swap::PairOperation::SwapV2(
        archive_swap::SwapV2Operation::State(s),
    )) = block.transaction.operation
    {
        assert_eq!(s.pa, state.pa);
        // assert_eq!(s.block_timestamp, state.block_timestamp);
        assert_eq!(s.supply, state.supply);
        assert_eq!(s.reserve0, state.reserve0);
        assert_eq!(s.reserve1, state.reserve1);
        assert_eq!(s.price_cumulative_exponent, state.price_cumulative_exponent);
        // assert_eq!(s.price0_cumulative, state.price0_cumulative);
        // assert_eq!(s.price1_cumulative, state.price1_cumulative);
    } else {
        panic!("Expected SwapV2State, got {:?}", block);
    }
}
fn assert_swap_block_pair_v2_burn(block: archive_swap::SwapBlock, burn: archive_swap::SwapV2BurnToken) {
    print_backtrace();
    if let archive_swap::SwapOperation::Pair(archive_swap::PairOperation::SwapV2(
        archive_swap::SwapV2Operation::Burn(b),
    )) = block.transaction.operation
    {
        return assert_eq!(b, burn);
    }
    panic!("Expected SwapV2BurnToken, got {:?}", block);
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

fn assert_token_block_chain_view(view: &swap::TokenBlockResult, required: swap::BlockChainView) {
    print_backtrace();
    if let swap::TokenBlockResult::Ok(swap::TokenBlockResponse::BlockChain(s)) = view {
        assert_eq!(s.current_archiving, required.current_archiving);
        // assert_eq!(s.latest_block_hash, required.latest_block_hash);
        assert_eq!(s.archive_config, required.archive_config);
        assert_eq!(s.next_block_index, required.next_block_index);
        assert_eq!(s.archived, required.archived);
    } else {
        panic!("Expected BlockChainView, got {view:?}");
    }
}
fn assert_token_archive_wasm_module(view: swap::TokenBlockResult, required: Vec<u8>) {
    print_backtrace();
    if let swap::TokenBlockResult::Ok(swap::TokenBlockResponse::WasmModule(s)) = view {
        return assert_eq!(s.unwrap(), serde_bytes::ByteBuf::from(required));
    }
    panic!("Expected WasmModule, got {view:?}");
}
fn assert_token_current_archiving_max_length(view: swap::TokenBlockResult, required: swap::CurrentArchiving) {
    print_backtrace();
    if let swap::TokenBlockResult::Ok(swap::TokenBlockResponse::CurrentArchivingMaxLength(s)) = view {
        return assert_eq!(s.unwrap(), required);
    }
    panic!("Expected CurrentArchiving, got {view:?}");
}
fn assert_token_archive_config_replace(view: swap::TokenBlockResult, required: swap::NextArchiveCanisterConfig) {
    print_backtrace();
    if let swap::TokenBlockResult::Ok(swap::TokenBlockResponse::NextArchiveCanisterConfig(s)) = view {
        return assert_eq!(s, required);
    }
    panic!("Expected NextArchiveCanisterConfig, got {view:?}");
}
fn assert_token_get_blocks(view: archive_token::GetTokenBlocksResult, required: usize) {
    print_backtrace();
    if let archive_token::GetTokenBlocksResult::Ok(s) = view {
        return assert_eq!(s.blocks.len(), required);
    }
    panic!("Expected GetTokenBlocksResult::Ok, got {view:?}");
}

fn assert_swap_block_chain_view(view: &swap::SwapBlockResult, required: swap::BlockChainView) {
    print_backtrace();
    if let swap::SwapBlockResult::Ok(swap::SwapBlockResponse::BlockChain(s)) = view {
        assert_eq!(s.current_archiving, required.current_archiving);
        // assert_eq!(s.latest_block_hash, required.latest_block_hash);
        assert_eq!(s.archive_config, required.archive_config);
        assert_eq!(s.next_block_index, required.next_block_index);
        assert_eq!(s.archived, required.archived);
    } else {
        panic!("Expected BlockChainView, got {view:?}");
    }
}
fn assert_swap_archive_wasm_module(view: swap::SwapBlockResult, required: Vec<u8>) {
    print_backtrace();
    if let swap::SwapBlockResult::Ok(swap::SwapBlockResponse::WasmModule(s)) = view {
        return assert_eq!(s.unwrap(), serde_bytes::ByteBuf::from(required));
    }
    panic!("Expected WasmModule, got {view:?}");
}
fn assert_swap_current_archiving_max_length(view: swap::SwapBlockResult, required: swap::CurrentArchiving) {
    print_backtrace();
    if let swap::SwapBlockResult::Ok(swap::SwapBlockResponse::CurrentArchivingMaxLength(s)) = view {
        return assert_eq!(s.unwrap(), required);
    }
    panic!("Expected CurrentArchiving, got {view:?}");
}
fn assert_swap_archive_config_replace(view: swap::SwapBlockResult, required: swap::NextArchiveCanisterConfig) {
    print_backtrace();
    if let swap::SwapBlockResult::Ok(swap::SwapBlockResponse::NextArchiveCanisterConfig(s)) = view {
        return assert_eq!(s, required);
    }
    panic!("Expected NextArchiveCanisterConfig, got {view:?}");
}
fn assert_swap_get_blocks(view: archive_swap::GetSwapBlocksResult, required: usize) {
    print_backtrace();
    if let archive_swap::GetSwapBlocksResult::Ok(s) = view {
        return assert_eq!(s.blocks.len(), required);
    }
    panic!("Expected GetSwapBlocksResult::Ok, got {view:?}");
}
