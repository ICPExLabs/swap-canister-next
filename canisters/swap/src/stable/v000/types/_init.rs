use ic_canister_kit::types::*;
use serde::{Deserialize, Serialize};

// initialization parameters
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArg {
    pub maintainers: Option<Vec<UserId>>, // init maintainers or deployer
    pub schedule: Option<DurationNanos>,  // init scheduled task or not
}
