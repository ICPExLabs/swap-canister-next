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

// 初始化参数
pub type InitArgV1 = ::common::archive::swap::InitArgV1;

// 升级参数
pub type UpgradeArgV1 = ::common::archive::swap::UpgradeArgV1;

#[allow(unused)]
pub use crate::types::{
    BlockIndex, DoHash, EncodedBlock, GetBlocksError, HashOf, IoResult, Message, MetricsEncoder,
    SwapBlock, from_proto_bytes, trap,
};
#[allow(unused)]
pub use ::common::proto;

mod blocks;
mod metrics;

#[allow(unused)]
pub use blocks::*;
#[allow(unused)]
pub use metrics::*;

// 框架需要的数据结构
#[derive(Serialize, Deserialize, Default)]
pub struct CanisterKit {}

// 默认的最大内存
const DEFAULT_MAX_MEMORY_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10 GB
// 最大请求块数
pub const MAX_BLOCKS_PER_REQUEST: u64 = 2000;

#[derive(Serialize, Deserialize, Default)]
pub struct BusinessData {
    pub maintainers: Option<HashSet<UserId>>, // None, 所有人可读, 否则指定人员可读

    pub max_memory_size_bytes: u64,                    // 最大使用内存
    pub core_canister_id: Option<CanisterId>, // 宿主罐子, 业务相关的 update 接口，都要检查是否宿主罐子发起的
    pub block_offset: (BlockIndex, HashOf<SwapBlock>), // 本罐子记录的偏移量
    pub last_upgrade_timestamp_ns: u64,       // 记录上次升级时间戳

    pub latest_block_hash: HashOf<SwapBlock>, // 本罐子记录最新的块 hash
}

impl BusinessData {
    pub fn block_height_offset(&self) -> BlockIndex {
        self.block_offset.0
    }
}

// 能序列化的和不能序列化的放在一起
// 其中不能序列化的采用如下注解
// #[serde(skip)] 默认初始化方式
// #[serde(skip, default="init_xxx_data")] 指定初始化方式
// ! 如果使用 ic-stable-structures 提供的稳定内存，不能变更 memory_id 的使用类型，否则会出现各个版本不兼容，数据会被清空
#[derive(Serialize, Deserialize)]
pub struct InnerState {
    pub canister_kit: CanisterKit, // 框架需要的数据 // ? 堆内存 序列化

    // 业务数据
    pub business_data: BusinessData, // 业务数据 // ? 堆内存 序列化

    #[serde(skip, default = "init_blocks")]
    pub blocks: Blocks, // 业务数据 // ? 稳定内存
}

impl Default for InnerState {
    fn default() -> Self {
        ic_cdk::println!("InnerState::default()");
        Self {
            canister_kit: Default::default(),

            // 业务数据
            business_data: Default::default(),

            blocks: init_blocks(),
        }
    }
}

use ic_canister_kit::stable;

const MEMORY_ID_BLOCKS_INDEX: MemoryId = MemoryId::new(0); // 测试 Log
const MEMORY_ID_BLOCKS_DATA: MemoryId = MemoryId::new(1); // 测试 Log

fn init_blocks() -> Blocks {
    Blocks::new(stable::init_log_data(
        MEMORY_ID_BLOCKS_INDEX,
        MEMORY_ID_BLOCKS_DATA,
    ))
}

impl InnerState {
    pub fn do_init(&mut self, arg: InitArgV1) {
        self.business_data.maintainers = arg.maintainers.map(HashSet::from_iter);

        self.business_data.max_memory_size_bytes =
            arg.max_memory_size_bytes.unwrap_or(DEFAULT_MAX_MEMORY_SIZE);
        self.business_data.core_canister_id = match arg.core_canister_id {
            Some(core_canister_id) => Some(core_canister_id),
            None => Some(ic_canister_kit::identity::caller()),
        };
        self.business_data.block_offset = arg.block_offset.unwrap_or_default();
        self.business_data.last_upgrade_timestamp_ns = 0;
        self.business_data.latest_block_hash = self.business_data.block_offset.1;
    }

    pub fn do_upgrade(&mut self, arg: UpgradeArgV1) {
        // 拓展维护人员
        if let Some(maintainers) = arg.maintainers {
            match &mut self.business_data.maintainers {
                Some(_maintainers) => _maintainers.extend(maintainers),
                None => self.business_data.maintainers = Some(HashSet::from_iter(maintainers)),
            }
        }

        // 更新业务数据
        self.business_data.last_upgrade_timestamp_ns = ic_cdk::api::time();

        if let Some(max_memory_size_bytes) = arg.max_memory_size_bytes {
            self.update_max_memory_size_bytes(max_memory_size_bytes);
        }
    }

    pub fn update_max_memory_size_bytes(&mut self, max_memory_size_bytes: u64) {
        let total_block_size = self.blocks.total_block_size();
        if max_memory_size_bytes < total_block_size {
            ic_cdk::trap(&format!(
                "Cannot set max_memory_size_bytes to {max_memory_size_bytes}, because it is lower than total_block_size {total_block_size}.",
            ));
        }
        self.business_data.max_memory_size_bytes = max_memory_size_bytes;
    }
}
