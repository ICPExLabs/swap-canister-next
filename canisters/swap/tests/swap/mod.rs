// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{
    self, CandidType, Decode, Deserialize, Encode, Principal, decode_args, encode_args, encode_one,
    utils::ArgumentEncoder,
};
use pocket_ic::RejectResponse;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct InitArg {
    pub maintainers: Option<Vec<Principal>>,
    pub schedule: Option<candid::Nat>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum InitArgs {
    V0(InitArg),
    V1(InitArg),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum Amm {
    #[serde(rename = "swap_v2_1%")]
    SwapV2H1,
    #[serde(rename = "swap_v2_0.01%")]
    SwapV2M100,
    #[serde(rename = "swap_v2_0.05%")]
    SwapV2M500,
    #[serde(rename = "swap_v2_0.3%")]
    SwapV2T3,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPair {
    pub token0: Principal,
    pub token1: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairAmm {
    pub amm: Amm,
    pub pair: TokenPair,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PairRemove {
    pub pa: TokenPairAmm,
    pub remover: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PairSwapToken {
    pub to: Account,
    pub amm: Amm,
    pub token_a: Principal,
    pub token_b: Principal,
    pub from: Account,
    pub amount_a: candid::Nat,
    pub amount_b: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct BurnFee {
    pub fee: candid::Nat,
    pub fee_to: Account,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapV2BurnToken {
    pub pa: TokenPairAmm,
    pub to: Account,
    pub fee: Option<BurnFee>,
    pub token: Principal,
    pub from: Account,
    pub amount0: candid::Nat,
    pub amount1: candid::Nat,
    pub token0: Principal,
    pub token1: Principal,
    pub amount: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapV2MintToken {
    pub pa: TokenPairAmm,
    pub to: Account,
    pub token: Principal,
    pub from: Account,
    pub amount0: candid::Nat,
    pub amount1: candid::Nat,
    pub token0: Principal,
    pub token1: Principal,
    pub amount: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapV2MintFeeToken {
    pub pa: TokenPairAmm,
    pub to: Account,
    pub token: Principal,
    pub amount: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapV2State {
    pub pa: TokenPairAmm,
    pub price_cumulative_exponent: u8,
    pub reserve0: candid::Nat,
    pub reserve1: candid::Nat,
    pub price0_cumulative: candid::Nat,
    pub supply: candid::Nat,
    pub block_timestamp: u64,
    pub price1_cumulative: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapV2TransferToken {
    pub pa: TokenPairAmm,
    pub to: Account,
    pub fee: Option<BurnFee>,
    pub token: Principal,
    pub from: Account,
    pub amount: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum SwapV2Operation {
    #[serde(rename = "burn")]
    Burn(SwapV2BurnToken),
    #[serde(rename = "mint")]
    Mint(SwapV2MintToken),
    #[serde(rename = "mint_fee")]
    MintFee(SwapV2MintFeeToken),
    #[serde(rename = "state")]
    State(SwapV2State),
    #[serde(rename = "transfer")]
    Transfer(SwapV2TransferToken),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PairCreate {
    pub pa: TokenPairAmm,
    pub creator: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum PairOperation {
    #[serde(rename = "remove")]
    Remove(PairRemove),
    #[serde(rename = "swap")]
    Swap(PairSwapToken),
    #[serde(rename = "swap_v2")]
    SwapV2(SwapV2Operation),
    #[serde(rename = "create")]
    Create(PairCreate),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum SwapOperation {
    #[serde(rename = "pair")]
    Pair(PairOperation),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapTransaction {
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub operation: SwapOperation,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapBlock {
    pub transaction: SwapTransaction,
    pub timestamp: u64,
    pub parent_hash: serde_bytes::ByteBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum QuerySwapBlockResult {
    #[serde(rename = "archive")]
    Archive(Principal),
    #[serde(rename = "block")]
    Block(SwapBlock),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct DepositToken {
    pub to: Account,
    pub token: Principal,
    pub from: Account,
    pub amount: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TransferToken {
    pub to: Account,
    pub fee: Option<BurnFee>,
    pub token: Principal,
    pub from: Account,
    pub amount: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TokenOperation {
    #[serde(rename = "withdraw")]
    Withdraw(DepositToken),
    #[serde(rename = "deposit")]
    Deposit(DepositToken),
    #[serde(rename = "transfer")]
    Transfer(TransferToken),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenTransaction {
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub operation: TokenOperation,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenBlock {
    pub transaction: TokenTransaction,
    pub timestamp: u64,
    pub parent_hash: serde_bytes::ByteBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum QueryTokenBlockResult {
    #[serde(rename = "archive")]
    Archive(Principal),
    #[serde(rename = "block")]
    Block(TokenBlock),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct FeeTo {
    pub token_fee_to: Option<Account>,
    pub swap_fee_to: Option<Account>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct FeeToView {
    pub token_fee_to: bool,
    pub swap_fee_to: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct MaintainArchives {
    pub recharged: Vec<(Principal, candid::Nat)>,
    pub checking_interval_ns: u64,
    pub recharge_cycles: u64,
    pub min_cycles_threshold: u64,
    pub last_checked_timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct MaintainArchivesConfig {
    pub checking_interval_ns: u64,
    pub recharge_cycles: u64,
    pub min_cycles_threshold: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct NextArchiveCanisterConfig {
    pub maintainers: Option<Vec<Principal>>,
    pub max_memory_size_bytes: Option<u64>,
    pub max_length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum BlockChainArgs {
    BlockQuery(u64),
    WasmModuleQuery,
    CurrentArchivingMaxLengthUpdate(u64),
    ArchivedCanisterMaxMemorySizeBytesUpdate {
        canister_id: Principal,
        max_memory_size_bytes: u64,
    },
    NextArchiveCanisterConfigUpdate(NextArchiveCanisterConfig),
    BlocksPush,
    CachedBlockQuery,
    ArchivedCanisterMaintainersUpdate {
        maintainers: Option<Vec<Principal>>,
        canister_id: Principal,
    },
    WasmModuleUpdate(serde_bytes::ByteBuf),
    BlockChainQuery,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct CurrentArchiving {
    pub canister_id: Principal,
    pub length: u64,
    pub max_length: u64,
    pub block_height_offset: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct ArchivedBlocks {
    pub canister_id: Principal,
    pub length: u64,
    pub block_height_offset: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct BlockChainView {
    pub current_archiving: Option<CurrentArchiving>,
    pub latest_block_hash: serde_bytes::ByteBuf,
    pub archive_config: NextArchiveCanisterConfig,
    pub next_block_index: u64,
    pub archived: Vec<ArchivedBlocks>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PushBlocks {
    pub block_height_start: u64,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum SwapBlockResponse {
    CachedBlock(Option<(u64, u64)>),
    BlockChain(BlockChainView),
    ArchivedCanisterMaintainers,
    CurrentArchivingMaxLength(Option<CurrentArchiving>),
    NextArchiveCanisterConfig(NextArchiveCanisterConfig),
    Block(QuerySwapBlockResult),
    WasmModule(Option<serde_bytes::ByteBuf>),
    BlocksPush(Option<PushBlocks>),
    ArchivedCanisterMaxMemorySizeBytes,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TransferError {
    GenericError { message: String, error_code: candid::Nat },
    TemporarilyUnavailable,
    BadBurn { min_burn_amount: candid::Nat },
    Duplicate { duplicate_of: candid::Nat },
    BadFee { expected_fee: candid::Nat },
    CreatedInFuture { ledger_time: u64 },
    TooOld,
    InsufficientFunds { balance: candid::Nat },
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenAccount {
    pub token: Principal,
    pub account: Account,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TransferFromError {
    GenericError { message: String, error_code: candid::Nat },
    TemporarilyUnavailable,
    InsufficientAllowance { allowance: candid::Nat },
    BadBurn { min_burn_amount: candid::Nat },
    Duplicate { duplicate_of: candid::Nat },
    BadFee { expected_fee: candid::Nat },
    CreatedInFuture { ledger_time: u64 },
    TooOld,
    InsufficientFunds { balance: candid::Nat },
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum BusinessError {
    InvalidTokenPair(Principal, Principal),
    TokenBlockChainLocked,
    TransferError(TransferError),
    NotSupportedToken(Principal),
    Swap(String),
    TokenPairAmmNotExist(TokenPairAmm),
    TokenPairAmmStillAlive(TokenPairAmm),
    TokenAccountsLocked(Vec<TokenAccount>),
    SystemError(String),
    MemoTooLong,
    InsufficientBalance { token: Principal, balance: candid::Nat },
    TokenPairAmmExist(TokenPairAmm),
    RequestTraceLocked(String),
    InvalidCreated { created: u64, system: u64 },
    InvalidAmm(String),
    InvalidTransferFee { fee: candid::Nat, token: Principal },
    SwapBlockChainLocked,
    TokenBlockChainError(String),
    TransferFromError(TransferFromError),
    TokenAccountsUnlocked(Vec<TokenAccount>),
    FrozenToken(Principal),
    NotOwner(Principal),
    BadTransferFee { expected_fee: candid::Nat },
    SwapBlockChainError(String),
    CallCanisterError(String),
    Liquidity(String),
    Expired { deadline: u64, system: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum SwapBlockResult {
    Ok(SwapBlockResponse),
    Err(BusinessError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TokenBlockResponse {
    CachedBlock(Option<(u64, u64)>),
    BlockChain(BlockChainView),
    ArchivedCanisterMaintainers,
    CurrentArchivingMaxLength(Option<CurrentArchiving>),
    NextArchiveCanisterConfig(NextArchiveCanisterConfig),
    Block(QueryTokenBlockResult),
    WasmModule(Option<serde_bytes::ByteBuf>),
    BlocksPush(Option<PushBlocks>),
    ArchivedCanisterMaxMemorySizeBytes,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TokenBlockResult {
    Ok(TokenBlockResponse),
    Err(BusinessError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenInfo {
    pub fee: candid::Nat,
    pub decimals: u8,
    pub name: String,
    pub canister_id: Principal,
    pub is_lp_token: bool,
    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenFrozenArg {
    pub token: Principal,
    pub frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum QueryBlockResult {
    #[serde(rename = "archive")]
    Archive(Principal),
    #[serde(rename = "block")]
    Block(serde_bytes::ByteBuf),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairPool {
    pub amm: String,
    pub token0: Principal,
    pub token1: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairCreateOrRemoveArgs {
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub pool: TokenPairPool,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct OuterLp {
    pub fee: candid::Nat,
    pub decimals: u8,
    pub token_canister_id: Principal,
    pub minimum_liquidity: candid::Nat,
    pub total_supply: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct InnerLp {
    pub fee: candid::Nat,
    pub decimals: u8,
    pub dummy_canister_id: Principal,
    pub minimum_liquidity: candid::Nat,
    pub total_supply: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum PoolLp {
    #[serde(rename = "outer")]
    Outer(OuterLp),
    #[serde(rename = "inner")]
    Inner(InnerLp),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapV2MarketMakerView {
    pub lp: PoolLp,
    pub price_cumulative_exponent: u8,
    pub block_timestamp_last: u64,
    pub reserve0: String,
    pub reserve1: String,
    pub subaccount: String,
    pub price1_cumulative_last: String,
    pub token0: String,
    pub token1: String,
    pub fee_rate: String,
    pub k_last: String,
    pub protocol_fee: Option<String>,
    pub price0_cumulative_last: String,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum MarketMakerView {
    #[serde(rename = "swap_v2")]
    SwapV2(SwapV2MarketMakerView),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TokenPairCreateOrRemoveResult {
    Ok(MarketMakerView),
    Err(BusinessError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapTokenPair {
    pub amm: String,
    pub token: (Principal, Principal),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairLiquidityAddArgs {
    pub to: Account,
    pub created: Option<u64>,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub deadline: Option<u64>,
    pub amount_desired: (candid::Nat, candid::Nat),
    pub amount_min: (candid::Nat, candid::Nat),
    pub swap_pair: SwapTokenPair,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairLiquidityAddSuccess {
    pub liquidity: candid::Nat,
    pub amount: (candid::Nat, candid::Nat),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TokenPairLiquidityAddResult {
    Ok(TokenPairLiquidityAddSuccess),
    Err(BusinessError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairLiquidityRemoveArgs {
    pub to: Account,
    pub created: Option<u64>,
    pub liquidity_without_fee: candid::Nat,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub deadline: Option<u64>,
    pub amount_min: (candid::Nat, candid::Nat),
    pub swap_pair: SwapTokenPair,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairLiquidityRemoveSuccess {
    pub amount: (candid::Nat, candid::Nat),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TokenPairLiquidityRemoveResult {
    Ok(TokenPairLiquidityRemoveSuccess),
    Err(BusinessError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TokenChangedResult {
    Ok(candid::Nat),
    Err(BusinessError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum ManyTokenChangedResult {
    Ok(Vec<TokenChangedResult>),
    Err(BusinessError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairSwapByLoanArgs {
    pub to: Account,
    pub created: Option<u64>,
    pub from: Account,
    pub loan: candid::Nat,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub path: Vec<SwapTokenPair>,
    pub deadline: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairSwapTokensSuccess {
    pub amounts: Vec<candid::Nat>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum TokenPairSwapTokensResult {
    Ok(TokenPairSwapTokensSuccess),
    Err(BusinessError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairSwapExactTokensForTokensArgs {
    pub to: Account,
    pub created: Option<u64>,
    pub amount_out_min: candid::Nat,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub path: Vec<SwapTokenPair>,
    pub deadline: Option<u64>,
    pub amount_in: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairSwapTokensForExactTokensArgs {
    pub to: Account,
    pub created: Option<u64>,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub path: Vec<SwapTokenPair>,
    pub deadline: Option<u64>,
    pub amount_out: candid::Nat,
    pub amount_in_max: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairSwapWithDepositAndWithdrawArgs {
    pub to: Account,
    pub created: Option<u64>,
    pub amount_out_min: candid::Nat,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub path: Vec<SwapTokenPair>,
    pub deadline: Option<u64>,
    pub deposit_amount_without_fee: candid::Nat,
    pub withdraw_fee: Option<candid::Nat>,
    pub deposit_fee: Option<candid::Nat>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapRatio {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct SwapV2MarketMaker {
    pub lp: PoolLp,
    pub price_cumulative_exponent: u8,
    pub block_timestamp_last: u64,
    pub reserve0: candid::Nat,
    pub reserve1: candid::Nat,
    pub subaccount: serde_bytes::ByteBuf,
    pub price1_cumulative_last: candid::Nat,
    pub token0: Principal,
    pub token1: Principal,
    pub fee_rate: SwapRatio,
    pub k_last: candid::Nat,
    pub protocol_fee: Option<SwapRatio>,
    pub price0_cumulative_last: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum MarketMaker {
    #[serde(rename = "swap_v2")]
    SwapV2(SwapV2MarketMaker),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PauseReason {
    pub timestamp_nanos: candid::Int,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum Permission {
    Permitted(String),
    Forbidden(String),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum PermissionUpdatedArg {
    UpdateRolePermission(String, Option<Vec<String>>),
    UpdateUserPermission(Principal, Option<Vec<String>>),
    UpdateUserRole(Principal, Option<Vec<String>>),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenDepositArgWithMeta {
    pub arg: DepositToken,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PairCreateArgWithMeta {
    pub arg: TokenPairAmm,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenCustomRemoveArgWithMeta {
    pub arg: Principal,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairSwapExactTokensForTokensArg {
    pub to: Account,
    pub pas: Vec<TokenPairAmm>,
    pub self_canister: Principal,
    pub amount_out_min: candid::Nat,
    pub from: Account,
    pub path: Vec<SwapTokenPair>,
    pub amount_in: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PairSwapExactTokensForTokensArgWithMeta {
    pub arg: TokenPairSwapExactTokensForTokensArg,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairLiquidityAddArg {
    pub pa: TokenPairAmm,
    pub to: Account,
    pub amount_a_min: candid::Nat,
    pub token_a: Principal,
    pub token_b: Principal,
    pub self_canister: Principal,
    pub from: Account,
    pub amount_b_desired: candid::Nat,
    pub amount_a_desired: candid::Nat,
    pub amount_b_min: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PairLiquidityAddArgWithMeta {
    pub arg: TokenPairLiquidityAddArg,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenCustomPutArgWithMeta {
    pub arg: TokenInfo,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenTransferArgWithMeta {
    pub arg: TransferToken,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairSwapByLoanArg {
    pub to: Account,
    pub pas: Vec<TokenPairAmm>,
    pub self_canister: Principal,
    pub from: Account,
    pub loan: candid::Nat,
    pub path: Vec<SwapTokenPair>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PairSwapByLoanArgWithMeta {
    pub arg: TokenPairSwapByLoanArg,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairLiquidityRemoveArg {
    pub pa: TokenPairAmm,
    pub to: Account,
    pub fee: Option<BurnFee>,
    pub amount_a_min: candid::Nat,
    pub token_a: Principal,
    pub token_b: Principal,
    pub self_canister: Principal,
    pub liquidity_without_fee: candid::Nat,
    pub from: Account,
    pub amount_b_min: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PairLiquidityRemoveArgWithMeta {
    pub arg: TokenPairLiquidityRemoveArg,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenPairSwapTokensForExactTokensArg {
    pub to: Account,
    pub pas: Vec<TokenPairAmm>,
    pub self_canister: Principal,
    pub from: Account,
    pub path: Vec<SwapTokenPair>,
    pub amount_out: candid::Nat,
    pub amount_in_max: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct PairSwapTokensForExactTokensArgWithMeta {
    pub arg: TokenPairSwapTokensForExactTokensArg,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenFrozenArgWithMeta {
    pub arg: TokenFrozenArg,
    pub now: u64,
    pub created: Option<u64>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub caller: Principal,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum RequestArgs {
    #[serde(rename = "token_block_push")]
    TokenBlockPush,
    #[serde(rename = "token_deposit")]
    TokenDeposit(TokenDepositArgWithMeta),
    #[serde(rename = "pair_create")]
    PairCreate(PairCreateArgWithMeta),
    #[serde(rename = "token_custom_remove")]
    TokenCustomRemove(TokenCustomRemoveArgWithMeta),
    #[serde(rename = "canisters_maintaining")]
    CanistersMaintaining,
    #[serde(rename = "pair_swap_exact_tokens_for_tokens")]
    PairSwapExactTokensForTokens(PairSwapExactTokensForTokensArgWithMeta),
    #[serde(rename = "pair_liquidity_add")]
    PairLiquidityAdd(PairLiquidityAddArgWithMeta),
    #[serde(rename = "token_custom_put")]
    TokenCustomPut(TokenCustomPutArgWithMeta),
    #[serde(rename = "token_transfer")]
    TokenTransfer(TokenTransferArgWithMeta),
    #[serde(rename = "pair_swap_by_loan")]
    PairSwapByLoan(PairSwapByLoanArgWithMeta),
    #[serde(rename = "pair_liquidity_remove")]
    PairLiquidityRemove(PairLiquidityRemoveArgWithMeta),
    #[serde(rename = "pair_swap_tokens_for_exact_tokens")]
    PairSwapTokensForExactTokens(PairSwapTokensForExactTokensArgWithMeta),
    #[serde(rename = "pair_remove")]
    PairRemove(PairCreateArgWithMeta),
    #[serde(rename = "swap_block_push")]
    SwapBlockPush,
    #[serde(rename = "token_withdraw")]
    TokenWithdraw(TokenDepositArgWithMeta),
    #[serde(rename = "token_frozen")]
    TokenFrozen(TokenFrozenArgWithMeta),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum RequestTraceResult {
    #[serde(rename = "ok")]
    Ok(String),
    #[serde(rename = "err")]
    Err(String),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct RequestTraceDone {
    pub result: RequestTraceResult,
    pub done: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct BusinessLocks {
    pub token: Option<bool>,
    pub swap: Option<bool>,
    pub balances: Option<Vec<TokenAccount>>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct RequestTrace {
    pub created: u64,
    pub args: RequestArgs,
    pub done: Option<RequestTraceDone>,
    pub traces: Vec<(u64, String)>,
    pub locks: BusinessLocks,
    pub index: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenDepositArgs {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub created: Option<u64>,
    pub token: Principal,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub deposit_amount_without_fee: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenTransferArgs {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub created: Option<u64>,
    pub token: Principal,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub transfer_amount_without_fee: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenWithdrawArgs {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub created: Option<u64>,
    pub token: Principal,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub withdraw_amount_without_fee: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenWithdrawManyArgs {
    pub args: Vec<TokenWithdrawArgs>,
}

#[derive(Clone, Copy)]
pub struct PocketedCanisterId<'a> {
    pub canister_id: Principal,
    pub pic: &'a pocket_ic::PocketIc,
}

impl<'a> PocketedCanisterId<'a> {
    pub fn new(canister_id: Principal, pic: &'a pocket_ic::PocketIc) -> Self {
        Self { canister_id, pic }
    }
    pub fn sender(&self, sender: Principal) -> Service<'a> {
        Service { pocket: *self, sender }
    }
}

type Result<R> = std::result::Result<R, RejectResponse>;
pub struct Service<'a> {
    pub pocket: PocketedCanisterId<'a>,
    pub sender: Principal,
}
impl Service<'_> {
    fn query_call<R: CandidType + DeserializeOwned>(&self, method: &str, payload: Vec<u8>) -> Result<R> {
        let response = self
            .pocket
            .pic
            .query_call(self.pocket.canister_id, self.sender, method, payload)?;
        let result = Decode!(response.as_slice(), R).unwrap();
        Ok(result)
    }
    fn update_call<R: CandidType + DeserializeOwned>(&self, method: &str, payload: Vec<u8>) -> Result<R> {
        let response = self
            .pocket
            .pic
            .update_call(self.pocket.canister_id, self.sender, method, payload)?;
        let result = Decode!(response.as_slice(), R).unwrap();
        Ok(result)
    }

    // ======================= common apis =======================

    pub fn get_candid_interface_tmp_hack(&self) -> Result<String> {
        self.query_call("__get_candid_interface_tmp_hack", Encode!(&()).unwrap())
    }
    pub fn version(&self) -> Result<u32> {
        self.query_call("version", Encode!(&()).unwrap())
    }
    pub fn wallet_balance(&self) -> Result<candid::Nat> {
        self.query_call("wallet_balance", Encode!(&()).unwrap())
    }
    pub fn wallet_receive(&self) -> Result<candid::Nat> {
        self.query_call("wallet_receive", Encode!(&()).unwrap())
    }

    // ======================= business apis =======================

    pub fn block_swap_get(&self, arg0: u64) -> Result<QuerySwapBlockResult> {
        self.query_call("block_swap_get", encode_one(arg0).unwrap())
    }
    pub fn block_token_get(&self, arg0: u64) -> Result<QueryTokenBlockResult> {
        self.query_call("block_token_get", encode_one(arg0).unwrap())
    }
    pub fn config_fee_to_query(&self) -> Result<FeeTo> {
        self.query_call("config_fee_to_query", Encode!(&()).unwrap())
    }
    pub fn config_fee_to_replace(&self, arg0: FeeTo) -> Result<FeeTo> {
        self.query_call("config_fee_to_replace", encode_one(arg0).unwrap())
    }
    pub fn config_fee_to_view_query(&self) -> Result<FeeToView> {
        self.query_call("config_fee_to_view_query", Encode!(&()).unwrap())
    }
    pub fn config_maintain_archives_query(&self) -> Result<MaintainArchives> {
        self.query_call("config_maintain_archives_query", Encode!(&()).unwrap())
    }
    pub fn config_maintain_archives_set(&self, arg0: MaintainArchivesConfig) -> Result<()> {
        self.query_call("config_maintain_archives_set", encode_one(arg0).unwrap())
    }
    pub fn config_swap_block_chain_query(&self, arg0: BlockChainArgs) -> Result<SwapBlockResult> {
        self.query_call("config_swap_block_chain_query", encode_one(arg0).unwrap())
    }
    pub fn config_swap_block_chain_update(&self, arg0: BlockChainArgs) -> Result<SwapBlockResult> {
        self.query_call("config_swap_block_chain_update", encode_one(arg0).unwrap())
    }
    pub fn config_token_block_chain_query(&self, arg0: BlockChainArgs) -> Result<TokenBlockResult> {
        self.query_call("config_token_block_chain_query", encode_one(arg0).unwrap())
    }
    pub fn config_token_block_chain_update(&self, arg0: BlockChainArgs) -> Result<TokenBlockResult> {
        self.query_call("config_token_block_chain_update", encode_one(arg0).unwrap())
    }
    pub fn config_token_custom_put(&self, arg0: TokenInfo) -> Result<()> {
        self.query_call("config_token_custom_put", encode_one(arg0).unwrap())
    }
    pub fn config_token_custom_query(&self) -> Result<Vec<TokenInfo>> {
        self.query_call("config_token_custom_query", Encode!(&()).unwrap())
    }
    pub fn config_token_custom_remove(&self, arg0: Principal) -> Result<Option<TokenInfo>> {
        self.query_call("config_token_custom_remove", encode_one(arg0).unwrap())
    }
    pub fn config_token_frozen(&self, arg0: TokenFrozenArg) -> Result<()> {
        self.query_call("config_token_frozen", encode_one(arg0).unwrap())
    }
    pub fn config_token_frozen_query(&self) -> Result<Vec<Principal>> {
        self.query_call("config_token_frozen_query", Encode!(&()).unwrap())
    }
    pub fn encoded_blocks_swap_get(&self, arg0: u64) -> Result<Vec<(u64, QueryBlockResult)>> {
        self.query_call("encoded_blocks_swap_get", encode_one(arg0).unwrap())
    }
    pub fn encoded_blocks_token_get(&self, arg0: u64) -> Result<Vec<(u64, QueryBlockResult)>> {
        self.query_call("encoded_blocks_token_get", encode_one(arg0).unwrap())
    }
    pub fn pair_create(&self, arg0: TokenPairCreateOrRemoveArgs) -> Result<TokenPairCreateOrRemoveResult> {
        self.update_call("pair_create", encode_one(arg0).unwrap())
    }
    pub fn pair_liquidity_add(
        &self,
        arg0: TokenPairLiquidityAddArgs,
        arg1: Option<u8>,
    ) -> Result<TokenPairLiquidityAddResult> {
        self.update_call("pair_liquidity_add", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn pair_liquidity_remove(
        &self,
        arg0: TokenPairLiquidityRemoveArgs,
        arg1: Option<u8>,
    ) -> Result<TokenPairLiquidityRemoveResult> {
        self.update_call("pair_liquidity_remove", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn pair_liquidity_remove_and_withdraw(
        &self,
        arg0: TokenPairLiquidityRemoveArgs,
    ) -> Result<(TokenPairLiquidityRemoveResult, Option<ManyTokenChangedResult>)> {
        self.update_call("pair_liquidity_remove_and_withdraw", encode_one(arg0).unwrap())
    }
    pub fn pair_liquidity_remove_and_withdraw_async(
        &self,
        arg0: TokenPairLiquidityRemoveArgs,
    ) -> Result<(TokenPairLiquidityRemoveResult, Option<ManyTokenChangedResult>)> {
        self.update_call("pair_liquidity_remove_and_withdraw_async", encode_one(arg0).unwrap())
    }
    pub fn pair_query(&self, arg0: TokenPairPool) -> Result<Option<MarketMakerView>> {
        self.query_call("pair_query", encode_one(arg0).unwrap())
    }
    pub fn pair_remove(&self, arg0: TokenPairCreateOrRemoveArgs) -> Result<TokenPairCreateOrRemoveResult> {
        self.update_call("pair_remove", encode_one(arg0).unwrap())
    }
    pub fn pair_swap_by_loan(
        &self,
        arg0: TokenPairSwapByLoanArgs,
        arg1: Option<u8>,
    ) -> Result<TokenPairSwapTokensResult> {
        self.update_call("pair_swap_by_loan", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn pair_swap_exact_tokens_for_tokens(
        &self,
        arg0: TokenPairSwapExactTokensForTokensArgs,
        arg1: Option<u8>,
    ) -> Result<TokenPairSwapTokensResult> {
        self.update_call(
            "pair_swap_exact_tokens_for_tokens",
            encode_args((&arg0, &arg1)).unwrap(),
        )
    }
    pub fn pair_swap_tokens_for_exact_tokens(
        &self,
        arg0: TokenPairSwapTokensForExactTokensArgs,
        arg1: Option<u8>,
    ) -> Result<TokenPairSwapTokensResult> {
        self.update_call(
            "pair_swap_tokens_for_exact_tokens",
            encode_args((&arg0, &arg1)).unwrap(),
        )
    }
    pub fn pair_swap_with_deposit_and_withdraw(
        &self,
        arg0: TokenPairSwapWithDepositAndWithdrawArgs,
    ) -> Result<(
        TokenChangedResult,
        Option<TokenPairSwapTokensResult>,
        Option<TokenChangedResult>,
    )> {
        self.update_call("pair_swap_with_deposit_and_withdraw", encode_one(arg0).unwrap())
    }
    pub fn pair_swap_with_deposit_and_withdraw_async(
        &self,
        arg0: TokenPairSwapWithDepositAndWithdrawArgs,
    ) -> Result<(
        TokenChangedResult,
        Option<TokenPairSwapTokensResult>,
        Option<TokenChangedResult>,
    )> {
        self.update_call("pair_swap_with_deposit_and_withdraw_async", encode_one(arg0).unwrap())
    }
    pub fn pairs_query(&self) -> Result<Vec<(TokenPairPool, MarketMakerView)>> {
        self.query_call("pairs_query", Encode!(&()).unwrap())
    }
    pub fn pairs_query_raw(&self) -> Result<Vec<(TokenPairPool, MarketMaker)>> {
        self.query_call("pairs_query_raw", Encode!(&()).unwrap())
    }
    pub fn pause_query(&self) -> Result<bool> {
        self.query_call("pause_query", Encode!(&()).unwrap())
    }
    pub fn pause_query_reason(&self) -> Result<Option<PauseReason>> {
        self.query_call("pause_query_reason", Encode!(&()).unwrap())
    }
    pub fn pause_replace(&self, arg0: Option<String>) -> Result<()> {
        self.update_call("pause_replace", encode_one(arg0).unwrap())
    }
    pub fn permission_all(&self) -> Result<Vec<Permission>> {
        self.query_call("permission_all", Encode!(&()).unwrap())
    }
    pub fn permission_assigned_by_user(&self, arg0: Principal) -> Result<Option<Vec<Permission>>> {
        self.query_call("permission_assigned_by_user", encode_one(arg0).unwrap())
    }
    pub fn permission_assigned_query(&self) -> Result<Option<Vec<Permission>>> {
        self.query_call("permission_assigned_query", Encode!(&()).unwrap())
    }
    pub fn permission_find_by_user(&self, arg0: Principal) -> Result<Vec<String>> {
        self.query_call("permission_find_by_user", encode_one(arg0).unwrap())
    }
    pub fn permission_query(&self) -> Result<Vec<String>> {
        self.query_call("permission_query", Encode!(&()).unwrap())
    }
    pub fn permission_roles_all(&self) -> Result<Vec<(String, Vec<Permission>)>> {
        self.query_call("permission_roles_all", Encode!(&()).unwrap())
    }
    pub fn permission_roles_by_user(&self, arg0: Principal) -> Result<Option<Vec<String>>> {
        self.query_call("permission_roles_by_user", encode_one(arg0).unwrap())
    }
    pub fn permission_roles_query(&self) -> Result<Option<Vec<String>>> {
        self.query_call("permission_roles_query", Encode!(&()).unwrap())
    }
    pub fn permission_update(&self, arg0: Vec<PermissionUpdatedArg>) -> Result<()> {
        self.update_call("permission_update", encode_one(arg0).unwrap())
    }
    pub fn request_trace_get(&self, arg0: u64) -> Result<Option<RequestTrace>> {
        self.query_call("request_trace_get", encode_one(arg0).unwrap())
    }
    pub fn request_trace_index_get(&self) -> Result<(u64, u64)> {
        let response = self.pocket.pic.query_call(
            self.pocket.canister_id,
            self.sender,
            "request_trace_index_get",
            Encode!(&()).unwrap(),
        )?;
        let result = decode_args(response.as_slice()).unwrap();
        Ok(result)
        // self.query_call("request_trace_index_get", Encode!(&()).unwrap())
    }
    pub fn request_trace_remove(&self, arg0: u64) -> Result<Option<RequestTrace>> {
        self.update_call("request_trace_remove", encode_one(arg0).unwrap())
    }
    pub fn request_traces_get(&self, arg0: u64, arg1: u64) -> Result<Vec<Option<RequestTrace>>> {
        self.query_call("request_traces_get", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn request_traces_remove(&self, arg0: u64, arg1: u64) -> Result<Vec<Option<RequestTrace>>> {
        self.update_call("request_traces_remove", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn schedule_find(&self) -> Result<Option<u64>> {
        self.query_call("schedule_find", Encode!(&()).unwrap())
    }
    pub fn schedule_replace(&self, arg0: Option<u64>) -> Result<()> {
        self.update_call("schedule_replace", encode_one(arg0).unwrap())
    }
    pub fn schedule_trigger(&self) -> Result<()> {
        self.update_call("schedule_trigger", Encode!(&()).unwrap())
    }
    pub fn test_config_swap_current_archiving_replace(
        &self,
        arg0: CurrentArchiving,
    ) -> Result<Option<CurrentArchiving>> {
        self.update_call("test_config_swap_current_archiving_replace", encode_one(arg0).unwrap())
    }
    pub fn test_config_token_current_archiving_replace(
        &self,
        arg0: CurrentArchiving,
    ) -> Result<Option<CurrentArchiving>> {
        self.update_call("test_config_token_current_archiving_replace", encode_one(arg0).unwrap())
    }
    pub fn test_set_controller(&self, arg0: Principal) -> Result<()> {
        self.update_call("test_set_controller", encode_one(arg0).unwrap())
    }
    pub fn test_withdraw_all_tokens(&self, arg0: Vec<Principal>) -> Result<Vec<String>> {
        self.update_call("test_withdraw_all_tokens", encode_one(arg0).unwrap())
    }
    pub fn token_balance(&self, arg0: Principal, arg1: Option<serde_bytes::ByteBuf>) -> Result<candid::Nat> {
        self.query_call("token_balance", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn token_balance_by(&self, arg0: Principal, arg1: Account) -> Result<candid::Nat> {
        self.query_call("token_balance_by", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn token_balance_of(&self, arg0: Principal, arg1: Account) -> Result<candid::Nat> {
        self.query_call("token_balance_of", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn token_deposit(&self, arg0: TokenDepositArgs, arg1: Option<u8>) -> Result<TokenChangedResult> {
        self.update_call("token_deposit", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn token_query(&self, arg0: Principal) -> Result<Option<TokenInfo>> {
        self.query_call("token_query", encode_one(arg0).unwrap())
    }
    pub fn token_transfer(&self, arg0: TokenTransferArgs, arg1: Option<u8>) -> Result<TokenChangedResult> {
        self.update_call("token_transfer", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn token_withdraw(&self, arg0: TokenWithdrawArgs, arg1: Option<u8>) -> Result<TokenChangedResult> {
        self.update_call("token_withdraw", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn token_withdraw_many(&self, arg0: TokenWithdrawManyArgs, arg1: Option<u8>) -> Result<ManyTokenChangedResult> {
        self.update_call("token_withdraw_many", encode_args((&arg0, &arg1)).unwrap())
    }
    pub fn tokens_balance(&self, arg0: Option<serde_bytes::ByteBuf>) -> Result<Vec<(Principal, candid::Nat)>> {
        self.query_call("tokens_balance", encode_one(arg0).unwrap())
    }
    pub fn tokens_balance_by(&self, arg0: Account) -> Result<Vec<(Principal, candid::Nat)>> {
        self.query_call("tokens_balance_by", encode_one(arg0).unwrap())
    }
    pub fn tokens_balance_of(&self, arg0: Account) -> Result<Vec<(Principal, candid::Nat)>> {
        self.query_call("tokens_balance_of", encode_one(arg0).unwrap())
    }
    pub fn tokens_query(&self) -> Result<Vec<TokenInfo>> {
        self.query_call("tokens_query", Encode!(&()).unwrap())
    }
    pub fn updated(&self) -> Result<u64> {
        self.query_call("updated", Encode!(&()).unwrap())
    }
}
