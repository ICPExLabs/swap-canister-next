use std::cmp::Ordering;

use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

use crate::utils::principal::cmp_canister_id;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, CandidType)]
pub struct TokenPair {
    pub token0: CanisterId, // id is small
    pub token1: CanisterId, // id is bigger
}

impl TokenPair {
    pub fn new(token0: CanisterId, token1: CanisterId) -> Self {
        let (token0, token1) = if matches!(cmp_canister_id(&token0, &token1), Ordering::Less) {
            (token0, token1)
        } else {
            (token1, token0)
        };
        Self { token0, token1 }
    }
}
