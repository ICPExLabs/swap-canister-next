use std::time::Duration;

use candid::CandidType;
use common::types::{BusinessError, Caller, TimestampNanos};
use serde::{Deserialize, Serialize};

// Default 1 day time
const TRANSACTION_WINDOW: Duration = Duration::from_secs(24 * 60 * 60);

pub fn check_meta(
    memo: &Option<Vec<u8>>,
    created: &Option<TimestampNanos>,
) -> Result<TimestampNanos, BusinessError> {
    let now = TimestampNanos::now();

    if let Some(memo) = memo.as_ref() {
        if 32 < memo.len() {
            return Err(BusinessError::MemoTooLong);
        }
    }
    if let Some(created) = created {
        if created.into_inner() as u128 + TRANSACTION_WINDOW.as_nanos() < now.into_inner() as u128 {
            return Err(BusinessError::InvalidCreated {
                system: now.into_inner(),
                created: created.into_inner(),
            });
        }
    }
    Ok(now)
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct ArgWithMeta<T> {
    pub now: TimestampNanos,
    pub caller: Caller,
    pub arg: T,
    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

impl<T> ArgWithMeta<T> {
    pub fn simple(now: TimestampNanos, caller: Caller, arg: T) -> Self {
        Self {
            now,
            caller,
            arg,
            memo: None,
            created: None,
        }
    }
}
