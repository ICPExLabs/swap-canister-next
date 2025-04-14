use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::types::QueryBlockResult;

use super::TokenBlock;

/// 查询 Token Block
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct QueryTokenBlockResult(QueryBlockResult<TokenBlock>);

impl From<QueryBlockResult<TokenBlock>> for QueryTokenBlockResult {
    fn from(value: QueryBlockResult<TokenBlock>) -> Self {
        Self(value)
    }
}
