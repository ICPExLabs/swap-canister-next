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
pub struct InitArg {
    pub maintainers: Option<Vec<UserId>>, // init maintainers or deployer
    pub schedule: Option<DurationNanos>,  // init scheduled task or not
}

// 升级参数
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType)]
pub struct UpgradeArg {
    pub maintainers: Option<Vec<UserId>>, // add new maintainers of not
    pub schedule: Option<DurationNanos>,  // init scheduled task or not
}

#[allow(unused)]
pub use crate::types::business::*;
#[allow(unused)]
pub use crate::types::common::*;
#[allow(unused)]
pub use crate::types::{Account, Nat};

mod amm;
mod balance;
mod lp;
mod pair;
mod token;

#[allow(unused)]
pub use amm::*;
#[allow(unused)]
pub use balance::*;
#[allow(unused)]
pub use lp::*;
#[allow(unused)]
pub use pair::*;
#[allow(unused)]
pub use token::*;

#[allow(unused)]
#[derive(Debug, Clone, Copy, EnumIter, EnumString, strum_macros::Display)]
pub enum RecordTopics {
    // ! 新的权限类型从 0 开始
    TokenBalance = 0, // 用户持有的某 Token 的余额

    // example
    Example = 100,              // 模版样例
    ExampleCell = 101,          // 模版样例
    ExampleVec = 102,           // 模版样例
    ExampleMap = 103,           // 模版样例
    ExampleLog = 104,           // 模版样例
    ExamplePriorityQueue = 105, // 模版样例

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

#[derive(Serialize, Deserialize, Default)]
pub struct BusinessData {
    pub token_balance_locks: TokenBalanceLocks, // 记录账户锁
    pub fee_to: Option<Account>, // 记录协议费收集者账户, lp 代币转移也需要收集转移费用
    pub token_pairs: TokenPairs, // 所有交易对池子
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

    #[serde(skip, default = "init_token_balances")]
    pub token_balances: TokenBalances, // 业务数据 // ? 稳定内存

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

            token_balances: init_token_balances(),

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

// Token
const MEMORY_ID_TOKEN_BALANCES: MemoryId = MemoryId::new(0); // token balances

// example
const MEMORY_ID_EXAMPLE_CELL: MemoryId = MemoryId::new(100); // 测试 Cell
const MEMORY_ID_EXAMPLE_VEC: MemoryId = MemoryId::new(101); // 测试 Vec
const MEMORY_ID_EXAMPLE_MAP: MemoryId = MemoryId::new(102); // 测试 Map
const MEMORY_ID_EXAMPLE_LOG_ID: MemoryId = MemoryId::new(103); // 测试 Log
const MEMORY_ID_EXAMPLE_LOG_DATA: MemoryId = MemoryId::new(104); // 测试 Log
const MEMORY_ID_EXAMPLE_PRIORITY_QUEUE: MemoryId = MemoryId::new(105); // 测试 PriorityQueue

fn init_token_balances() -> TokenBalances {
    TokenBalances::new(stable::init_map_data(MEMORY_ID_TOKEN_BALANCES))
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
    stable::init_log_data(MEMORY_ID_EXAMPLE_LOG_ID, MEMORY_ID_EXAMPLE_LOG_DATA)
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
        Cow::Owned(ic_canister_kit::functions::stable::to_bytes(self))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        ic_canister_kit::functions::stable::from_bytes(&bytes)
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
