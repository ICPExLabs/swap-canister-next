use std::cmp::Ordering;

use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};

use crate::utils::{hash::hash_sha256, principal::cmp_canister_id};

use super::{Amm, AmmText};

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

    pub fn get_subaccount(&self, amm: &Amm) -> Subaccount {
        let amm: AmmText = amm.into();

        let mut data = Vec::new();
        data.extend_from_slice(self.token0.as_slice());
        data.extend_from_slice(self.token1.as_slice());
        data.extend_from_slice(amm.as_ref().as_bytes());

        hash_sha256(&data)
    }
}
