use std::cmp::Ordering;

use ic_canister_kit::types::CanisterId;
use num_bigint::BigUint;

pub fn cmp_canister_id(canister_id1: &CanisterId, canister_id2: &CanisterId) -> Ordering {
    let n1 = BigUint::from_bytes_be(canister_id1.as_slice());
    let n2 = BigUint::from_bytes_be(canister_id2.as_slice());
    n1.cmp(&n2)
}
