use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};

use crate::{
    types::{BusinessError, DummyCanisterId},
    utils::{hash::hash_sha256, principal::sort_tokens},
};

use super::{Amm, AmmText};

/// 代币对
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, CandidType)]
pub struct TokenPair {
    /// small one
    pub token0: CanisterId,
    /// bigger one
    pub token1: CanisterId,
}

impl TokenPair {
    /// 构建
    pub fn new(token_a: CanisterId, token_b: CanisterId) -> Self {
        let (token0, token1) = sort_tokens(token_a, token_b);
        Self { token0, token1 }
    }
}

/// 代币对和算法

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct TokenPairAmm {
    /// 代币对
    pub pair: TokenPair,
    /// amm 算法
    pub amm: Amm,
}

impl TokenPairAmm {
    /// 获取池子的子账户
    pub fn get_subaccount(&self) -> [u8; 32] {
        let amm: AmmText = self.amm.into();
        let mut data = Vec::new();
        data.extend_from_slice(self.pair.token0.as_slice());
        data.extend_from_slice(self.pair.token1.as_slice());
        data.extend_from_slice(amm.as_ref().as_bytes());
        hash_sha256(&data)
    }

    /// 获取池子的子账户和模拟代币地址
    pub fn get_subaccount_and_dummy_canister_id(&self) -> (Subaccount, DummyCanisterId) {
        let subaccount = self.get_subaccount();
        let canister_id = CanisterId::from_slice(&subaccount[..CanisterId::MAX_LENGTH_IN_BYTES]);
        (subaccount, DummyCanisterId::new(canister_id))
    }

    /// not exist
    pub fn not_exist(&self) -> BusinessError {
        BusinessError::TokenPairAmmNotExist((self.pair, self.amm.into()))
    }
}
