#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== query ==============================

#[ic_cdk::query]
fn config_token_frozen_query() -> HashSet<CanisterId> {
    with_state(|s| s.business_config_token_frozen_query().clone())
}

// ============================== update ==============================

#[ic_cdk::update(guard = "has_business_config_maintaining")]
fn config_token_frozen(token: CanisterId, frozen: bool) {
    with_mut_state(|s| s.business_config_token_frozen(token, frozen))
}
