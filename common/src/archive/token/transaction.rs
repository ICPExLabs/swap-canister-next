use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    common::{DoHash, HashOf, TimestampNanos},
    proto,
    utils::{hash::hash_sha256, pb::to_proto_bytes},
};

use super::TokenOperation;

/// 代币交易
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct TokenTransaction {
    /// 用户操作
    pub operation: TokenOperation,
    /// 用户标记，最大 32 字节
    pub memo: Option<Vec<u8>>,
    /// 用户设置的创建时间
    pub created: Option<TimestampNanos>,
}

impl TryFrom<TokenTransaction> for proto::TokenTransaction {
    type Error = candid::Error;

    fn try_from(value: TokenTransaction) -> Result<Self, Self::Error> {
        let operaction = value.operation.try_into()?;
        let memo = value.memo.map(|m| m.into());
        let created = value.created.map(|t| t.into_inner());

        Ok(Self {
            operation: Some(operaction),
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
