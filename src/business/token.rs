#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ========================== query token and balance ==========================

// anyone can query
#[ic_cdk::query]
fn tokens_query() -> Vec<TokenInfo> {
    with_state(|s| s.business_tokens_query().values().cloned().collect())
}

// anyone can query
#[ic_cdk::query]
fn token_query(canister_id: CanisterId) -> Option<TokenInfo> {
    with_state(|s| s.business_tokens_query().get(&canister_id).cloned())
}

// anyone can query
#[ic_cdk::query]
fn token_balance_of(canister_id: CanisterId, account: Account) -> candid::Nat {
    with_state(|s| s.business_token_balance_of(canister_id, account))
}

// anyone can query
#[ic_cdk::query]
fn tokens_balance_of(account: Account) -> Vec<(CanisterId, candid::Nat)> {
    with_state(|s| {
        s.business_tokens_query()
            .keys()
            .cloned()
            .map(|canister_id| {
                (
                    canister_id,
                    s.business_token_balance_of(canister_id, account),
                )
            })
            .collect()
    })
}
