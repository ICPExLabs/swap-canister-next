#[cfg(feature = "cdk")]
use std::borrow::Cow;

use candid::CandidType;
#[cfg(feature = "cdk")]
use ic_canister_kit::types::{Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::{proto, types::TimestampNanos, utils::hash::hash_sha256};

use super::{BlockIndex, DoHash, HashOf};

// ========================== Block ==========================

/// Encoded blocks
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct EncodedBlock(pub Vec<u8>);

impl From<EncodedBlock> for proto::EncodedBlock {
    fn from(value: EncodedBlock) -> Self {
        proto::EncodedBlock { block: value.0.into() }
    }
}
impl From<proto::EncodedBlock> for EncodedBlock {
    fn from(value: proto::EncodedBlock) -> Self {
        EncodedBlock(value.block.into())
    }
}
impl From<Vec<u8>> for EncodedBlock {
    fn from(value: Vec<u8>) -> Self {
        EncodedBlock(value)
    }
}

#[cfg(feature = "cdk")]
impl Storable for EncodedBlock {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Borrowed(&self.0)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(bytes.to_vec())
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// block
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct CandidBlock<B, T> {
    /// parent hash
    pub parent_hash: HashOf<B>,
    /// Timestamp
    pub timestamp: TimestampNanos,
    /// Transaction content
    pub transaction: T,
}

impl<B, T: DoHash> CandidBlock<B, T> {
    /// hash_without_parent_hash
    pub fn hash_without_parent_hash(&self) -> Result<HashOf<CandidBlock<B, T>>, String> {
        let mut bytes = Vec::with_capacity(8 + 32);
        bytes.extend(self.timestamp.into_inner().to_le_bytes());
        bytes.extend(self.transaction.do_hash()?.as_slice());
        let hash = hash_sha256(&bytes);
        Ok(HashOf::new(hash))
    }
}

// ========================== query ==========================

/// query blocks
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct GetBlocksArgs {
    /// start
    pub start: BlockIndex,
    /// length
    pub length: u64,
}

/// Query block error
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum GetBlocksError {
    /// The error starts
    BadFirstBlockIndex {
        /// The first block requested
        requested_index: BlockIndex,
        /// The first block that actually works
        first_valid_index: BlockIndex,
    },
    /// Other errors
    Other {
        /// Error code
        error_code: u64,
        /// Error message
        error_message: String,
    },
}

/// Encoded results
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct GetEncodedBlocksResult(Result<Vec<EncodedBlock>, GetBlocksError>);

impl From<Result<Vec<EncodedBlock>, GetBlocksError>> for GetEncodedBlocksResult {
    fn from(value: Result<Vec<EncodedBlock>, GetBlocksError>) -> Self {
        GetEncodedBlocksResult(value)
    }
}
