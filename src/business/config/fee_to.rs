#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== query ==============================

#[ic_cdk::update(guard = "has_business_config_fee_to")]
fn config_fee_to_query() -> Option<Account> {
    with_state(|s| s.business_config_fee_to_query().cloned())
}

// ============================== replace ==============================

#[ic_cdk::update(guard = "has_business_config_fee_to")]
fn config_fee_to_replace(fee_to: Option<Account>) -> Option<Account> {
    with_mut_state_without_record(|s| s.business_config_fee_to_replace(fee_to))
}
