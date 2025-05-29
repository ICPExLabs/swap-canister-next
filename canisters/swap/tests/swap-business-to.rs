//! https://github.com/dfinity/pocketic

use candid::{Nat, Principal, encode_one};
use ic_cdk::management_canister::CanisterSettings;
use pocket_ic::PocketIc;

mod archive_swap;
mod archive_token;
mod icrc2;
mod swap;

// 2T cycles
const INIT_CYCLES: u128 = 2_000_000_000_000;

const WASM_MODULE: &[u8] = include_bytes!("../sources/source_opt.wasm.gz");
const ICRC2_WASM_MODULE: &[u8] = include_bytes!("../../../ledger/ic-icrc1-ledger.wasm");
const ARCHIVE_TOKEN_WASM_MODULE: &[u8] = include_bytes!("../../archive-token/sources/source_opt.wasm.gz");
const ARCHIVE_SWAP_WASM_MODULE: &[u8] = include_bytes!("../../archive-swap/sources/source_opt.wasm.gz");

#[ignore]
#[test]
#[rustfmt::skip]
fn test_swap_business_to_apis() {
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
    fn nat(value: u64) -> candid::Nat {
        Nat::from(value)
    }
    fn principal(text: &str) -> Principal {
        Principal::from_text(text).unwrap()
    }

    pic.create_canister_with_id(Some(default_identity), None, canister_id).unwrap();
    pic.add_cycles(canister_id, 20_000_000_000_000);
    pic.install_canister(canister_id, WASM_MODULE.to_vec(), encode_one(Some(InitArgs::V1(InitArgV1 { maintainers: None, schedule: None, current_archiving_token: Some(CurrentArchiving { canister_id: archive_token_canister_id, length: 0, max_length: 100_000_000, block_height_offset: 0 }), current_archiving_swap: Some(CurrentArchiving { canister_id: archive_swap_canister_id, length: 0, max_length: 100_000_000, block_height_offset: 0 }) }))).unwrap(), Some(default_identity));

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
    assert_eq!(default.config_fee_to_replace(FeeTo { token_fee_to: Some(account(bob_identity)), swap_fee_to: Some(account(bob_identity)) }).unwrap(), FeeTo { token_fee_to: None, swap_fee_to: None });
    assert_eq!(default.config_fee_to_query().unwrap(), FeeTo { token_fee_to: Some(account(bob_identity)), swap_fee_to: Some(account(bob_identity)) });
    assert_eq!(default.config_fee_to_replace(FeeTo { token_fee_to: None, swap_fee_to: None }).unwrap(), FeeTo { token_fee_to: Some(account(bob_identity)), swap_fee_to: Some(account(bob_identity)) });
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
    assert_eq!(token_ck_eth.sender(default_identity).icrc2_approve(icrc2::ApproveArgs::new(icrc2_account(canister_id), nat(10_000_000_000_000_000_000))).unwrap(), icrc2::Result2::Ok(nat(2)));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(9_999_998_000_000_000_000));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(canister_id)).unwrap(), nat(0));
    assert_tokens_balance(alice.tokens_balance_of(account(alice_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(0))]);
    assert_eq!(default.token_deposit(TokenDepositArgs { token: token_ck_eth_canister_id, from: account(default_identity), deposit_amount_without_fee: nat(5_000_000_000_000_000_000), to: account(alice_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(3)));
    pic.tick(); // 🕰︎
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(4_999_996_000_000_000_000));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(canister_id)).unwrap(), nat(5_000_000_000_000_000_000));
    assert_tokens_balance(alice.tokens_balance_of(account(alice_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(5_000_000_000_000_000_000))]);

    // 🚩 1.2 business tokens withdraw
    assert_eq!(alice.token_withdraw(TokenWithdrawArgs { token: token_ck_eth_canister_id, from: account(alice_identity), withdraw_amount_without_fee: nat(999_998_000_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(4)));
    pic.tick(); // 🕰︎
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(default_identity)).unwrap(), nat(5_999_994_000_000_000_000));
    assert_eq!(token_ck_eth.sender(default_identity).icrc1_balance_of(icrc2_account(canister_id)).unwrap(), nat(4_000_000_000_000_000_000));
    assert_tokens_balance(alice.tokens_balance_of(account(alice_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(4_000_000_000_000_000_000))]);

    // 🚩 1.3 business tokens transfer
    assert_tokens_balance(  alice.tokens_balance_of(account(  alice_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(4_000_000_000_000_000_000))]);
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(0))]);
    assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(0))]);
    assert_eq!(alice.token_transfer(TokenTransferArgs { token: token_ck_eth_canister_id, from: account(alice_identity), transfer_amount_without_fee: nat(2_000_000_000_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(2_000_000_000_000_000_000)));
    pic.tick(); // 🕰︎
    assert_tokens_balance(  alice.tokens_balance_of(account(  alice_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(2_000_000_000_000_000_000))]);
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(2_000_000_000_000_000_000))]);
    assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(0))]);

    // 🚩 1.4 create liquidity
    let token_ck_eth_token_ck_usdt_dummy_canister_id = principal("vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4");
    let token_ck_eth_token_ck_usdt_subaccount = hex::decode("11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c").unwrap();
    assert_eq!(default.pair_create(TokenPairCreateOrRemoveArgs { pool: TokenPairPool { token0: token_ck_eth_canister_id, token1: token_ck_usdt_canister_id, amm: "swap_v2_0.3%".to_string() }, memo: None, created: None }).unwrap(), TokenPairCreateOrRemoveResult::Ok(MarketMakerView::SwapV2(SwapV2MarketMakerView { lp: PoolLpView::Inner(InnerLpView { fee: "100_000_000".to_string(), decimals: 12, dummy_canister_id: token_ck_eth_token_ck_usdt_dummy_canister_id.to_text(), minimum_liquidity: "100_000_000_000".to_string(), total_supply: "0".to_string() }), price_cumulative_exponent: 64, block_timestamp_last: 0, reserve0: "0".to_string(), reserve1: "0".to_string(), subaccount: hex::encode(&token_ck_eth_token_ck_usdt_subaccount), price1_cumulative_last: "0".to_string(), token0: token_ck_eth_canister_id.to_text(), token1: token_ck_usdt_canister_id.to_text(), fee_rate: "3/1000".to_string(), k_last: "0".to_string(), protocol_fee: None, price0_cumulative_last: "0".to_string() })));
    assert_eq!(token_ck_usdt.sender(default_identity).icrc2_approve(icrc2::ApproveArgs::new(icrc2_account(canister_id), nat(900_000_000_000))).unwrap(), icrc2::Result2::Ok(nat(1)));
    assert_eq!(default.token_deposit(TokenDepositArgs { token: token_ck_usdt_canister_id, from: account(default_identity), deposit_amount_without_fee: nat(800_000_000_000), to: account(default_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(2)));
    pic.tick(); // 🕰︎
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(2_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(800_000_000_000))]);
    assert_eq!(default.pair_liquidity_add(TokenPairLiquidityAddArgs { swap_pair: SwapTokenPair { token: (token_ck_eth_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_0.3%".to_string() }, from: account(default_identity), to: account(default_identity), amount_desired: (nat(2_000_000_000_000_000_000), nat(400_000_000_000)), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityAddResult::Ok(TokenPairLiquidityAddSuccess { liquidity: nat(894_427_190_999_915), amount: (nat(2_000_000_000_000_000_000), nat(400_000_000_000)) }));
    pic.tick(); // 🕰︎
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(0)), (token_ck_usdt_canister_id, nat(400_000_000_000)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(894_427_190_999_915))]);
    assert_tokens_balance(  alice.tokens_balance_of(account(  alice_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(2_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(0)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);

    // 🚩 1.5 transfer liquidity
    assert_eq!(default.token_transfer(TokenTransferArgs { token: token_ck_eth_token_ck_usdt_dummy_canister_id, from: account(default_identity), transfer_amount_without_fee: nat(894_427_190_999_915), to: account(alice_identity), fee: None, created: None, memo: None }, None).unwrap(), TokenChangedResult::Ok(nat(894_427_190_999_915)));
    assert_tokens_balance(  bob.tokens_balance_of(account(  bob_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(0)), (token_ck_usdt_canister_id, nat(0)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);
    pic.tick(); // 🕰︎
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(0)), (token_ck_usdt_canister_id, nat(400_000_000_000)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_tokens_balance(  alice.tokens_balance_of(account(  alice_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(2_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(0)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(894_427_190_999_915))]);
    assert_tokens_balance(  bob.tokens_balance_of(account(  bob_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(0)), (token_ck_usdt_canister_id, nat(0)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);

    // 🚩 1.6 remove liquidity
    assert_tokens_balance(alice.tokens_balance_of(Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_ck_eth_canister_id, nat(2_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(400_000_000_000))]);
    assert_eq!(alice.pair_liquidity_remove(TokenPairLiquidityRemoveArgs { swap_pair: SwapTokenPair { token: (token_ck_eth_canister_id, token_ck_usdt_canister_id), amm: "swap_v2_0.3%".to_string() }, from: account(alice_identity), to: account(default_identity), liquidity_without_fee: nat(894_427_190_999_915), amount_min: (nat(1), nat(1)), deadline:None, created: None, memo: None}, None).unwrap(), TokenPairLiquidityRemoveResult::Ok(TokenPairLiquidityRemoveSuccess { amount: (nat(2_000_000_000_000_000_000), nat(400_000_000_000)) }));
    pic.tick(); // 🕰︎
    assert_tokens_balance(default.tokens_balance_of(account(default_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(2_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(800_000_000_000)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_tokens_balance(  alice.tokens_balance_of(account(  alice_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(2_000_000_000_000_000_000)), (token_ck_usdt_canister_id, nat(0)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_tokens_balance(    bob.tokens_balance_of(account(    bob_identity)).unwrap(), vec![(token_ck_eth_canister_id, nat(0)), (token_ck_usdt_canister_id, nat(0)), (token_ck_eth_token_ck_usdt_dummy_canister_id, nat(0))]);
    assert_tokens_balance(alice.tokens_balance_of(Account { owner: canister_id, subaccount: Some(token_ck_eth_token_ck_usdt_subaccount.clone().into()) }).unwrap(), vec![(token_ck_eth_canister_id, nat(0)), (token_ck_usdt_canister_id, nat(0))]);
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
