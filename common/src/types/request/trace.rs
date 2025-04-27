#[cfg(feature = "cdk")]
use std::borrow::Cow;

use candid::CandidType;
#[cfg(feature = "cdk")]
use ic_canister_kit::types::{Bound, Storable};
use serde::{Deserialize, Serialize};

#[allow(unused)]
use crate::types::{TimestampNanos, TokenAccount};

use super::{BusinessLocks, RequestArgs, RequestIndex};

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum RequestTraceResult {
    #[serde(rename = "ok")]
    Ok(String),
    #[serde(rename = "err")]
    Err(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct RequestTraceDone {
    pub done: TimestampNanos,
    pub result: RequestTraceResult,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct RequestTrace {
    pub index: RequestIndex,
    pub created: TimestampNanos,
    pub args: RequestArgs,
    pub locks: BusinessLocks,
    pub traces: Vec<(TimestampNanos, String)>,
    pub done: Option<RequestTraceDone>,
}

#[cfg(feature = "cdk")]
impl Storable for RequestTrace {
    fn to_bytes(&self) -> Cow<[u8]> {
        use ic_canister_kit::common::trap;
        Cow::Owned(trap(ic_canister_kit::functions::stable::to_bytes(self)))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        use ic_canister_kit::common::trap;
        trap(ic_canister_kit::functions::stable::from_bytes(&bytes))
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl RequestTrace {
    #[cfg(feature = "cdk")]
    pub fn new(
        index: RequestIndex,
        args: RequestArgs,
        token: Option<bool>,
        swap: Option<bool>,
        balances: Option<Vec<TokenAccount>>,
        trace: Option<String>,
    ) -> Self {
        let mut request_trace = Self {
            index,
            created: TimestampNanos::now(),
            args,
            locks: BusinessLocks::new(token, swap, balances),
            traces: vec![],
            done: None,
        };
        if let Some(trace) = trace {
            request_trace.trace(trace);
        }
        request_trace
    }

    #[cfg(feature = "cdk")]
    pub fn from_args(args: RequestArgs) -> Self {
        Self {
            index: Default::default(),
            created: TimestampNanos::now(),
            args,
            locks: BusinessLocks::default(),
            traces: vec![],
            done: None,
        }
    }

    #[cfg(feature = "cdk")]
    pub fn trace(&mut self, trace: String) {
        self.traces.push((TimestampNanos::now(), trace));
    }

    #[cfg(feature = "cdk")]
    pub fn success(&mut self, success: String) {
        self.done = Some(RequestTraceDone {
            done: TimestampNanos::now(),
            result: RequestTraceResult::Ok(success),
        });
    }

    #[cfg(feature = "cdk")]
    pub fn failed(&mut self, failed: String) {
        self.done = Some(RequestTraceDone {
            done: TimestampNanos::now(),
            result: RequestTraceResult::Err(failed),
        });
    }
}
