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
    with_state(|s| {
        s.business_all_tokens_query()
            .into_values()
            .map(|token| token.into_owned())
            .collect()
    })
}

// anyone can query
#[ic_cdk::query]
fn token_query(token: CanisterId) -> Option<TokenInfo> {
    with_state(|s| {
        s.business_all_tokens_query()
            .remove(&token)
            .map(|token| token.into_owned())
    })
}

// anyone can query
#[ic_cdk::query]
fn token_balance_of(token: CanisterId, account: Account) -> candid::Nat {
    crate::utils::owner::check_owner_for_token_balance_of(&account.owner); // ! must be owner or self canister
    token_balance_by(token, account)
}

// anyone can query
#[ic_cdk::query]
fn tokens_balance_of(account: Account) -> Vec<(CanisterId, candid::Nat)> {
    crate::utils::owner::check_owner_for_token_balance_of(&account.owner); // ! must be owner or self canister
    tokens_balance_by(account)
}

// anyone can query
#[ic_cdk::query(guard = "has_business_token_balance_by")]
fn token_balance_by(token: CanisterId, account: Account) -> candid::Nat {
    with_state(|s| s.business_token_balance_of(token, account))
}

// anyone can query
#[ic_cdk::query(guard = "has_business_token_balance_by")]
fn tokens_balance_by(account: Account) -> Vec<(CanisterId, candid::Nat)> {
    with_state(|s| {
        s.business_all_tokens_query()
            .keys()
            .map(|&canister_id| {
                (
                    canister_id,
                    s.business_token_balance_of(canister_id, account),
                )
            })
            .collect()
    })
}
