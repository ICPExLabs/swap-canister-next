use candid::CandidType;
use common::types::{BlockIndex, BusinessError};
use serde::{Deserialize, Serialize};

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
