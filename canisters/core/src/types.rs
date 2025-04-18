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
pub use ::common::archive::swap::{
    PairCreate, PairCumulativePrice, PairOperation, PairSwapToken, QuerySwapBlockResult, SwapBlock,
    SwapOperation, SwapTransaction, SwapV2BurnToken, SwapV2MintFeeToken, SwapV2MintToken,
    SwapV2Operation, SwapV2TransferToken,
};
#[allow(unused)]
pub use ::common::archive::token::{
    DepositToken, GetTokenBlocksResult, QueryTokenBlockResult, TokenBlock, TokenBlockRange,
    TokenOperation, TokenTransaction, TransferToken, WithdrawToken,
};
#[allow(unused)]
pub use ::common::proto;
#[allow(unused)]
pub use ::common::types::{
    Amm, AmmText, BlockIndex, BurnFee, BusinessError, Caller, CandidBlock, DoHash, DummyCanisterId,
    EncodedBlock, GetBlocksArgs, GetBlocksError, GetEncodedBlocksResult, HashOf, QueryBlockResult,
    SelfCanister, SwapTokenPair, TimestampNanos, TokenAccount, TokenPair, TokenPairAmm,
    TokenPairPool, TransferFee, check_caller, display_account,
};
#[allow(unused)]
pub use ::common::utils::pb::{from_proto_bytes, to_proto_bytes};

#[allow(unused)]
pub use ic_canister_kit::common::trap;

pub trait CheckArgs {
    type Result;
    fn check_args(&self) -> Result<Self::Result, BusinessError>;
}
