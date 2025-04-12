use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};

use crate::utils::principal::sort_tokens;

use super::{Amm, AmmText, BusinessError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, CandidType)]
pub struct DummyCanisterId(CanisterId);
impl DummyCanisterId {
    pub fn id(&self) -> CanisterId {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, CandidType)]
pub struct TokenPair {
    pub token0: CanisterId, // id is small
    pub token1: CanisterId, // id is bigger
}

impl TokenPair {
    pub fn new(token_a: CanisterId, token_b: CanisterId) -> Self {
        let (token0, token1) = sort_tokens(token_a, token_b);
        Self { token0, token1 }
    }
}

pub struct PairAmm {
    pub pair: TokenPair,
    pub amm: Amm,
}

impl PairAmm {
    pub fn get_subaccount(&self) -> [u8; 32] {
        let amm: AmmText = (&self.amm).into();
        let mut data = Vec::new();
        data.extend_from_slice(self.pair.token0.as_slice());
        data.extend_from_slice(self.pair.token1.as_slice());
        data.extend_from_slice(amm.as_ref().as_bytes());
        common::utils::hash::hash_sha256(&data)
    }
    pub fn get_subaccount_and_dummy_canister_id(&self) -> (Subaccount, DummyCanisterId) {
        let subaccount = self.get_subaccount();
        let canister_id = CanisterId::from_slice(&subaccount[..CanisterId::MAX_LENGTH_IN_BYTES]);
        (subaccount, DummyCanisterId(canister_id))
    }

    pub fn not_exist(&self) -> BusinessError {
        BusinessError::TokenPairAmmNotExist((self.pair, (&self.amm).into()))
    }
}
