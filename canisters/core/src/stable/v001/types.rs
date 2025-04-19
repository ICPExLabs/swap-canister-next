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
pub use crate::types::{
    Account, Amm, AmmText, BlockIndex, BurnFee, BusinessError, Caller, CandidBlock, DepositToken,
    DoHash, DummyCanisterId, EncodedBlock, HashOf, Nat, PairCreate, PairCumulativePrice,
    PairOperation, PairSwapToken, QueryBlockResult, QuerySwapBlockResult, QueryTokenBlockResult,
    SelfCanister, SwapBlock, SwapOperation, SwapTransaction, SwapV2BurnToken, SwapV2MintFeeToken,
    SwapV2MintToken, SwapV2Operation, SwapV2TransferToken, TimestampNanos, TokenAccount,
    TokenBlock, TokenOperation, TokenPair, TokenPairAmm, TokenPairLiquidityAddSuccessView,
    TokenPairPool, TokenPairSwapByLoanArg, TokenPairSwapExactTokensForTokensArg,
    TokenPairSwapTokensForExactTokensArg, TokenTransaction, TransferFee, TransferToken, UserId,
    WithdrawToken, display_account, proto, system_error,
};

mod common;
#[allow(unused)]
pub use common::*;

mod balance;
mod blockchain;
mod fee_to;
mod maintain;
mod pair;
mod request;

#[allow(unused)]
pub use balance::*;
#[allow(unused)]
pub use blockchain::*;
#[allow(unused)]
pub use fee_to::*;
#[allow(unused)]
pub use maintain::*;
#[allow(unused)]
pub use pair::*;
#[allow(unused)]
pub use request::*;

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

#[derive(Serialize, Deserialize)]
pub struct BusinessData {
    pub updated: TimestampNanos,             // 记录罐子的最后更新时间
    pub fee_to: FeeTo,                       // 记录协议费收集者账户, lp 代币转移也需要收集转移费用
    pub maintain_archives: MaintainArchives, // 维护罐子信息
}

