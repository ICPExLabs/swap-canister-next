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

use super::transaction::SwapTransaction;

/// Token blocks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct SwapBlock(pub CandidBlock<SwapBlock, SwapTransaction>);

impl SwapBlock {
    /// parent hash
    pub fn get_parent_hash(&self) -> HashOf<SwapBlock> {
        self.0.parent_hash
    }
}

impl DoHash for SwapBlock {
    fn do_hash(&self) -> Result<HashOf<SwapBlock>, String> {
        let mut bytes = Vec::with_capacity(32 + 32);
        bytes.extend(self.0.parent_hash.as_slice());
        bytes.extend(self.0.hash_without_parent_hash()?.as_slice());
        let hash = hash_sha256(&bytes);
        Ok(HashOf::new(hash))
    }
}

/// Multiple blocks
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SwapBlockRange {
    /// piece
    pub blocks: Vec<SwapBlock>,
}

/// Query block results
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct GetSwapBlocksResult(Result<SwapBlockRange, GetBlocksError>);

impl From<Result<SwapBlockRange, GetBlocksError>> for GetSwapBlocksResult {
    fn from(value: Result<SwapBlockRange, GetBlocksError>) -> Self {
        Self(value)
    }
}

impl TryFrom<SwapBlock> for proto::SwapBlock {
    type Error = candid::Error;

    fn try_from(value: SwapBlock) -> Result<Self, Self::Error> {
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

impl TryFrom<proto::SwapBlock> for SwapBlock {
    type Error = String;
    fn try_from(value: proto::SwapBlock) -> Result<Self, Self::Error> {
        let parent_hash = value
            .parent_hash
            .ok_or_else(|| "parent_hash of swap block can not be none".to_string())?
            .try_into()?;
        let timestamp = TimestampNanos::from_inner(value.timestamp);
        let transaction = value
            .transaction
            .ok_or_else(|| "transaction of swap block can not be none".to_string())?
            .try_into()?;
        Ok(Self(CandidBlock {
            parent_hash,
            timestamp,
            transaction,
        }))
    }
}

impl TryFrom<SwapBlock> for EncodedBlock {
    type Error = String;

    fn try_from(value: SwapBlock) -> Result<Self, Self::Error> {
        let block: Result<proto::SwapBlock, _> = value.try_into();
        let block = block.map_err(|err| err.to_string())?;
        let bytes = to_proto_bytes(&block)?;
        Ok(Self(bytes))
    }
}

impl TryFrom<EncodedBlock> for SwapBlock {
    type Error = String;
    fn try_from(value: EncodedBlock) -> Result<Self, Self::Error> {
        let bytes = value.0;
        let block: proto::SwapBlock = from_proto_bytes(&bytes)?;
        block.try_into()
    }
}
