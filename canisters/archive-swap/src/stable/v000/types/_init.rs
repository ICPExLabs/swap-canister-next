use serde::{Deserialize, Serialize};

// Initialization parameters
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArg {}