impl Default for BusinessData {
    fn default() -> Self {
        Self {
            updated: TimestampNanos::from_inner(0),
            fee_to: Default::default(),
            maintain_archives: Default::default(),
        }
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

    pub request_traces: RequestTraces, // 业务数据, 记录请求步骤 // ? 堆内存 序列化 稳定内存
    pub token_block_chain: TokenBlockChain, // 业务数据, 记录 Token 块数据 // ? 堆内存 序列化 稳定内存
    pub swap_block_chain: SwapBlockChain, // 业务数据, 记录 Swap 块数据 // ? 堆内存 序列化 稳定内存

    pub token_pairs: TokenPairs, // 业务数据, 记录交易对数据 // ? 堆内存 序列化 稳定内存
    pub token_balances: TokenBalances, // 业务数据, 记录账户余额数据 // ? 堆内存 序列化 稳定内存

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

            request_traces: Default::default(),
            token_block_chain: Default::default(),
            swap_block_chain: Default::default(),

            token_pairs: Default::default(),
            token_balances: Default::default(),

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

// stable memory
const MEMORY_ID_REQUEST_TRACES: MemoryId = MemoryId::new(0); // request traces

const MEMORY_ID_TOKEN_BLOCKS: MemoryId = MemoryId::new(8); // token blocks
const MEMORY_ID_TOKEN_WASM_MODULE: MemoryId = MemoryId::new(9); // token blocks

const MEMORY_ID_SWAP_BLOCKS: MemoryId = MemoryId::new(16); // swap blocks
const MEMORY_ID_SWAP_WASM_MODULE: MemoryId = MemoryId::new(17); // token blocks

const MEMORY_ID_TOKEN_PAIRS: MemoryId = MemoryId::new(24); // token pairs
const MEMORY_ID_TOKEN_BALANCES: MemoryId = MemoryId::new(25); // token balances

// example
const MEMORY_ID_EXAMPLE_CELL: MemoryId = MemoryId::new(100); // 测试 Cell
const MEMORY_ID_EXAMPLE_VEC: MemoryId = MemoryId::new(101); // 测试 Vec
const MEMORY_ID_EXAMPLE_MAP: MemoryId = MemoryId::new(102); // 测试 Map
const MEMORY_ID_EXAMPLE_LOG_INDEX: MemoryId = MemoryId::new(103); // 测试 Log
const MEMORY_ID_EXAMPLE_LOG_DATA: MemoryId = MemoryId::new(104); // 测试 Log
const MEMORY_ID_EXAMPLE_PRIORITY_QUEUE: MemoryId = MemoryId::new(105); // 测试 PriorityQueue

fn init_request_traces() -> StableBTreeMap<RequestIndex, RequestTrace> {
    stable::init_map_data(MEMORY_ID_REQUEST_TRACES)
}

fn init_token_blocks() -> StableBTreeMap<BlockIndex, EncodedBlock> {
    stable::init_map_data(MEMORY_ID_TOKEN_BLOCKS)
}
fn init_token_wasm_module() -> StableCell<Option<Vec<u8>>> {
    stable::init_cell_data(MEMORY_ID_TOKEN_WASM_MODULE, Default::default())
}

fn init_swap_blocks() -> StableBTreeMap<BlockIndex, EncodedBlock> {
    stable::init_map_data(MEMORY_ID_SWAP_BLOCKS)
}
fn init_swap_wasm_module() -> StableCell<Option<Vec<u8>>> {
    stable::init_cell_data(MEMORY_ID_SWAP_WASM_MODULE, Default::default())
}

fn init_token_pairs() -> StableBTreeMap<TokenPairAmm, MarketMaker> {
    stable::init_map_data(MEMORY_ID_TOKEN_PAIRS)
}
fn init_token_balances() -> StableBTreeMap<TokenAccount, TokenBalance> {
    stable::init_map_data(MEMORY_ID_TOKEN_BALANCES)
}

// fn init_swap_blocks() -> StableBTreeMap<BlockIndex, EncodedBlock> {
//     stable::init_map_data(MEMORY_ID_SWAP_BLOCKS)
// }

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
    pub fn do_init(&mut self, arg: InitArg) {
        self.updated(|s| {
            let maintainers = arg.maintainers.clone().unwrap_or_else(|| {
                vec![ic_canister_kit::identity::caller()] // 默认调用者为维护人员
            });
            s.token_block_chain
                .set_archive_maintainers(Some(maintainers));

            let _ = s.token_block_chain.init_wasm_module();
            let _ = s.swap_block_chain.init_wasm_module();
        });
    }

    pub fn do_upgrade(&mut self, _arg: UpgradeArg) {
        // maybe do something
        let _ = self.token_block_chain.init_wasm_module();
        let _ = self.swap_block_chain.init_wasm_module();

        self.updated(|_| {});
    }

    pub fn updated<T, F>(&mut self, handle: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        let result = handle(self);
        self.business_data.updated = TimestampNanos::now();
        result
    }

    pub fn get_token_guard<'a, T>(
        &'a mut self,
        locks: &'a (TokenBlockChainLock, TokenBalancesLock),
        arg: T,
        trace: Option<String>,
    ) -> Result<TokenGuard<'a>, BusinessError>
    where
        T: Into<RequestArgs>,
    {
        let token_guard = self.token_block_chain.be_guard(&locks.0);
        let balances_guard = self.token_balances.be_guard(&locks.1);
        let trace_guard = self.request_traces.be_guard(
            arg.into(),
            Some(&token_guard),
            None,
            Some(&balances_guard),
            trace,
        )?;
        Ok(TokenGuard::new(trace_guard, balances_guard, token_guard))
    }

    pub fn get_pair_swap_guard<'a, T>(
        &'a mut self,
        locks: &'a (TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: T,
        trace: Option<String>,
    ) -> Result<TokenPairSwapGuard<'a>, BusinessError>
    where
        T: Into<RequestArgs>,
    {
        let token_guard = self.token_block_chain.be_guard(&locks.0);
        let swap_guard = self.swap_block_chain.be_guard(&locks.1);
        let balances_guard = self.token_balances.be_guard(&locks.2);
        let trace_guard = self.request_traces.be_guard(
            arg.into(),
            Some(&token_guard),
            Some(&swap_guard),
            Some(&balances_guard),
            trace,
        )?;
        Ok(TokenPairSwapGuard::new(
            trace_guard,
            balances_guard,
            token_guard,
            swap_guard,
            &mut self.token_pairs,
            self.business_data.fee_to,
        ))
    }
}
