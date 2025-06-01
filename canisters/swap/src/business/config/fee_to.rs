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
    let protocol_fee = protocol_fee.map(|pf| SwapRatio::new(pf.numerator, pf.denominator));
    let pa = with_state(|s| {
        s.business_token_pair_pools_query()
            .into_iter()
            .find(|(pa, _)| pa.get_subaccount() == subaccount)
            .map(|(pa, _)| pa)
    })?;
    let required = vec![pa];
    let lock = match trap(super::super::lock_token_pairs(required, 0)) {
        LockResult::Locked(lock) => lock,
        LockResult::Retry(_) => unreachable!(),
    };
    with_mut_state(|s| s.business_config_protocol_fee_replace(&lock, &pa, protocol_fee))
}
