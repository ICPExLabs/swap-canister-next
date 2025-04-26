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

    // TODO check controller
    with_mut_state(|s| s.business_config_token_custom_put(token))
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

    // TODO check controller
    with_mut_state(|s| s.business_config_token_custom_remove(&canister_id))
}
