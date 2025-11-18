use ic_canister_kit::types::*;
use serde::{Deserialize, Serialize};

use super::CurrentArchiving;

// initialization parameters
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArgV1 {
    pub maintainers: Option<Vec<UserId>>, // init maintainers or deployer
    pub schedule: Option<DurationNanos>,  // init scheduled task or not
    pub current_archiving_token: Option<CurrentArchiving>,
    pub current_archiving_swap: Option<CurrentArchiving>,
}
