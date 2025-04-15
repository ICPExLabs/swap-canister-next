use std::sync::RwLock;

use common::types::BusinessError;
use ic_canister_kit::types::StableBTreeMap;
use serde::{Deserialize, Serialize};

mod index;
pub use index::*;

mod args;
pub use args::*;

mod lock;
pub use lock::*;

mod trace;
pub use trace::*;

use super::{SwapBlockChainGuard, TokenBalancesGuard, TokenBlockChainGuard, init_request_traces};

// ============================ request traces ============================

#[derive(Serialize, Deserialize)]
pub struct RequestTraces {
    #[serde(skip, default = "init_request_traces")]
    traces: StableBTreeMap<RequestIndex, RequestTrace>,
    next_index: RwLock<RequestIndex>,
}

impl Default for RequestTraces {
    fn default() -> Self {
        Self {
            traces: init_request_traces(),
            next_index: RwLock::new(RequestIndex::default()),
        }
    }
}

impl RequestTraces {
    fn new_guard<'a>(
        &'a mut self,
        args: RequestArgs,
        balances: Option<&TokenBalancesGuard<'_>>,
        token: Option<&TokenBlockChainGuard<'_>>,
        swap: Option<&SwapBlockChainGuard<'_>>,
    ) -> Result<RequestTraceGuard<'a>, BusinessError> {
        let mut next_index = self
            .next_index
            .write()
            .map_err(|err| BusinessError::RequestTraceLocked(format!("{err}")))?;
        let index = next_index.next();
        let trace = RequestTrace::new(index, args, balances, token, swap);
        self.traces.insert(index, trace); // insert
        let lock = RequestTraceLock { index };
        Ok(RequestTraceGuard {
            traces: &mut self.traces,
            lock,
        })
    }
}

// ============================ lock ============================

pub struct RequestTraceLock {
    index: RequestIndex, // 记录 index
}
impl Drop for RequestTraceLock {
    fn drop(&mut self) {
        ic_cdk::println!("🔐 Unlock request index: {}", self.index.as_ref());
    }
}

// ============================ guard ============================

pub struct RequestTraceGuard<'a> {
    traces: &'a mut StableBTreeMap<RequestIndex, RequestTrace>,
    lock: RequestTraceLock,
}

impl RequestTraceGuard<'_> {
    fn do_trace<F>(&mut self, handle: F)
    where
        F: FnOnce(&mut RequestTrace),
    {
        let mut trace = match self.traces.get(&self.lock.index) {
            Some(trace) => trace,
            None => {
                ic_cdk::println!(
                    "can not find request trace by {}. CAN NOT BE.",
                    self.lock.index.as_ref()
                );
                return;
            }
        };
        handle(&mut trace);
        self.traces.insert(self.lock.index, trace);
    }

    pub fn trace(&mut self, trace: String) {
        self.do_trace(|request_trace| request_trace.trace(trace));
    }

    fn success(&mut self, success: String) {
        self.do_trace(|request_trace| request_trace.success(success));
    }

    fn failed(&mut self, failed: String) {
        self.do_trace(|request_trace| request_trace.failed(failed));
    }
}
