use candid::CandidType;
use ic_canister_kit::types::UserId;
use serde::{Deserialize, Serialize};

use crate::{proto, types::TokenPairAmm};

/// Create a pool
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct PairCreate {
    /// Token pairs and algorithms
    pub pa: TokenPairAmm,
    /// Creator
    pub creator: UserId,
}

impl From<PairCreate> for proto::PairCreate {
    fn from(value: PairCreate) -> Self {
        let pa = value.pa.into();
        let creator = value.creator.into();

        Self {
            pa: Some(pa),
            creator: Some(creator),
        }
    }
}

impl TryFrom<proto::PairCreate> for PairCreate {
    type Error = String;

    fn try_from(value: proto::PairCreate) -> Result<Self, Self::Error> {
        let pa = value
            .pa
            .ok_or_else(|| "pa of pair create can not be none".to_string())?
            .try_into()?;
        let creator = value
            .creator
            .ok_or_else(|| "creator of pair create can not be none".to_string())?
            .into();

        Ok(Self { pa, creator })
    }
}
