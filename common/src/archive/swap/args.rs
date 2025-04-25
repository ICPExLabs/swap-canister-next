use serde::{Deserialize, Serialize};

use crate::types::{BlockIndex, CanisterId, HashOf, UserId};

use super::SwapBlock;

/// swap archive Deployment parameters
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArgV1 {
    /// Maintainer Account
    pub maintainers: Option<Vec<UserId>>, // init maintainers or deployer

    /// Maximum memory
    pub max_memory_size_bytes: Option<u64>,
    /// Host canister
    pub host_canister_id: Option<CanisterId>,
    /// Block Offset
    pub block_offset: Option<(BlockIndex, HashOf<SwapBlock>)>,
}

/// swap archive Upgrade parameters
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType)]
pub struct UpgradeArgV1 {
    /// Add a new maintainer account
    pub maintainers: Option<Vec<UserId>>, // add new maintainers of not

    /// Maximum memory
    pub max_memory_size_bytes: Option<u64>,
}
