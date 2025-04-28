#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== query ==============================

#[ic_cdk::query]
fn config_token_custom_query() -> Vec<TokenInfo> {
    with_state(|s| s.business_config_token_custom_query())
}

// ============================== update ==============================

#[ic_cdk::update(guard = "has_business_config_maintaining")]
async fn config_token_custom_put(token: TokenInfo) {
    // preset can not modify
    if with_state(|s| s.business_config_token_preset_query().contains_key(&token.canister_id)) {
        ic_cdk::trap("can not put preset token");
    }

    // check standard
    let service = crate::services::icrc2::Service(token.canister_id);
    let (name, symbol, decimals, fee, supported) = futures::future::join5(
        service.icrc_1_name(),
        service.icrc_1_symbol(),
        service.icrc_1_decimals(),
        service.icrc_1_fee(),
        service.icrc_1_supported_standards(),
    )
    .await;
    let name = ic_canister_kit::common::trap_debug(name).0;
    let symbol = ic_canister_kit::common::trap_debug(symbol).0;
    let decimals = ic_canister_kit::common::trap_debug(decimals).0;
    let fee = ic_canister_kit::common::trap_debug(fee).0;
    let supported = ic_canister_kit::common::trap_debug(supported).0;
    if token.name != name
        || token.symbol != symbol
        || token.decimals != decimals
        || token.fee != fee
        || supported.iter().find(|s| s.name == "ICRC-2").is_none()
    {
        ic_cdk::trap("token standard not match");
    }

    // ? check controller

    let arg = ArgWithMeta::data(token);
    with_mut_state(|s| s.business_config_token_custom_put(arg))
}

#[ic_cdk::update(guard = "has_business_config_maintaining")]
async fn config_token_custom_remove(canister_id: CanisterId) -> Option<TokenInfo> {
    // preset can not modify
    if with_state(|s| s.business_config_token_preset_query().contains_key(&canister_id)) {
        ic_cdk::trap("can not put preset token");
    }

    // check balance
    let service = crate::services::icrc2::Service(canister_id);
    let balance = ic_canister_kit::common::trap_debug(
        service
            .icrc_1_balance_of(Account {
                owner: self_canister_id(),
                subaccount: None,
            })
            .await,
    )
    .0;
    if *::common::utils::math::ZERO < balance {
        ic_cdk::trap(&format!("still has balance of token: [{}]", canister_id.to_text()));
    }

    let arg = ArgWithMeta::data(canister_id);
    with_mut_state(|s| s.business_config_token_custom_remove(arg))
}
