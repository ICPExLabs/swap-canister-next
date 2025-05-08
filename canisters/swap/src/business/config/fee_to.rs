#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== query ==============================

#[ic_cdk::query]
fn config_fee_to_view_query() -> FeeToView {
    with_state(|s| s.business_config_fee_to_query()).into()
}

#[ic_cdk::query(guard = "has_business_config_fee_to")]
fn config_fee_to_query() -> FeeTo {
    with_state(|s| s.business_config_fee_to_query())
}

// ============================== replace ==============================

#[ic_cdk::update(guard = "has_business_config_fee_to")]
fn config_fee_to_replace(fee_to: FeeTo) -> FeeTo {
    with_mut_state(|s| s.business_config_fee_to_replace(fee_to))
}

#[ic_cdk::update(guard = "has_business_config_fee_to")]
fn config_protocol_fee_replace(subaccount: Subaccount, protocol_fee: Option<SwapRatio>) -> Option<SwapRatio> {
    with_mut_state(|s| s.business_config_protocol_fee_replace(subaccount, protocol_fee))
}
