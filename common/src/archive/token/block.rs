use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{CandidBlock, DoHash, EncodedBlock, GetBlocksError, HashOf, TimestampNanos},
    utils::{
        hash::hash_sha256,
        pb::{from_proto_bytes, to_proto_bytes},
    },
};

use super::transaction::TokenTransaction;

/// Token blocks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct TokenBlock(pub CandidBlock<TokenBlock, TokenTransaction>);

impl TokenBlock {
    /// parent hash
    pub fn get_parent_hash(&self) -> HashOf<TokenBlock> {
        self.0.parent_hash
    }
}

impl DoHash for TokenBlock {
    fn do_hash(&self) -> Result<HashOf<TokenBlock>, String> {
        let mut bytes = Vec::with_capacity(32 + 32);
        bytes.extend(self.0.parent_hash.as_slice());
        bytes.extend(self.0.hash_without_parent_hash()?.as_slice());
        let hash = hash_sha256(&bytes);
        Ok(HashOf::new(hash))
    }
}

/// Multiple blocks
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenBlockRange {
    /// block
    pub blocks: Vec<TokenBlock>,
}

/// Query block results
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct GetTokenBlocksResult(Result<TokenBlockRange, GetBlocksError>);

impl From<Result<TokenBlockRange, GetBlocksError>> for GetTokenBlocksResult {
    fn from(value: Result<TokenBlockRange, GetBlocksError>) -> Self {
        Self(value)
    }
}

impl TryFrom<TokenBlock> for proto::TokenBlock {
    type Error = candid::Error;

    fn try_from(value: TokenBlock) -> Result<Self, Self::Error> {
        let parent_hash = value.0.parent_hash.into();
        let timestamp = value.0.timestamp.into_inner();
        let transaction = value.0.transaction.try_into()?;
        Ok(Self {
            parent_hash: Some(parent_hash),
            timestamp,
            transaction: Some(transaction),
        })
    }
}

impl TryFrom<proto::TokenBlock> for TokenBlock {
    type Error = String;
    fn try_from(value: proto::TokenBlock) -> Result<Self, Self::Error> {
        let parent_hash = value
            .parent_hash
            .ok_or_else(|| "parent_hash of token block can not be none".to_string())?
            .try_into()?;
        let timestamp = TimestampNanos::from_inner(value.timestamp);
        let transaction = value
            .transaction
            .ok_or_else(|| "transaction of token block can not be none".to_string())?
            .try_into()?;
        Ok(Self(CandidBlock {
            parent_hash,
            timestamp,
            transaction,
        }))
    }
}

impl TryFrom<TokenBlock> for EncodedBlock {
    type Error = String;

    fn try_from(value: TokenBlock) -> Result<Self, Self::Error> {
        let block: Result<proto::TokenBlock, _> = value.try_into();
        let block = block.map_err(|err| err.to_string())?;
        let bytes = to_proto_bytes(&block)?;
        Ok(Self(bytes))
    }
}

impl TryFrom<EncodedBlock> for TokenBlock {
    type Error = String;
    fn try_from(value: EncodedBlock) -> Result<Self, Self::Error> {
        let bytes = value.0;
        let block: proto::TokenBlock = from_proto_bytes(&bytes)?;
        block.try_into()
    }
}
