// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult;
use icrc_ledger_types::{
    icrc1::{account::Account, transfer::TransferError},
    icrc2::transfer_from::TransferFromError,
};

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
pub enum Value {
    Int(candid::Int),
    Map(Vec<(String, Box<Value>)>),
    Nat(candid::Nat),
    Nat64(u64),
    Blob(serde_bytes::ByteBuf),
    Text(String),
    Array(Vec<Value>),
}

#[derive(CandidType, Deserialize)]
pub struct BlockRange {
    pub blocks: Vec<Value>,
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
    pub blocks: Vec<Value>,
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

#[derive(Debug, CandidType, Deserialize)]
pub struct TransferArg {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub from_subaccount: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

pub type Result_ = Result<candid::Nat, TransferError>;

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
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    AllowanceChanged {
        current_allowance: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    Expired {
        ledger_time: u64,
    },
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(candid::Nat),
    Err(ApproveError),
}

#[derive(Debug, CandidType, Deserialize)]
pub struct TransferFromArgs {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub spender_subaccount: Option<serde_bytes::ByteBuf>,
    pub from: Account,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

pub type Result3 = Result<candid::Nat, TransferFromError>;

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
    Array(Vec<Icrc3Value>),
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

pub struct Service(pub Principal);
impl Service {
    pub async fn archives(&self) -> CallResult<(Vec<ArchiveInfo>,)> {
        ic_cdk::call(self.0, "archives", ()).await
    }
    pub async fn get_blocks(&self, arg0: GetBlocksRequest) -> CallResult<(GetBlocksResponse,)> {
        ic_cdk::call(self.0, "get_blocks", (arg0,)).await
    }
    pub async fn get_data_certificate(&self) -> CallResult<(DataCertificate,)> {
        ic_cdk::call(self.0, "get_data_certificate", ()).await
    }
    pub async fn get_transactions(
        &self,
        arg0: GetBlocksRequest,
    ) -> CallResult<(GetTransactionsResponse,)> {
        ic_cdk::call(self.0, "get_transactions", (arg0,)).await
    }
    pub async fn icrc_10_supported_standards(&self) -> CallResult<(Vec<StandardRecord>,)> {
        ic_cdk::call(self.0, "icrc10_supported_standards", ()).await
    }
    pub async fn icrc_1_balance_of(&self, arg0: Account) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_balance_of", (arg0,)).await
    }
    pub async fn icrc_1_decimals(&self) -> CallResult<(u8,)> {
        ic_cdk::call(self.0, "icrc1_decimals", ()).await
    }
    pub async fn icrc_1_fee(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_fee", ()).await
    }
    pub async fn icrc_1_metadata(&self) -> CallResult<(Vec<(String, MetadataValue)>,)> {
        ic_cdk::call(self.0, "icrc1_metadata", ()).await
    }
    pub async fn icrc_1_minting_account(&self) -> CallResult<(Option<Account>,)> {
        ic_cdk::call(self.0, "icrc1_minting_account", ()).await
    }
    pub async fn icrc_1_name(&self) -> CallResult<(String,)> {
        ic_cdk::call(self.0, "icrc1_name", ()).await
    }
    pub async fn icrc_1_supported_standards(&self) -> CallResult<(Vec<StandardRecord>,)> {
        ic_cdk::call(self.0, "icrc1_supported_standards", ()).await
    }
    pub async fn icrc_1_symbol(&self) -> CallResult<(String,)> {
        ic_cdk::call(self.0, "icrc1_symbol", ()).await
    }
    pub async fn icrc_1_total_supply(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_total_supply", ()).await
    }
    pub async fn icrc_1_transfer(&self, arg0: TransferArg) -> CallResult<(Result_,)> {
        ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await
    }
    pub async fn icrc_21_canister_call_consent_message(
        &self,
        arg0: ConsentMessageRequest,
    ) -> CallResult<(Result1,)> {
        ic_cdk::call(self.0, "icrc21_canister_call_consent_message", (arg0,)).await
    }
    pub async fn icrc_2_allowance(&self, arg0: AllowanceArgs) -> CallResult<(Allowance,)> {
        ic_cdk::call(self.0, "icrc2_allowance", (arg0,)).await
    }
    pub async fn icrc_2_approve(&self, arg0: ApproveArgs) -> CallResult<(Result2,)> {
        ic_cdk::call(self.0, "icrc2_approve", (arg0,)).await
    }
    pub async fn icrc_2_transfer_from(&self, arg0: TransferFromArgs) -> CallResult<(Result3,)> {
        ic_cdk::call(self.0, "icrc2_transfer_from", (arg0,)).await
    }
    pub async fn icrc_3_get_archives(
        &self,
        arg0: GetArchivesArgs,
    ) -> CallResult<(Vec<Icrc3ArchiveInfo>,)> {
        ic_cdk::call(self.0, "icrc3_get_archives", (arg0,)).await
    }
    pub async fn icrc_3_get_blocks(
        &self,
        arg0: Vec<GetBlocksRequest>,
    ) -> CallResult<(GetBlocksResult,)> {
        ic_cdk::call(self.0, "icrc3_get_blocks", (arg0,)).await
    }
    pub async fn icrc_3_get_tip_certificate(&self) -> CallResult<(Option<Icrc3DataCertificate>,)> {
        ic_cdk::call(self.0, "icrc3_get_tip_certificate", ()).await
    }
    pub async fn icrc_3_supported_block_types(&self) -> CallResult<(Vec<SupportedBlockType>,)> {
        ic_cdk::call(self.0, "icrc3_supported_block_types", ()).await
    }
    pub async fn is_ledger_ready(&self) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "is_ledger_ready", ()).await
    }
}
