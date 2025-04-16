use candid::{CandidType, Nat};
use ic_canister_kit::types::{CanisterId, UserId};
use ic_cdk::api::call::RejectionCode;
use icrc_ledger_types::{icrc1::transfer::TransferError, icrc2::transfer_from::TransferFromError};
use serde::Deserialize;
use thiserror::Error;

use crate::types::TokenPairAmm;

use super::TokenAccount;

/// 所有错误
#[derive(Debug, Clone, Deserialize, CandidType, Error)]
pub enum BusinessError {
    // ================= 通用错误 =================
    /// memo 太长
    #[error("too long memo.")]
    MemoTooLong,
    /// 交易时间窗口无效
    #[error("invalid created timestamp. (system time is {system} but created at {created})")]
    InvalidCreated {
        /// system time
        system: u64,
        /// created
        created: u64,
    },
    /// 跨罐子调用错误
    #[error("call canister error. (code: {0:?}, message: {1})")]
    CallCanisterError(RejectionCode, String),
    /// 必须自己作为所有者才能调用
    #[error("caller must be owner. (owner: {0})")]
    NotOwner(UserId),
    /// 请求过期
    #[error("call is expired. (system time is {system} but deadline is {deadline})")]
    Expired {
        /// system time
        system: u64,
        /// deadline
        deadline: u64,
    },

    // ================= 代币转移错误 =================
    /// 代币转账错误
    #[error("transfer token from error. ({0})")]
    TransferFromError(TransferFromError),
    /// 代币转账错误
    #[error("transfer token error. ({0})")]
    TransferError(TransferError),
    /// 提现余额不足
    #[error("insufficient balance. (token: {token}. balance: {balance})")]
    InsufficientBalance {
        /// token
        token: CanisterId,
        /// balance
        balance: Nat,
    },
    /// 错误的转账费用
    #[error("invalid transfer fee. (token: {token}. fee: {fee})")]
    InvalidTransferFee {
        /// token
        token: CanisterId,
        /// fee
        fee: Nat,
    },

    // ================= 代币错误 =================
    /// 不支持的代币罐子
    #[error("token:[{0}] is not supported.")]
    NotSupportedToken(CanisterId),
    /// 不支持的代币对
    #[error("token pair is not supported. ([{0}],[{1}])")]
    InvalidTokenPair(CanisterId, CanisterId),

    // ================= 并发错误 =================
    /// 申请新的 Request Index 错误
    #[error("request trace is locked. ({0})")]
    RequestTraceLocked(String),
    /// 用户操作涉及到的账户被锁定，无法执行操作
    #[error("token accounts are locked. ({})", display_token_accounts(.0))]
    TokenAccountsLocked(Vec<TokenAccount>),
    /// 用户操作涉及到的 Token Blockchain 被锁定，无法执行操作
    #[error("token block chain is locked.")]
    TokenBlockChainLocked,
    /// 用户操作涉及到的 Swap Blockchain 被锁定，无法执行操作
    #[error("swap block chain is locked.")]
    SwapBlockChainLocked,
    /// 用户操作涉及到的账户未被锁定，无法执行操作
    #[error("token accounts are not locked, can not unlock. ({})", display_token_accounts(.0))]
    TokenAccountsUnlocked(Vec<TokenAccount>),

    // ================= swap =================
    /// 无效的 Amm 算法
    #[error("invalid amm:{0}.")]
    InvalidAmm(String),
    /// 代币池子已经存在, 不能再次创建
    #[error("token pair amm is already exist. ({0})")]
    TokenPairAmmExist(TokenPairAmm),
    /// 代币池子不存在, 不能进行操作
    #[error("token pair amm is not exist. ({0})")]
    TokenPairAmmNotExist(TokenPairAmm),
    /// 流动性错误
    #[error("liquidity error: {0}.")]
    Liquidity(String),
    /// swap 错误
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
        accounts
            .iter()
            .map(|ta| ta.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

impl From<(RejectionCode, String)> for BusinessError {
    fn from(value: (RejectionCode, String)) -> Self {
        Self::CallCanisterError(value.0, value.1)
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
    /// 余额不足
    pub fn insufficient_balance(token: CanisterId, balance: Nat) -> Self {
        Self::InsufficientBalance { token, balance }
    }
    /// 费用错误
    pub fn invalid_transfer_fee(token: CanisterId, fee: Nat) -> Self {
        Self::InvalidTransferFee { token, fee }
    }
}
