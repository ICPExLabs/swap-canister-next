use candid::CandidType;
use common::types::{BlockIndex, BusinessError};
use serde::{Deserialize, Serialize};

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
