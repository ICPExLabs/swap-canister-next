use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

/// 查询块
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum QueryBlockResult<T> {
    /// 块内内容
    Block(T),
    /// 罐子
    Archive(CanisterId),
}
