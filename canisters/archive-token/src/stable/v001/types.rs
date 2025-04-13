use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

pub use ic_canister_kit::types::*;

#[allow(unused)]
pub use super::super::{Business, ParsePermission, ScheduleTask};

#[allow(unused)]
pub use super::super::business::*;
#[allow(unused)]
pub use super::business::*;
#[allow(unused)]
pub use super::permission::*;
#[allow(unused)]
pub use super::schedule::schedule_task;

// 初始化参数
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArgV1 {
    pub maintainers: Option<Vec<UserId>>, // init maintainers or deployer
    pub schedule: Option<DurationNanos>,  // init scheduled task or not

    pub max_memory_size_bytes: Option<u64>,
    pub core_canister_id: Option<CanisterId>,
    pub block_offset: Option<(BlockIndex, HashOf<TokenBlock>)>,
}

// 升级参数
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType)]
pub struct UpgradeArgV1 {
    pub maintainers: Option<Vec<UserId>>, // add new maintainers of not
    pub schedule: Option<DurationNanos>,  // init scheduled task or not

    pub max_memory_size_bytes: Option<u64>,
}

#[allow(unused)]
pub use crate::types::{
    BlockIndex, DoHash, EncodedBlock, GetBlocksError, HashOf, IoResult, Message, MetricsEncoder,
    TokenBlock, from_proto_bytes, trap,
};
#[allow(unused)]
pub use ::common::proto;

mod blocks;

#[allow(unused)]
pub use blocks::*;

#[allow(unused)]
#[derive(Debug, Clone, Copy, EnumIter, EnumString, strum_macros::Display)]
pub enum RecordTopics {
    // ! 新的权限类型从 0 开始
    Example = 0,              // 模版样例
    ExampleCell = 1,          // 模版样例
    ExampleVec = 2,           // 模版样例
    ExampleMap = 3,           // 模版样例
    ExampleLog = 4,           // 模版样例
    ExamplePriorityQueue = 5, // 模版样例

    // ! 系统倒序排列
    CyclesCharge = 249, // 充值
    Upgrade = 250,      // 升级
    Schedule = 251,     // 定时任务
    Record = 252,       // 记录
    Permission = 253,   // 权限
    Pause = 254,        // 维护
    Initial = 255,      // 初始化
}
#[allow(unused)]
impl RecordTopics {
    pub fn topic(&self) -> RecordTopic {
        *self as u8
    }
    pub fn topics() -> Vec<String> {
        RecordTopics::iter().map(|x| x.to_string()).collect()
    }
    pub fn from(topic: &str) -> Result<Self, strum::ParseError> {
        RecordTopics::from_str(topic)
    }
}

// 框架需要的数据结构
#[derive(Serialize, Deserialize, Default)]
pub struct CanisterKit {
    pub pause: Pause,             // 记录维护状态 // ? 堆内存 序列化
    pub permissions: Permissions, // 记录自身权限 // ? 堆内存 序列化
    pub records: Records,         // 记录操作记录 // ? 堆内存 序列化
    pub schedule: Schedule,       // 记录定时任务 // ? 堆内存 序列化
}

// 默认的最大内存
const DEFAULT_MAX_MEMORY_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10 GB
// 最大请求块数
pub const MAX_BLOCKS_PER_REQUEST: u64 = 2000;

#[derive(Serialize, Deserialize, Default)]
pub struct BusinessData {
    pub maintainers: Option<HashSet<UserId>>, // None, 所有人可读, 否则指定人员可读

    pub max_memory_size_bytes: u64,                     // 最大使用内存
    pub core_canister_id: Option<CanisterId>, // 宿主罐子, 业务相关的 update 接口，都要检查是否宿主罐子发起的
    pub block_offset: (BlockIndex, HashOf<TokenBlock>), // 本罐子记录的偏移量
    pub last_upgrade_timestamp_ns: u64,       // 记录上次升级时间戳

    pub latest_block_hash: HashOf<TokenBlock>, // 本罐子记录最新的块 hash
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

    pub example_data: String, // 样例数据 // ? 堆内存 序列化

