use std::cmp::Ordering;

use ic_canister_kit::types::CanisterId;
use num_bigint::BigUint;

fn cmp_canister_id(canister_id1: &CanisterId, canister_id2: &CanisterId) -> Ordering {
    let n1 = BigUint::from_bytes_be(canister_id1.as_slice());
    let n2 = BigUint::from_bytes_be(canister_id2.as_slice());
    n1.cmp(&n2)
}

// returns sorted token addresses, used to handle return values from pairs sorted in this order
pub fn sort_tokens(token_a: CanisterId, token_b: CanisterId) -> (CanisterId, CanisterId) {
    if matches!(cmp_canister_id(&token_a, &token_b), Ordering::Less) {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    }
}
