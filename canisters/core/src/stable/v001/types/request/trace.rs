use std::borrow::Cow;

use common::types::TimestampNanos;
use ic_canister_kit::types::{Bound, Storable};
use serde::{Deserialize, Serialize};

use super::{
    BusinessLocks, RequestArgs, RequestIndex, SwapBlockChainGuard, TokenBalancesGuard,
    TokenBlockChainGuard,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTrace {
    index: RequestIndex,
    args: RequestArgs,
    locks: BusinessLocks,
    traces: Vec<(TimestampNanos, String)>,
    done: Option<(TimestampNanos, Result<String, String>)>,
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
        balances: Option<&TokenBalancesGuard<'_>>,
        token: Option<&TokenBlockChainGuard<'_>>,
        swap: Option<&SwapBlockChainGuard<'_>>,
        trace: Option<String>,
    ) -> Self {
        let mut request_trace = Self {
            index,
            args,
            locks: BusinessLocks::new(balances, token, swap),
            traces: vec![],
            done: None,
        };
        if let Some(trace) = trace {
            request_trace.trace(trace);
        }
        request_trace
    }

    pub fn trace(&mut self, trace: String) {
        self.traces.push((TimestampNanos::now(), trace));
    }

    pub fn success(&mut self, success: String) {
        self.done = Some((TimestampNanos::now(), Ok(success)));
    }

    pub fn failed(&mut self, failed: String) {
        self.done = Some((TimestampNanos::now(), Err(failed)));
    }
}
