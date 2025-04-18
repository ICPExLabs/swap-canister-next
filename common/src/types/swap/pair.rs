use std::{borrow::Cow, fmt::Display};

use candid::CandidType;
use ic_canister_kit::types::{Bound, CanisterId, Storable};
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};

use crate::{
    types::{BusinessError, DummyCanisterId},
    utils::{hash::hash_sha256, principal::sort_tokens},
};

use super::{Amm, AmmText};

/// Sequential token pairs
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, CandidType,
)]
pub struct TokenPair {
    /// small one
    pub token0: CanisterId,
    /// bigger one
    pub token1: CanisterId,
}

impl TokenPair {
    /// new
    pub fn new(token_a: CanisterId, token_b: CanisterId) -> Self {
        let (token0, token1) = sort_tokens(token_a, token_b);
        Self { token0, token1 }
    }
}

/// Sequential token pairs and algorithms
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, CandidType,
)]
pub struct TokenPairAmm {
    /// Token pairs
    pub pair: TokenPair,
    /// amm algorithm
    pub amm: Amm,
}

impl Storable for TokenPairAmm {
    fn to_bytes(&self) -> Cow<[u8]> {
        use ic_canister_kit::common::trap;
        Cow::Owned(trap(ic_canister_kit::functions::stable::to_bytes(self)))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        use ic_canister_kit::common::trap;
        trap(ic_canister_kit::functions::stable::from_bytes(&bytes))
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl TokenPairAmm {
    /// Get the sub-account of the pool
    pub fn get_subaccount(&self) -> [u8; 32] {
        let amm: AmmText = self.amm.into();
        let mut data = Vec::new();
        data.extend_from_slice(self.pair.token0.as_slice());
        data.extend_from_slice(self.pair.token1.as_slice());
        data.extend_from_slice(amm.as_ref().as_bytes());
        hash_sha256(&data)
    }

    /// Get the sub-account and simulated token address of the pool
    pub fn get_subaccount_and_dummy_canister_id(&self) -> (Subaccount, DummyCanisterId) {
        let subaccount = self.get_subaccount();
        let canister_id = CanisterId::from_slice(&subaccount[..CanisterId::MAX_LENGTH_IN_BYTES]);
        (subaccount, DummyCanisterId::new(canister_id))
    }

    /// not exist
    pub fn not_exist(&self) -> BusinessError {
        BusinessError::TokenPairAmmNotExist(*self)
    }
}

impl Display for TokenPairAmm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}],[{}],{}",
            self.pair.token0.to_text(),
            self.pair.token1.to_text(),
            self.amm.into_text().as_ref()
        )
    }
}
