use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{DoHash, HashOf, TimestampNanos},
    utils::{hash::hash_sha256, pb::to_proto_bytes},
};

use super::TokenOperation;

/// Token trading
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct TokenTransaction {
    /// User Operation
    pub operation: TokenOperation,
    /// User tag, up to 32 bytes
    pub memo: Option<Vec<u8>>,
    /// Created time set by user
    pub created: Option<TimestampNanos>,
}

impl TryFrom<TokenTransaction> for proto::TokenTransaction {
    type Error = candid::Error;

    fn try_from(value: TokenTransaction) -> Result<Self, Self::Error> {
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

impl TryFrom<proto::TokenTransaction> for TokenTransaction {
    type Error = String;

    fn try_from(value: proto::TokenTransaction) -> Result<Self, Self::Error> {
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

impl DoHash for TokenTransaction {
    fn do_hash(&self) -> Result<HashOf<TokenTransaction>, String> {
        let transaction: proto::TokenTransaction =
            self.clone().try_into().map_err(|err| format!("{err:?}"))?;
        let bytes = to_proto_bytes(&transaction)?;
        let hash = hash_sha256(&bytes);
        Ok(HashOf::new(hash))
    }
}
