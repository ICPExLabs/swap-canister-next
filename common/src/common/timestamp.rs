use candid::CandidType;
use serde::{Deserialize, Serialize};

/// 时间戳 纳秒
/// i64 最大值 9_223_372_036_854_775_807 -> 2262-04-12 等这个时候我早挂了，所以“千年虫”问题留给别人解决吧 🐶
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, CandidType,
)]
pub struct TimestampNanos(u64);

impl TimestampNanos {
    /// 当前时间
    pub fn now() -> Self {
        Self(ic_cdk::api::time())
    }

    /// 构造
    pub fn from_inner(inner: u64) -> Self {
        Self(inner)
    }

    /// 取出
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
