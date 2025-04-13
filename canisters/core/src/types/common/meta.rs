use std::time::Duration;

use common::common::TimestampNanos;

use super::{BusinessError, Caller};

// 默认 1 天时间
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
            return Err(BusinessError::InvalidCreated(format!(
                "System Time: {} but created at {}",
                now.into_inner(),
                created.into_inner()
            )));
        }
    }
    Ok(now)
}

pub struct ArgWithMeta<T> {
    pub now: TimestampNanos,
    pub caller: Caller,
    pub arg: T,
    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}
