use std::collections::HashSet;

use serde::{Deserialize, Serialize};

pub use ic_canister_kit::types::*;

#[allow(unused)]
pub use super::super::Business;

#[allow(unused)]
pub use super::super::business::*;
#[allow(unused)]
pub use super::business::*;
#[allow(unused)]
pub use super::permission::*;

// Initialization parameters
pub type InitArgV1 = ::common::archive::token::InitArgV1;

// Upgrade parameters
pub type UpgradeArgV1 = ::common::archive::token::UpgradeArgV1;

#[allow(unused)]
pub use crate::types::{
    BlockIndex, DoHash, EncodedBlock, GetBlocksError, HashOf, IoResult, Message, MetricsEncoder, TokenBlock,
    from_proto_bytes, trap,
};
#[allow(unused)]
pub use ::common::proto;

mod blocks;
mod metrics;

#[allow(unused)]
pub use blocks::*;
#[allow(unused)]
pub use metrics::*;

// Data structures required by the framework
#[derive(Serialize, Deserialize, Default)]
pub struct CanisterKit {}

// The default maximum memory
const DEFAULT_MAX_MEMORY_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10 GB
// Maximum number of requested blocks
pub const MAX_BLOCKS_PER_REQUEST: u64 = 2000;

#[derive(Serialize, Deserialize, Default)]
pub struct BusinessData {
    pub maintainers: Option<HashSet<UserId>>, // None, readable by everyone, otherwise the designated person can read by

    pub max_memory_size_bytes: u64,                     // Maximum memory used
    pub host_canister_id: Option<CanisterId>, // host canister, For business-related update interfaces, check whether the host can be initiated.
    pub block_offset: (BlockIndex, HashOf<TokenBlock>), // Offset recorded in this canister
    pub last_upgrade_timestamp_ns: u64,       // Record the last upgrade time stamp

    pub latest_block_hash: HashOf<TokenBlock>, // This canister records the latest block hash
}

impl BusinessData {
    pub fn block_height_offset(&self) -> BlockIndex {
        self.block_offset.0
    }
}

// Put together those that can be serialized and those that cannot be serialized
// The following annotations are used for serialization
// #[serde(skip)] Default initialization method
// #[serde(skip, default="init_xxx_data")] Specify the initialization method
// ! If you use the stable memory provided by ic-stable-structures, the usage type of memory_id cannot be changed, otherwise each version will be incompatible and the data will be cleared
#[derive(Serialize, Deserialize)]
pub struct InnerState {
    pub canister_kit: CanisterKit, // Data required by the framework //  ? Heap memory Serialization

    // Business data
    pub business_data: BusinessData, // Business data //  ? Heap memory Serialization

    #[serde(skip, default = "init_blocks")]
    pub blocks: Blocks, // Business data // ? Stable memory
}

impl Default for InnerState {
    fn default() -> Self {
        ic_cdk::println!("InnerState::default()");
        Self {
            canister_kit: Default::default(),

            // Business data
            business_data: Default::default(),

            blocks: init_blocks(),
        }
    }
}

use ic_canister_kit::stable;

const MEMORY_ID_BLOCKS_INDEX: MemoryId = MemoryId::new(0); // blocks index
const MEMORY_ID_BLOCKS_DATA: MemoryId = MemoryId::new(1); // blocks data

fn init_blocks() -> Blocks {
    Blocks::new(stable::init_log_data(MEMORY_ID_BLOCKS_INDEX, MEMORY_ID_BLOCKS_DATA))
}

impl InnerState {
    pub fn do_init(&mut self, arg: InitArgV1) {
        self.business_data.maintainers = arg.maintainers.map(HashSet::from_iter);

        self.business_data.max_memory_size_bytes = arg.max_memory_size_bytes.unwrap_or(DEFAULT_MAX_MEMORY_SIZE);
        self.business_data.host_canister_id = match arg.host_canister_id {
            Some(host_canister_id) => Some(host_canister_id),
            None => Some(ic_canister_kit::identity::caller()),
        };
        self.business_data.block_offset = arg.block_offset.unwrap_or_default();
        self.business_data.last_upgrade_timestamp_ns = 0;
        self.business_data.latest_block_hash = self.business_data.block_offset.1;
    }

    pub fn do_upgrade(&mut self, arg: UpgradeArgV1) {
        // Expand maintenance personnel
        if let Some(maintainers) = arg.maintainers {
            match &mut self.business_data.maintainers {
                Some(_maintainers) => _maintainers.extend(maintainers),
                None => self.business_data.maintainers = Some(HashSet::from_iter(maintainers)),
            }
        }

        // Update business data
        self.business_data.last_upgrade_timestamp_ns = ic_cdk::api::time();

        if let Some(max_memory_size_bytes) = arg.max_memory_size_bytes {
            self.update_max_memory_size_bytes(max_memory_size_bytes);
        }
    }

    pub fn update_max_memory_size_bytes(&mut self, max_memory_size_bytes: u64) {
        let total_block_size = self.blocks.total_block_size();
        if max_memory_size_bytes < total_block_size {
            ic_cdk::trap(format!(
                "Cannot set max_memory_size_bytes to {max_memory_size_bytes}, because it is lower than total_block_size {total_block_size}.",
            ));
        }
        self.business_data.max_memory_size_bytes = max_memory_size_bytes;
    }
}
