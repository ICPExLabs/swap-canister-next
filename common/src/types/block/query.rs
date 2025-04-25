use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::types::CanisterId;

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
