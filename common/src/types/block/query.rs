use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

/// 查询块
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum QueryBlockResult<T> {
    /// 块内内容
    #[serde(rename = "block")]
    Block(T),
    /// 罐子
    #[serde(rename = "archive")]
    Archive(CanisterId),
}
