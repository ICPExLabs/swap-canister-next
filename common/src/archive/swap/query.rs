use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::types::QueryBlockResult;

use super::SwapBlock;

/// query Swap Block
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct QuerySwapBlockResult(QueryBlockResult<SwapBlock>);

impl From<QueryBlockResult<SwapBlock>> for QuerySwapBlockResult {
    fn from(value: QueryBlockResult<SwapBlock>) -> Self {
        Self(value)
    }
}
