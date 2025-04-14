#[allow(unused)]
pub use candid::{CandidType, Nat};

#[allow(unused)]
pub use ic_canister_kit::types::*;

#[allow(unused)]
pub use crate::stable::*;

// ==================== common types ====================

pub mod common;
#[allow(unused)]
pub use common::*;

// ==================== business ====================

// business
pub mod business;
#[allow(unused)]
pub use business::*;

// ==================== ::common ====================

#[allow(unused)]
pub use ::common::archive::swap::{QuerySwapBlockResult, SwapBlock, SwapV2MintToken};
#[allow(unused)]
pub use ::common::archive::token::{
    DepositToken, GetTokenBlocksResult, QueryTokenBlockResult, TokenBlock, TokenBlockRange,
    TokenOperation, TokenTransaction, TransferFee, TransferToken, WithdrawToken,
};
#[allow(unused)]
pub use ::common::proto;
#[allow(unused)]
pub use ::common::types::{
    Amm, AmmText, BlockIndex, BusinessError, Caller, DoHash, DummyCanisterId, EncodedBlock,
    GetBlocksArgs, GetBlocksError, GetEncodedBlocksResult, QueryBlockResult, SelfCanister,
    TimestampNanos, TokenAccount, TokenPair, TokenPairAmm, check_caller,
};
#[allow(unused)]
pub use ::common::utils::pb::{from_proto_bytes, to_proto_bytes};

#[allow(unused)]
pub use ic_canister_kit::common::trap;

pub trait CheckArgs {
    type Result;
    fn check_args(&self) -> Result<Self::Result, BusinessError>;
}
