use candid::{CandidType, Nat};
use icrc_ledger_types::{icrc1::transfer::TransferError, icrc2::transfer_from::TransferFromError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::types::{CanisterId, TokenPairAmm, UserId};

use super::TokenAccount;

/// All errors
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, Error)]
pub enum BusinessError {
    // ================= general error =================
    /// System Error
    #[error("system error: {0}")]
    SystemError(String),
    /// memo too long
    #[error("too long memo.")]
    MemoTooLong,
    /// The transaction time window is invalid
    #[error("invalid created timestamp. (system time is {system} but created at {created})")]
    InvalidCreated {
        /// system time
        system: u64,
        /// created
        created: u64,
    },
    /// Cross-can call error
    #[error("call canister error. ({0})")]
    CallCanisterError(String),
    /// You must be the owner to call it
    #[error("caller must be owner. (owner: {0})")]
    NotOwner(UserId),
    /// Request expires
    #[error("call is expired. (system time is {system} but deadline is {deadline})")]
    Expired {
        /// system time
        system: u64,
        /// deadline
        deadline: u64,
    },

    // ================= Token transfer error =================
    /// Token transfer error
    #[error("transfer token from error. ({0})")]
    TransferFromError(TransferFromError),
    /// Token transfer error
    #[error("transfer token error. ({0})")]
    TransferError(TransferError),
    /// Token transfer fee error
    #[error("bad transfer fee. (expected_fee: {})", .expected_fee)]
    BadTransferFee {
        /// The correct fee
        expected_fee: Nat,
    },
    /// Inadequate withdrawal balance
    #[error("insufficient balance. (token: {token}. balance: {balance})")]
    InsufficientBalance {
        /// token
        token: CanisterId,
        /// balance
        balance: Nat,
    },
    /// Incorrect transfer fee
    #[error("invalid transfer fee. (token: {token}. fee: {fee})")]
    InvalidTransferFee {
        /// token
        token: CanisterId,
        /// fee
        fee: Nat,
    },

    // ================= Token error =================
    /// Unsupported token canisters
    #[error("token:[{0}] is not supported.")]
    NotSupportedToken(CanisterId),
    /// frozen token
    #[error("token:[{0}] is frozen.")]
    FrozenToken(CanisterId),
    /// Unsupported token pairs
    #[error("token pair is not supported. ([{0}],[{1}])")]
    InvalidTokenPair(CanisterId, CanisterId),

    // ================= Concurrency error =================
    /// Apply for a new Request Index error
    #[error("request trace is locked. ({0})")]
    RequestTraceLocked(String),
    /// The account involved in the user's operation is locked and the operation cannot be performed.
    #[error("token accounts are locked. ({})", display_token_accounts(.0))]
    TokenAccountsLocked(Vec<TokenAccount>),
    /// The Token Blockchain involved in the user's operation is locked and the operation cannot be performed.
    #[error("token block chain is locked.")]
    TokenBlockChainLocked,
    /// The Swap Blockchain involved in user operations is locked and cannot perform the operation
    #[error("swap block chain is locked.")]
    SwapBlockChainLocked,
    /// The account involved in the user's operation is not locked and the operation cannot be performed.
    #[error("token accounts are not locked, can not unlock. ({})", display_token_accounts(.0))]
    TokenAccountsUnlocked(Vec<TokenAccount>),

    // ================= swap =================
    /// Invalid Amm algorithm
    #[error("invalid amm:{0}.")]
    InvalidAmm(String),
    /// Token pool already exists and cannot be created again
    #[error("token pair amm is already exist. ({0})")]
    TokenPairAmmExist(TokenPairAmm),
    /// The token pool does not exist and cannot be operated
    #[error("token pair amm is not exist. ({0})")]
    TokenPairAmmNotExist(TokenPairAmm),
    /// The token pool is alive, can not remove
    #[error("token pair amm is still alive. ({0})")]
    TokenPairAmmStillAlive(TokenPairAmm),
    /// Liquidity errors
    #[error("liquidity error: {0}.")]
    Liquidity(String),
    /// swap error
    #[error("swap error: {0}.")]
    Swap(String),

    // ================= token block chain =================
    /// Token Block Chain Error
    #[error("token block chain error: {0}.")]
    TokenBlockChainError(String),

    // ================= swap block chain =================
    /// Swap Block Chain Error
    #[error("token block chain error: {0}.")]
    SwapBlockChainError(String),
}

fn display_token_accounts(accounts: &[TokenAccount]) -> String {
    format!(
        "[{}]",
        accounts.iter().map(|ta| ta.to_string()).collect::<Vec<_>>().join(",")
    )
}

#[cfg(feature = "cdk")]
impl From<ic_cdk::call::Error> for BusinessError {
    fn from(value: ic_cdk::call::Error) -> Self {
        Self::CallCanisterError(value.to_string())
    }
}
#[cfg(feature = "cdk")]
impl From<ic_cdk::call::CallFailed> for BusinessError {
    fn from(value: ic_cdk::call::CallFailed) -> Self {
        Self::CallCanisterError(value.to_string())
    }
}
#[cfg(feature = "cdk")]
impl From<ic_cdk::call::CandidDecodeFailed> for BusinessError {
    fn from(value: ic_cdk::call::CandidDecodeFailed) -> Self {
        Self::CallCanisterError(value.to_string())
    }
}

impl From<TransferFromError> for BusinessError {
    fn from(value: TransferFromError) -> Self {
        Self::TransferFromError(value)
    }
}
impl From<TransferError> for BusinessError {
    fn from(value: TransferError) -> Self {
        Self::TransferError(value)
    }
}

impl BusinessError {
    /// system error
    pub fn system_error(message: impl Into<String>) -> BusinessError {
        BusinessError::SystemError(message.into())
    }
    /// Insufficient balance
    pub fn insufficient_balance(token: CanisterId, balance: Nat) -> Self {
        Self::InsufficientBalance { token, balance }
    }
    /// Fee error
    pub fn invalid_transfer_fee(token: CanisterId, fee: Nat) -> Self {
        Self::InvalidTransferFee { token, fee }
    }
}
