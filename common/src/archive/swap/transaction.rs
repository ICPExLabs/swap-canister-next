use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{DoHash, HashOf, TimestampNanos},
    utils::{hash::hash_sha256, pb::to_proto_bytes},
};

use super::SwapOperation;

/// Token trading
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct SwapTransaction {
    /// User Operation
    pub operation: SwapOperation,
    /// User tag, up to 32 bytes
    pub memo: Option<Vec<u8>>,
    /// Created time set by user
    pub created: Option<TimestampNanos>,
}

impl TryFrom<SwapTransaction> for proto::SwapTransaction {
    type Error = candid::Error;

    fn try_from(value: SwapTransaction) -> Result<Self, Self::Error> {
        let operation = value.operation.try_into()?;
        let memo = value.memo.map(|m| m.into());
        let created = value.created.map(|t| t.into_inner());

        Ok(Self {
            operation: Some(operation),
            memo,
            created,
        })
    }
}

impl TryFrom<proto::SwapTransaction> for SwapTransaction {
    type Error = String;

    fn try_from(value: proto::SwapTransaction) -> Result<Self, Self::Error> {
        let operation = value
            .operation
            .ok_or_else(|| "operation of transaction can not be none".to_string())?
            .try_into()?;
        let memo = value.memo.map(|m| m.into());
        let created = value.created.map(TimestampNanos::from_inner);

        Ok(Self {
            operation,
            memo,
            created,
        })
    }
}

impl DoHash for SwapTransaction {
    fn do_hash(&self) -> Result<HashOf<SwapTransaction>, String> {
        let transaction: proto::SwapTransaction =
            self.clone().try_into().map_err(|err| format!("{err:?}"))?;
        let bytes = to_proto_bytes(&transaction)?;
        let hash = hash_sha256(&bytes);
        Ok(HashOf::new(hash))
    }
}
