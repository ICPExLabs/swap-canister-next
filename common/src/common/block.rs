use std::borrow::Cow;

use candid::CandidType;
use ic_canister_kit::types::{Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::{proto, utils::hash::hash_sha256};

use super::{BlockIndex, DoHash, HashOf, timestamp::TimestampNanos};

// ========================== Block ==========================

/// 编码后的快
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct EncodedBlock(pub Vec<u8>);

impl From<EncodedBlock> for proto::EncodedBlock {
    fn from(value: EncodedBlock) -> Self {
        proto::EncodedBlock {
            block: value.0.into(),
        }
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

impl Storable for EncodedBlock {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Borrowed(&self.0)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(bytes.to_vec())
    }

    const BOUND: Bound = Bound::Unbounded;
}

/// 块
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct CandidBlock<B, T> {
    /// 前置 hash
    pub parent_hash: HashOf<B>,
    /// 时间戳
    pub timestamp: TimestampNanos,
    /// 交易内容
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

// ========================== 查询 ==========================

/// 查询块
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct GetBlocksArgs {
    /// 起始
    pub start: BlockIndex,
    /// 数量
    pub length: u64,
}

/// 查询块错误
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum GetBlocksError {
    /// 错误起始
    BadFirstBlockIndex {
        /// 请求的第一个块
        requested_index: BlockIndex,
        /// 实际有效的第一个块
        first_valid_index: BlockIndex,
    },
    /// 其他错误
    Other {
        /// 错误码
        error_code: u64,
        /// 错误消息
        error_message: String,
    },
}

/// 编码后的结果
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct GetEncodedBlocksResult(Result<Vec<EncodedBlock>, GetBlocksError>);

impl From<Result<Vec<EncodedBlock>, GetBlocksError>> for GetEncodedBlocksResult {
    fn from(value: Result<Vec<EncodedBlock>, GetBlocksError>) -> Self {
        GetEncodedBlocksResult(value)
    }
}
