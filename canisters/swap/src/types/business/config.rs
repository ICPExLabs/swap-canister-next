use candid::CandidType;
use common::{
    archive::{swap::SwapBlock, token::TokenBlock},
    types::{BlockIndex, BusinessError, QueryBlockResult},
};
use ic_canister_kit::types::{CanisterId, UserId};
use serde::{Deserialize, Serialize};

use crate::types::{BlockChainView, CurrentArchiving, NextArchiveCanisterConfig};

// ========================== replace wasm module ==========================

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct ReplaceArchiveWasmModuleResult(Result<Option<Vec<u8>>, BusinessError>);

impl From<Result<Option<Vec<u8>>, BusinessError>> for ReplaceArchiveWasmModuleResult {
    fn from(value: Result<Option<Vec<u8>>, BusinessError>) -> Self {
        Self(value)
    }
}

// ========================== push blocks ==========================

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PushBlocks {
    pub block_height_start: BlockIndex,
    pub length: u64,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PushBlocksResult(Result<Option<PushBlocks>, BusinessError>);

impl From<Result<Option<PushBlocks>, BusinessError>> for PushBlocksResult {
    fn from(value: Result<Option<PushBlocks>, BusinessError>) -> Self {
        Self(value)
    }
}

// ========================== maintain archives config ==========================

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct MaintainArchivesConfig {
    /// Minimum cycles for canister trigger recharge
    pub min_cycles_threshold: u64,
    /// Number of recharges per trigger
    pub recharge_cycles: u64,
    /// Check interval ns
    pub checking_interval_ns: u64,
}

// ========================== archive config update ==========================

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum BlockChainArgs {
    // query
    BlockChainQuery,
    WasmModuleQuery,
    CachedBlockQuery,
    BlockQuery(BlockIndex),
    // update
    WasmModuleUpdate(Vec<u8>),
    CurrentArchivingMaxLengthUpdate(u64),
    NextArchiveCanisterConfigUpdate(NextArchiveCanisterConfig),
    ArchivedCanisterMaintainersUpdate {
        canister_id: CanisterId,
        maintainers: Option<Vec<UserId>>,
    },
    ArchivedCanisterMaxMemorySizeBytesUpdate {
        canister_id: CanisterId,
        max_memory_size_bytes: u64,
    },
    BlocksPush,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum BlockChainResponse<T> {
    // query
    BlockChain(BlockChainView<T>),
    WasmModule(Option<Vec<u8>>), // update
    CachedBlock(Option<(BlockIndex, u64)>),
    Block(QueryBlockResult<T>),
    // update
    // WasmModuleUpdate(Vec<u8>),
    CurrentArchivingMaxLength(Option<CurrentArchiving>),
    NextArchiveCanisterConfig(NextArchiveCanisterConfig),
    ArchivedCanisterMaintainers,
    ArchivedCanisterMaxMemorySizeBytes,
    BlocksPush(Option<PushBlocks>),
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenBlockResponse(BlockChainResponse<TokenBlock>);
impl From<BlockChainResponse<TokenBlock>> for TokenBlockResponse {
    fn from(value: BlockChainResponse<TokenBlock>) -> Self {
        Self(value)
    }
}
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenBlockResult(Result<TokenBlockResponse, BusinessError>);
impl From<Result<TokenBlockResponse, BusinessError>> for TokenBlockResult {
    fn from(value: Result<TokenBlockResponse, BusinessError>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct SwapBlockResponse(BlockChainResponse<SwapBlock>);
impl From<BlockChainResponse<SwapBlock>> for SwapBlockResponse {
    fn from(value: BlockChainResponse<SwapBlock>) -> Self {
        Self(value)
    }
}
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct SwapBlockResult(Result<SwapBlockResponse, BusinessError>);
impl From<Result<SwapBlockResponse, BusinessError>> for SwapBlockResult {
    fn from(value: Result<SwapBlockResponse, BusinessError>) -> Self {
        Self(value)
    }
}
