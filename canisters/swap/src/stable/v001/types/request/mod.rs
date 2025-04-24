use std::sync::RwLock;

use common::types::BusinessError;
use ic_canister_kit::{common::trap, types::StableBTreeMap};
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
    fn get_start_request_index(&self, length: Option<u64>) -> RequestIndex {
        let length = length.unwrap_or_else(|| self.traces.len());
        trap(self.next_index.read()).previous(length)
    }

    pub fn get_request_index(&self) -> (RequestIndex, u64) {
        let length = self.traces.len();
        (self.get_start_request_index(Some(length)), length)
    }
    pub fn get_request_trace(&self, index: &RequestIndex) -> Option<RequestTrace> {
        self.traces.get(index)
    }
    pub fn remove_request_trace(&mut self, index: &RequestIndex) -> Option<RequestTrace> {
        // must be min
        let min = self.traces.keys().min()?;
        if *index != min {
            ic_cdk::trap(&format!("must remove min request index: {}", min.as_ref()));
        }
        self.traces.remove(index)
    }
    pub fn insert_request_trace(&mut self, trace: RequestTrace) {
        let mut guard = trap(self.be_guard(trace.args, None, None, None, None));
        guard.do_trace(|t| {
            t.traces = trace.traces;
            t.done = trace.done;
        });
    }

    pub fn be_guard<'a>(
        &'a mut self,
        args: RequestArgs,
        token: Option<&TokenBlockChainGuard<'_>>,
        swap: Option<&SwapBlockChainGuard<'_>>,
        balances: Option<&TokenBalancesGuard<'_>>,
        trace: Option<String>,
    ) -> Result<RequestTraceGuard<'a>, BusinessError> {
        let mut next_index = self
            .next_index
            .write()
            .map_err(|err| BusinessError::RequestTraceLocked(format!("{err}")))?;
        let index = next_index.next();
        let trace = RequestTrace::new(index, args, token, swap, balances, trace);
        self.traces.insert(index, trace); // insert
        let lock = RequestTraceLock { index };
        ic_cdk::println!("🔒 Locked request index: {}", index.as_ref());

        Ok(RequestTraceGuard {
            traces: &mut self.traces,
            lock,
        })
    }
}

// ============================ lock ============================

pub struct RequestTraceLock {
    index: RequestIndex, // request index
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

    pub fn handle<T, F, D>(&mut self, handle: F, success: D) -> Result<T, BusinessError>
    where
        F: FnOnce(&mut RequestTraceGuard<'_>) -> Result<T, BusinessError>,
        D: FnOnce(&T) -> String,
    {
        let result = handle(self);
        match &result {
            Ok(data) => {
                let success = success(data);
                self.success(success);
            }
            Err(err) => {
                self.failed(err.to_string());
            }
        }
        result
    }
}
