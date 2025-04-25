use candid::CandidType;
use serde::{Deserialize, Serialize};

/// Timestamp Nanoseconds
/// i64 Maximum value 9_223_372_036_854_775_807 -> 2262-04-12 I've died long ago at this time, so I'll leave the "Millennium Bug" problem to others to solve 🐶
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, CandidType)]
pub struct TimestampNanos(u64);

impl TimestampNanos {
    /// Current time
    #[cfg(feature = "cdk")]
    pub fn now() -> Self {
        Self(ic_cdk::api::time())
    }

    /// new
    pub fn from_inner(inner: u64) -> Self {
        Self(inner)
    }

    /// inner
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
