use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::types::CanisterId;

use super::{BlockIndex, EncodedBlock};

/// Query block
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum QueryBlockResult<T> {
    /// Contents in the block
    #[serde(rename = "block")]
    Block(T),
    /// archive canister
    #[serde(rename = "archive")]
    Archive(CanisterId),
}

/// query blocks
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct QueryBlocksResult(Vec<(BlockIndex, QueryBlockResult<EncodedBlock>)>);

impl From<Vec<(BlockIndex, QueryBlockResult<EncodedBlock>)>> for QueryBlocksResult {
    fn from(value: Vec<(BlockIndex, QueryBlockResult<EncodedBlock>)>) -> Self {
        Self(value)
    }
}
