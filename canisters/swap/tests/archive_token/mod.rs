// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{
    self, CandidType, Decode, Deserialize, Encode, Principal, encode_args, encode_one, utils::ArgumentEncoder,
};
use pocket_ic::RejectResponse;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct InitArgV1 {
    pub maintainers: Option<Vec<Principal>>,
    pub block_offset: Option<(u64, serde_bytes::ByteBuf)>,
    pub host_canister_id: Option<Principal>,
    pub max_memory_size_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum InitArgs {
    V0 {},
    V1(InitArgV1),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct DepositToken {
    pub to: Account,
    pub token: Principal,
    pub from: Account,
    pub amount: candid::Nat,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TransferFee {
    pub fee: candid::Nat,
    pub fee_to: Account,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TransferToken {
    pub to: Account,
    pub fee: Option<TransferFee>,
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
pub struct GetBlocksArgs {
    pub start: u64,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct TokenBlockRange {
    pub blocks: Vec<TokenBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum GetBlocksError {
    BadFirstBlockIndex {
        requested_index: u64,
        first_valid_index: u64,
    },
    Other {
        error_message: String,
        error_code: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum GetTokenBlocksResult {
    Ok(TokenBlockRange),
    Err(GetBlocksError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum GetEncodedBlocksResult {
    Ok(Vec<serde_bytes::ByteBuf>),
    Err(GetBlocksError),
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct CustomHttpRequest {
    pub url: String,
    pub method: String,
    pub body: serde_bytes::ByteBuf,
    pub headers: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct StreamingCallbackToken {
    pub token: Vec<(String, String)>,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct StreamingCallbackHttpResponse {
    pub token: Option<StreamingCallbackToken>,
    pub body: serde_bytes::ByteBuf,
}

candid::define_function!(pub StreamingStrategyCallbackCallback : (
    StreamingCallbackToken,
  ) -> (StreamingCallbackHttpResponse) query);
#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum StreamingStrategy {
    Callback {
        token: StreamingCallbackToken,
        callback: StreamingStrategyCallbackCallback,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct CustomHttpResponse {
    pub body: serde_bytes::ByteBuf,
    pub headers: Vec<(String, String)>,
    pub upgrade: Option<bool>,
    pub streaming_strategy: Option<StreamingStrategy>,
    pub status_code: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, CandidType, Deserialize)]
pub struct CustomMetrics {
    pub stable_memory_pages: u64,
    pub stable_memory_bytes: u64,
    pub heap_memory_bytes: u64,
    pub last_upgrade_time_seconds: u64,
    pub max_memory_size_bytes: u64,
    pub blocks: u64,
    pub blocks_bytes: u64,
    pub block_height_offset: u64,
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

    pub fn append_blocks(&self, arg0: Vec<serde_bytes::ByteBuf>) -> Result<()> {
        self.update_call("append_blocks", encode_one(arg0).unwrap())
    }
    pub fn get_block(&self, arg0: u64) -> Result<Option<TokenBlock>> {
        self.query_call("get_block", encode_one(arg0).unwrap())
    }
    pub fn get_block_pb(&self, arg0: serde_bytes::ByteBuf) -> Result<serde_bytes::ByteBuf> {
        self.query_call("get_block_pb", encode_one(arg0).unwrap())
    }
    pub fn get_blocks(&self, arg0: GetBlocksArgs) -> Result<GetTokenBlocksResult> {
        self.query_call("get_blocks", encode_one(arg0).unwrap())
    }
    pub fn get_blocks_pb(&self, arg0: serde_bytes::ByteBuf) -> Result<serde_bytes::ByteBuf> {
        self.query_call("get_blocks_pb", encode_one(arg0).unwrap())
    }
    pub fn get_encoded_blocks(&self, arg0: GetBlocksArgs) -> Result<GetEncodedBlocksResult> {
        self.query_call("get_encoded_blocks", encode_one(arg0).unwrap())
    }
    pub fn http_request(&self, arg0: CustomHttpRequest) -> Result<CustomHttpResponse> {
        self.query_call("http_request", encode_one(arg0).unwrap())
    }
    pub fn iter_blocks_pb(&self, arg0: serde_bytes::ByteBuf) -> Result<serde_bytes::ByteBuf> {
        self.query_call("iter_blocks_pb", encode_one(arg0).unwrap())
    }
    pub fn query_latest_block_index(&self) -> Result<Option<u64>> {
        self.query_call("query_latest_block_index", Encode!(&()).unwrap())
    }
    pub fn query_metrics(&self) -> Result<CustomMetrics> {
        self.query_call("query_metrics", Encode!(&()).unwrap())
    }
    pub fn remaining_capacity(&self) -> Result<u64> {
        self.query_call("remaining_capacity", Encode!(&()).unwrap())
    }
    pub fn set_maintainers(&self, arg0: Option<Vec<Principal>>) -> Result<()> {
        self.update_call("set_maintainers", encode_one(arg0).unwrap())
    }
    pub fn set_max_memory_size_bytes(&self, arg0: u64) -> Result<()> {
        self.update_call("set_max_memory_size_bytes", encode_one(arg0).unwrap())
    }
}
