use candid::CandidType;
use common::types::BlockIndex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct CustomMetrics {
    pub block_height_offset: BlockIndex,
    pub max_memory_size_bytes: u64,
    pub blocks: u64,
    pub blocks_bytes: u64,
    pub stable_memory_pages: u64,
    pub stable_memory_bytes: u64,
    pub heap_memory_bytes: u64,
    pub last_upgrade_time_seconds: u64,
}