    #[serde(skip, default = "init_example_cell_data")]
    pub example_cell: StableCell<ExampleCell>, // 样例数据 // ? 稳定内存
    #[serde(skip, default = "init_example_vec_data")]
    pub example_vec: StableVec<ExampleVec>, // 样例数据 // ? 稳定内存
    #[serde(skip, default = "init_example_map_data")]
    pub example_map: StableBTreeMap<u64, String>, // 样例数据 // ? 稳定内存
    #[serde(skip, default = "init_example_log_data")]
    pub example_log: StableLog<String>, // 样例数据 // ? 稳定内存
    #[serde(skip, default = "init_example_priority_queue_data")]
    pub example_priority_queue: StablePriorityQueue<ExampleVec>, // 样例数据 // ? 稳定内存
}

impl Default for InnerState {
    fn default() -> Self {
        ic_cdk::println!("InnerState::default()");
        Self {
            canister_kit: Default::default(),

            // 业务数据
            business_data: Default::default(),

            blocks: init_blocks(),

            example_data: Default::default(),

            example_cell: init_example_cell_data(),
            example_vec: init_example_vec_data(),
            example_map: init_example_map_data(),
            example_log: init_example_log_data(),
            example_priority_queue: init_example_priority_queue_data(),
        }
    }
}

use candid::CandidType;
use ic_canister_kit::stable;

const MEMORY_ID_BLOCKS_INDEX: MemoryId = MemoryId::new(103); // 测试 Log
const MEMORY_ID_BLOCKS_DATA: MemoryId = MemoryId::new(104); // 测试 Log

const MEMORY_ID_EXAMPLE_CELL: MemoryId = MemoryId::new(100); // 测试 Cell
const MEMORY_ID_EXAMPLE_VEC: MemoryId = MemoryId::new(101); // 测试 Vec
const MEMORY_ID_EXAMPLE_MAP: MemoryId = MemoryId::new(102); // 测试 Map
const MEMORY_ID_EXAMPLE_LOG_INDEX: MemoryId = MemoryId::new(103); // 测试 Log
const MEMORY_ID_EXAMPLE_LOG_DATA: MemoryId = MemoryId::new(104); // 测试 Log
const MEMORY_ID_EXAMPLE_PRIORITY_QUEUE: MemoryId = MemoryId::new(105); // 测试 PriorityQueue

fn init_blocks() -> Blocks {
    Blocks::new(stable::init_log_data(
        MEMORY_ID_BLOCKS_INDEX,
        MEMORY_ID_BLOCKS_DATA,
    ))
}

fn init_example_cell_data() -> StableCell<ExampleCell> {
    stable::init_cell_data(MEMORY_ID_EXAMPLE_CELL, ExampleCell::default())
}

fn init_example_vec_data() -> StableVec<ExampleVec> {
    stable::init_vec_data(MEMORY_ID_EXAMPLE_VEC)
}

fn init_example_map_data() -> StableBTreeMap<u64, String> {
    stable::init_map_data(MEMORY_ID_EXAMPLE_MAP)
}

fn init_example_log_data() -> StableLog<String> {
    stable::init_log_data(MEMORY_ID_EXAMPLE_LOG_INDEX, MEMORY_ID_EXAMPLE_LOG_DATA)
}

fn init_example_priority_queue_data() -> StablePriorityQueue<ExampleVec> {
    stable::init_priority_queue_data(MEMORY_ID_EXAMPLE_PRIORITY_QUEUE)
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
pub struct ExampleCell {
    pub cell_data: String,
}

impl Storable for ExampleCell {
    fn to_bytes(&self) -> Cow<[u8]> {
        use ic_canister_kit::common::trap;
        Cow::Owned(trap(ic_canister_kit::functions::stable::to_bytes(self)))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        use ic_canister_kit::common::trap;
        trap(ic_canister_kit::functions::stable::from_bytes(&bytes))
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ExampleVec {
    pub vec_data: u64,
}

impl Storable for ExampleVec {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = vec![];
        ic_canister_kit::stable::common::u64_to_bytes(&mut bytes, self.vec_data);
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self {
            vec_data: ic_canister_kit::stable::common::u64_from_bytes(&bytes),
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 8,
        is_fixed_size: true,
    };
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
            let total_block_size = self.blocks.total_block_size();
            if max_memory_size_bytes < total_block_size {
                ic_cdk::trap(&format!(
                    "Cannot set max_memory_size_bytes to {max_memory_size_bytes}, because it is lower than total_block_size {total_block_size}.",
                ));
            }
            self.business_data.max_memory_size_bytes = max_memory_size_bytes;
        }
    }
}
