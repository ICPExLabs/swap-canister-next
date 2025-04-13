use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{proto, types::TokenPairAmm};

/// 创建池子

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct PairCreate {
    /// 代币对和算法
    pub pair_amm: TokenPairAmm,
}

impl From<PairCreate> for proto::PairCreate {
    fn from(value: PairCreate) -> Self {
        let pair_amm = value.pair_amm.into();

        Self {
            pair_amm: Some(pair_amm),
        }
    }
}

impl TryFrom<proto::PairCreate> for PairCreate {
    type Error = String;

    fn try_from(value: proto::PairCreate) -> Result<Self, Self::Error> {
        let pair_amm = value
            .pair_amm
            .ok_or_else(|| "pair_amm of pair create can not be none".to_string())?
            .try_into()?;

        Ok(Self { pair_amm })
    }
}
