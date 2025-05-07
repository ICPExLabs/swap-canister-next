// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{
    self, CandidType, Decode, Deserialize, Encode, Principal, encode_args, encode_one, utils::ArgumentEncoder,
};
use pocket_ic::RejectResponse;
use serde::de::DeserializeOwned;

#[derive(CandidType, Deserialize)]
pub struct ChangeArchiveOptions {
    pub num_blocks_to_archive: Option<u64>,
    pub max_transactions_per_response: Option<u64>,
    pub trigger_threshold: Option<u64>,
    pub more_controller_ids: Option<Vec<Principal>>,
    pub max_message_size_bytes: Option<u64>,
    pub cycles_for_archive_creation: Option<u64>,
    pub node_max_memory_size_bytes: Option<u64>,
    pub controller_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub enum MetadataValue {
    Int(candid::Int),
    Nat(candid::Nat),
    Blob(serde_bytes::ByteBuf),
    Text(String),
}

#[derive(CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub enum ChangeFeeCollector {
    SetTo(Account),
    Unset,
}

#[derive(CandidType, Deserialize)]
pub struct FeatureFlags {
    pub icrc2: bool,
}

#[derive(CandidType, Deserialize)]
pub struct UpgradeArgs {
    pub change_archive_options: Option<ChangeArchiveOptions>,
    pub token_symbol: Option<String>,
    pub transfer_fee: Option<candid::Nat>,
    pub metadata: Option<Vec<(String, MetadataValue)>>,
    pub change_fee_collector: Option<ChangeFeeCollector>,
    pub max_memo_length: Option<u16>,
    pub token_name: Option<String>,
    pub feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize)]
pub struct ArchiveOptions {
    pub num_blocks_to_archive: u64,
    pub max_transactions_per_response: Option<u64>,
    pub trigger_threshold: u64,
    pub more_controller_ids: Option<Vec<Principal>>,
    pub max_message_size_bytes: Option<u64>,
    pub cycles_for_archive_creation: Option<u64>,
    pub node_max_memory_size_bytes: Option<u64>,
    pub controller_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub decimals: Option<u8>,
    pub token_symbol: String,
    pub transfer_fee: candid::Nat,
    pub metadata: Vec<(String, MetadataValue)>,
    pub minting_account: Account,
    pub initial_balances: Vec<(Account, candid::Nat)>,
    pub fee_collector_account: Option<Account>,
    pub archive_options: ArchiveOptions,
    pub max_memo_length: Option<u16>,
    pub token_name: String,
    pub feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize)]
pub enum LedgerArgument {
    Upgrade(Option<UpgradeArgs>),
    Init(InitArgs),
}

