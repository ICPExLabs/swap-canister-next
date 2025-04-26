use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::types::CanisterId;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenFrozenArg {
    pub token: CanisterId,
    pub frozen: bool,
}
