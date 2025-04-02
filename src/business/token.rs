#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// anyone can query
#[ic_cdk::query]
fn tokens_query() -> Vec<TokenInfo> {
    with_state(|s| s.business_tokens_query().values().cloned().collect())
}
