use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

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
