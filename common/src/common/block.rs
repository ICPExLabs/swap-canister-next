use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::proto;

use super::{BlockIndex, HashOf, timestamp::TimestampNanos};

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

/// 块
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct CandidBlock<T> {
    /// 前置 hash
    pub parent_hash: HashOf<T>,
    /// 时间戳
    pub timestamp: TimestampNanos,
    /// 交易内容
    pub transaction: T,
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
