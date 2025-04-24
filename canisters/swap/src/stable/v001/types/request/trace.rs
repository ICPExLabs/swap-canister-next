use std::borrow::Cow;

use candid::CandidType;
use common::types::TimestampNanos;
use ic_canister_kit::types::{Bound, Storable};
use serde::{Deserialize, Serialize};

use super::{
    BusinessLocks, RequestArgs, RequestIndex, SwapBlockChainGuard, TokenBalancesGuard,
    TokenBlockChainGuard,
};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum RequestTraceResult {
    #[serde(rename = "ok")]
    Ok(String),
    #[serde(rename = "err")]
    Err(String),
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct RequestTraceDone {
    done: TimestampNanos,
    result: RequestTraceResult,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct RequestTrace {
    index: RequestIndex,
    pub args: RequestArgs,
    locks: BusinessLocks,
    pub traces: Vec<(TimestampNanos, String)>,
    pub done: Option<RequestTraceDone>,
}

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
    pub fn new(
        index: RequestIndex,
        args: RequestArgs,
        token: Option<&TokenBlockChainGuard<'_>>,
        swap: Option<&SwapBlockChainGuard<'_>>,
        balances: Option<&TokenBalancesGuard<'_>>,
        trace: Option<String>,
    ) -> Self {
        let mut request_trace = Self {
            index,
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

    pub fn from_args(args: RequestArgs) -> Self {
        Self {
            index: Default::default(),
            args,
            locks: BusinessLocks::default(),
            traces: vec![],
            done: None,
        }
    }

    pub fn trace(&mut self, trace: String) {
        self.traces.push((TimestampNanos::now(), trace));
    }

    pub fn success(&mut self, success: String) {
        self.done = Some(RequestTraceDone {
            done: TimestampNanos::now(),
            result: RequestTraceResult::Ok(success),
        });
    }

    pub fn failed(&mut self, failed: String) {
        self.done = Some(RequestTraceDone {
            done: TimestampNanos::now(),
            result: RequestTraceResult::Err(failed),
        });
    }
}
