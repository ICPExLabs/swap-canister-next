use ic_canister_kit::types::{CanisterId, UserId};
use serde::{Deserialize, Serialize};

use crate::types::{BlockIndex, HashOf};

use super::TokenBlock;

/// token archive deploy args
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArgV1 {
    /// Maintainer Account
    pub maintainers: Option<Vec<UserId>>, // init maintainers or deployer

    /// Maximum memory
    pub max_memory_size_bytes: Option<u64>,
    /// Host canister
    pub core_canister_id: Option<CanisterId>,
    /// Block Offset
    pub block_offset: Option<(BlockIndex, HashOf<TokenBlock>)>,
}

/// token archive upgrade args
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType)]
pub struct UpgradeArgV1 {
    /// Add a new maintainer account
    pub maintainers: Option<Vec<UserId>>, // add new maintainers of not

    /// Maximum memory
    pub max_memory_size_bytes: Option<u64>,
}
