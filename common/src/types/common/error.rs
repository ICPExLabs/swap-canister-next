use candid::{CandidType, Nat};
use ic_canister_kit::types::{CanisterId, UserId};
use ic_cdk::api::call::RejectionCode;
use icrc_ledger_types::{icrc1::transfer::TransferError, icrc2::transfer_from::TransferFromError};
use serde::Deserialize;

use crate::types::{AmmText, TokenPair};

use super::TokenAccount;

/// 所有错误
#[derive(Debug, Clone, Deserialize, CandidType)]
pub enum BusinessError {
    // ================= 通用错误 =================
    /// memo 太长
    MemoTooLong,
    /// 交易时间窗口无效
    InvalidCreated(String),
    /// 跨罐子调用错误
    CallCanisterError((RejectionCode, String)),
    /// 必须自己作为所有者才能调用
    NotOwner(UserId),
    /// 请求过期
    Expired(u64),

    // ================= 代币转移错误 =================
    /// 代币转账错误
    TransferFromError(TransferFromError),
    /// 代币转账错误
    TransferError(TransferError),
    /// 提现余额不足
    InsufficientBalance((CanisterId, Nat)),

    // ================= 代币错误 =================
    /// 不支持的代币罐子
    NotSupportedToken(CanisterId),
    /// 不支持的代币对
    InvalidTokenPair((CanisterId, CanisterId)),

    // ================= 并发错误 =================
    /// 用户操作涉及到的账户被锁定，无法执行操作
    TokenAccountsLocked(Vec<TokenAccount>),
    /// 用户操作涉及到的 Token Blockchain 被锁定，无法执行操作
    TokenBlockChainLocked,
    /// 用户操作涉及到的 Swap Blockchain 被锁定，无法执行操作
    SwapBlockChainLocked,
    /// 用户操作涉及到的账户未被锁定，无法执行操作
    TokenAccountsUnlocked(Vec<TokenAccount>),

    // ================= swap =================
    /// 无效的 Amm 算法
    InvalidAmm(String),
    /// 代币池子已经存在
    TokenPairAmmExist((TokenPair, AmmText)),
    /// 代币池子不存在
    TokenPairAmmNotExist((TokenPair, AmmText)),
    /// 流动性错误
    Liquidity(String),
    /// swap 错误
    Swap(String),

    // ================= token block chain =================
    /// 用户操作涉及到的账户被锁定，无法执行操作
    TokenBlockChainError(String),
}
