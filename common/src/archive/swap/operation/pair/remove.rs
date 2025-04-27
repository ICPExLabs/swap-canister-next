use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{TokenPairAmm, UserId},
};

/// Remove a pool
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct PairRemove {
    /// Token pairs and algorithms
    pub pa: TokenPairAmm,
    /// Remover
    pub remover: UserId,
}

impl From<PairRemove> for proto::PairRemove {
    fn from(value: PairRemove) -> Self {
        let pa = value.pa.into();
        let remover = value.remover.into();

        Self {
            pa: Some(pa),
            remover: Some(remover),
        }
    }
}

impl TryFrom<proto::PairRemove> for PairRemove {
    type Error = String;

    fn try_from(value: proto::PairRemove) -> Result<Self, Self::Error> {
        let pa = value
            .pa
            .ok_or_else(|| "pa of pair remove can not be none".to_string())?
            .try_into()?;
        let remover = value
            .remover
            .ok_or_else(|| "remover of pair remove can not be none".to_string())?
            .into();

        Ok(Self { pa, remover })
    }
}
