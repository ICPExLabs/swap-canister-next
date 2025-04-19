use ic_canister_kit::types::{CanisterId, UserId};
use serde::{Deserialize, Serialize};

use crate::types::{BlockIndex, HashOf};

use super::SwapBlock;

/// swap archive 部署参数
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArgV1 {
    /// 维护者账户
    pub maintainers: Option<Vec<UserId>>, // init maintainers or deployer

    /// 最大内存
    pub max_memory_size_bytes: Option<u64>,
    /// 宿主罐子
    pub core_canister_id: Option<CanisterId>,
    /// 块偏移
    pub block_offset: Option<(BlockIndex, HashOf<SwapBlock>)>,
}

/// swap archive 升级参数
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType)]
pub struct UpgradeArgV1 {
    /// 添加新的维护者账户
    pub maintainers: Option<Vec<UserId>>, // add new maintainers of not

    /// 最大内存
    pub max_memory_size_bytes: Option<u64>,
}
