use serde::{Deserialize, Serialize};

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

// initialization parameters
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArg {
    pub maintainers: Option<Vec<UserId>>, // init maintainers or deployer
    pub schedule: Option<DurationNanos>,  // init scheduled task or not
}

// Upgrade parameters
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
    Account, Amm, AmmText, ArgWithMeta, BlockIndex, BurnFee, BusinessError, Caller, CandidBlock, DepositToken, DoHash,
    DummyCanisterId, EncodedBlock, HashOf, MarketMaker, MarketMakerView, Nat, PairCreate, PairCumulativePrice,
    PairOperation, PairSwapToken, QueryBlockResult, QuerySwapBlockResult, QueryTokenBlockResult, SelfCanister,
    SwapBlock, SwapOperation, SwapTransaction, SwapV2BurnToken, SwapV2MarketMaker, SwapV2MintFeeToken, SwapV2MintToken,
    SwapV2Operation, SwapV2TransferToken, TimestampNanos, TokenAccount, TokenBlock, TokenInfo, TokenOperation,
    TokenPair, TokenPairAmm, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccessView, TokenPairLiquidityRemoveArg,
    TokenPairPool, TokenPairSwapByLoanArg, TokenPairSwapExactTokensForTokensArg, TokenPairSwapTokensForExactTokensArg,
    TokenTransaction, TransferFee, TransferToken, UserId, WithdrawToken, display_account, proto,
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

// Data structures required by the framework
#[derive(Serialize, Deserialize, Default)]
pub struct CanisterKit {
    pub pause: Pause,             // Record maintenance status //  ? Heap memory Serialization
    pub permissions: Permissions, // Record your own permissions //  ? Heap memory Serialization
    pub schedule: Schedule,       // Record timing tasks //  ? Heap memory Serialization
}

#[derive(Serialize, Deserialize)]
pub struct BusinessData {
    pub updated: TimestampNanos,             // Record the last update time of the canister
    pub fee_to: FeeTo, // Record the agreement fee collector account, lp token transfer also requires the collection of transfer fees
    pub maintain_archives: MaintainArchives, // Maintain canister information
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

    pub request_traces: RequestTraces, // Business data, Record request steps //  ? Heap memory Serialization Stable memory
    pub token_block_chain: TokenBlockChain, // Business data, Record Token block data //  ? Heap memory Serialization Stable memory
    pub swap_block_chain: SwapBlockChain, // Business data, Record Swap block data //  ? Heap memory Serialization Stable memory

    pub token_pairs: TokenPairs, // Business data, Record transaction pair data //  ? Heap memory Serialization Stable memory
    pub token_balances: TokenBalances, // Business data, Record account balance data //  ? Heap memory Serialization Stable memory
}

impl Default for InnerState {
    fn default() -> Self {
        ic_cdk::println!("InnerState::default()");
        Self {
            canister_kit: Default::default(),

            // Business data
            business_data: Default::default(),

            request_traces: Default::default(),
            token_block_chain: Default::default(),
            swap_block_chain: Default::default(),

            token_pairs: Default::default(),
            token_balances: Default::default(),
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

impl InnerState {
    pub fn do_init(&mut self, arg: InitArg) {
        self.updated(|s| {
            let maintainers = arg.maintainers.clone().unwrap_or_else(|| {
                vec![ic_canister_kit::identity::caller()] // The default caller is the maintenance person
            });
            s.token_block_chain.set_archive_maintainers(Some(maintainers));

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
        let trace_guard =
            self.request_traces
                .be_guard(arg.into(), Some(&token_guard), None, Some(&balances_guard), trace)?;
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
