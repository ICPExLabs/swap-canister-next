use serde::{Deserialize, Serialize};

// Upgrade parameters
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType)]
pub struct UpgradeArg {}