#[derive(CandidType, Deserialize)]
pub struct ArchiveInfo {
    pub block_range_end: candid::Nat,
    pub canister_id: Principal,
    pub block_range_start: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlocksRequest {
    pub start: candid::Nat,
    pub length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum VecItem {
    Int(candid::Int),
    Map(Vec<(String, Box<Value>)>),
    Nat(candid::Nat),
    Nat64(u64),
    Blob(serde_bytes::ByteBuf),
    Text(String),
    Array(Box<Vec2>),
}

#[derive(CandidType, Deserialize)]
pub struct Vec2(Vec<VecItem>);

#[derive(CandidType, Deserialize)]
pub enum Value {
    Int(candid::Int),
    Map(Vec<(String, Box<Value>)>),
    Nat(candid::Nat),
    Nat64(u64),
    Blob(serde_bytes::ByteBuf),
    Text(String),
    Array(Box<Vec2>),
}

#[derive(CandidType, Deserialize)]
pub struct BlockRange {
    pub blocks: Vec<Box<Value>>,
}

candid::define_function!(pub ArchivedRangeCallback : (GetBlocksRequest) -> (
    BlockRange,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedRange {
    pub callback: ArchivedRangeCallback,
    pub start: candid::Nat,
    pub length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlocksResponse {
    pub certificate: Option<serde_bytes::ByteBuf>,
    pub first_index: candid::Nat,
    pub blocks: Vec<Box<Value>>,
    pub chain_length: u64,
    pub archived_blocks: Vec<ArchivedRange>,
}

#[derive(CandidType, Deserialize)]
pub struct DataCertificate {
    pub certificate: Option<serde_bytes::ByteBuf>,
    pub hash_tree: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct Burn {
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Mint {
    pub to: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Approve {
    pub fee: Option<candid::Nat>,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub expected_allowance: Option<candid::Nat>,
    pub expires_at: Option<u64>,
    pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
    pub burn: Option<Burn>,
    pub kind: String,
    pub mint: Option<Mint>,
    pub approve: Option<Approve>,
    pub timestamp: u64,
    pub transfer: Option<Transfer>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionRange {
    pub transactions: Vec<Transaction>,
}

candid::define_function!(pub ArchivedRange1Callback : (GetBlocksRequest) -> (
    TransactionRange,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedRange1 {
    pub callback: ArchivedRange1Callback,
    pub start: candid::Nat,
    pub length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsResponse {
    pub first_index: candid::Nat,
    pub log_length: candid::Nat,
    pub transactions: Vec<Transaction>,
    pub archived_transactions: Vec<ArchivedRange1>,
}

#[derive(CandidType, Deserialize)]
pub struct StandardRecord {
    pub url: String,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub from_subaccount: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
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

#[derive(CandidType, Deserialize)]
pub enum Result_ {
    Ok(candid::Nat),
    Err(TransferError),
}

#[derive(CandidType, Deserialize)]
pub struct ConsentMessageMetadata {
    pub utc_offset_minutes: Option<i16>,
    pub language: String,
}

#[derive(CandidType, Deserialize)]
pub enum DisplayMessageType {
    GenericDisplay,
    LineDisplay {
        characters_per_line: u16,
        lines_per_page: u16,
    },
}

#[derive(CandidType, Deserialize)]
pub struct ConsentMessageSpec {
    pub metadata: ConsentMessageMetadata,
    pub device_spec: Option<DisplayMessageType>,
}

#[derive(CandidType, Deserialize)]
pub struct ConsentMessageRequest {
    pub arg: serde_bytes::ByteBuf,
    pub method: String,
    pub user_preferences: ConsentMessageSpec,
}

#[derive(CandidType, Deserialize)]
pub struct LineDisplayPage {
    pub lines: Vec<String>,
}

#[derive(CandidType, Deserialize)]
pub enum ConsentMessage {
    LineDisplayMessage { pages: Vec<LineDisplayPage> },
    GenericDisplayMessage(String),
}

#[derive(CandidType, Deserialize)]
pub struct ConsentInfo {
    pub metadata: ConsentMessageMetadata,
    pub consent_message: ConsentMessage,
}

#[derive(CandidType, Deserialize)]
pub struct ErrorInfo {
    pub description: String,
}

#[derive(CandidType, Deserialize)]
pub enum Icrc21Error {
    GenericError {
        description: String,
        error_code: candid::Nat,
    },
    InsufficientPayment(ErrorInfo),
    UnsupportedCanisterCall(ErrorInfo),
    ConsentMessageUnavailable(ErrorInfo),
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(ConsentInfo),
    Err(Icrc21Error),
}

#[derive(CandidType, Deserialize)]
pub struct AllowanceArgs {
    pub account: Account,
    pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Allowance {
    pub allowance: candid::Nat,
    pub expires_at: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct ApproveArgs {
    pub fee: Option<candid::Nat>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub from_subaccount: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub expected_allowance: Option<candid::Nat>,
    pub expires_at: Option<u64>,
    pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub enum ApproveError {
    GenericError { message: String, error_code: candid::Nat },
    TemporarilyUnavailable,
    Duplicate { duplicate_of: candid::Nat },
    BadFee { expected_fee: candid::Nat },
    AllowanceChanged { current_allowance: candid::Nat },
    CreatedInFuture { ledger_time: u64 },
    TooOld,
    Expired { ledger_time: u64 },
    InsufficientFunds { balance: candid::Nat },
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(candid::Nat),
    Err(ApproveError),
}

#[derive(CandidType, Deserialize)]
pub struct TransferFromArgs {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub spender_subaccount: Option<serde_bytes::ByteBuf>,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
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

#[derive(CandidType, Deserialize)]
pub enum Result3 {
    Ok(candid::Nat),
    Err(TransferFromError),
}

#[derive(CandidType, Deserialize)]
pub struct GetArchivesArgs {
    pub from: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct Icrc3ArchiveInfo {
    pub end: candid::Nat,
    pub canister_id: Principal,
    pub start: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum Icrc3Value {
    Int(candid::Int),
    Map(Vec<(String, Box<Icrc3Value>)>),
    Nat(candid::Nat),
    Blob(serde_bytes::ByteBuf),
    Text(String),
    Array(Vec<Box<Icrc3Value>>),
}

#[derive(CandidType, Deserialize)]
pub struct BlockWithId {
    pub id: candid::Nat,
    pub block: Box<Icrc3Value>,
}

candid::define_function!(pub ArchivedBlocksCallback : (
    Vec<GetBlocksRequest>,
  ) -> (GetBlocksResult) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedBlocks {
    pub args: Vec<GetBlocksRequest>,
    pub callback: ArchivedBlocksCallback,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlocksResult {
    pub log_length: candid::Nat,
    pub blocks: Vec<BlockWithId>,
    pub archived_blocks: Vec<ArchivedBlocks>,
}

#[derive(CandidType, Deserialize)]
pub struct Icrc3DataCertificate {
    pub certificate: serde_bytes::ByteBuf,
    pub hash_tree: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct SupportedBlockType {
    pub url: String,
    pub block_type: String,
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

    // ======================= account apis =======================

    pub fn archives(&self) -> Result<Vec<ArchiveInfo>> {
        self.query_call("archives", Encode!(&()).unwrap())
    }
    pub fn get_blocks(&self, arg0: GetBlocksRequest) -> Result<(GetBlocksResponse,)> {
        self.query_call("get_blocks", encode_one(arg0).unwrap())
    }
    pub fn get_data_certificate(&self) -> Result<(DataCertificate,)> {
        self.query_call("get_data_certificate", Encode!(&()).unwrap())
    }
    pub fn get_transactions(&self, arg0: GetBlocksRequest) -> Result<(GetTransactionsResponse,)> {
        self.query_call("get_transactions", encode_one(arg0).unwrap())
    }
    pub fn is_ledger_ready(&self) -> Result<(bool,)> {
        self.query_call("is_ledger_ready", Encode!(&()).unwrap())
    }

    // ======================= icrc1 apis =======================

    pub fn icrc_1_balance_of(&self, arg0: Account) -> Result<candid::Nat> {
        self.query_call("icrc1_balance_of", encode_one(arg0).unwrap())
    }
    pub fn icrc_1_decimals(&self) -> Result<u8> {
        self.query_call("icrc1_decimals", Encode!(&()).unwrap())
    }
    pub fn icrc_1_fee(&self) -> Result<candid::Nat> {
        self.query_call("icrc1_fee", Encode!(&()).unwrap())
    }
    pub fn icrc_1_metadata(&self) -> Result<Vec<(String, MetadataValue)>> {
        self.query_call("icrc1_metadata", Encode!(&()).unwrap())
    }
    pub fn icrc_1_minting_account(&self) -> Result<Option<Account>> {
        self.query_call("icrc1_minting_account", Encode!(&()).unwrap())
    }
    pub fn icrc_1_name(&self) -> Result<String> {
        self.query_call("icrc1_name", Encode!(&()).unwrap())
    }
    pub fn icrc_1_supported_standards(&self) -> Result<Vec<StandardRecord>> {
        self.query_call("icrc1_supported_standards", Encode!(&()).unwrap())
    }
    pub fn icrc_1_symbol(&self) -> Result<String> {
        self.query_call("icrc1_symbol", Encode!(&()).unwrap())
    }
    pub fn icrc_1_total_supply(&self) -> Result<candid::Nat> {
        self.query_call("icrc1_total_supply", Encode!(&()).unwrap())
    }
    pub fn icrc_1_transfer(&self, arg0: TransferArg) -> Result<Result_> {
        self.query_call("icrc1_transfer", encode_one(arg0).unwrap())
    }

    // ======================= icrc2 apis =======================

    pub fn icrc_2_allowance(&self, arg0: AllowanceArgs) -> Result<Allowance> {
        self.query_call("icrc2_allowance", encode_one(arg0).unwrap())
    }
    pub fn icrc_2_approve(&self, arg0: ApproveArgs) -> Result<Result2> {
        self.query_call("icrc2_approve", encode_one(arg0).unwrap())
    }
    pub fn icrc_2_transfer_from(&self, arg0: TransferFromArgs) -> Result<Result3> {
        self.query_call("icrc2_transfer_from", encode_one(arg0).unwrap())
    }

    // ======================= icrc3 apis =======================

    pub fn icrc_3_get_archives(&self, arg0: GetArchivesArgs) -> Result<Vec<Icrc3ArchiveInfo>> {
        self.query_call("icrc3_get_archives", encode_one(arg0).unwrap())
    }
    pub fn icrc_3_get_blocks(&self, arg0: Vec<GetBlocksRequest>) -> Result<GetBlocksResult> {
        self.query_call("icrc3_get_blocks", encode_one(arg0).unwrap())
    }
    pub fn icrc_3_get_tip_certificate(&self) -> Result<Option<Icrc3DataCertificate>> {
        self.query_call("icrc3_get_tip_certificate", Encode!(&()).unwrap())
    }
    pub fn icrc_3_supported_block_types(&self) -> Result<Vec<SupportedBlockType>> {
        self.query_call("icrc3_supported_block_types", Encode!(&()).unwrap())
    }

    // ======================= other apis =======================

    pub fn icrc_10_supported_standards(&self) -> Result<Vec<StandardRecord>> {
        self.query_call("icrc10_supported_standards", Encode!(&()).unwrap())
    }
    pub fn icrc_21_canister_call_consent_message(&self, arg0: ConsentMessageRequest) -> Result<Result1> {
        self.query_call("icrc21_canister_call_consent_message", encode_one(arg0).unwrap())
    }
}
