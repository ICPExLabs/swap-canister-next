use candid::{CandidType, Nat};
use ic_canister_kit::types::{CanisterId, UserId};
use ic_cdk::api::call::RejectionCode;
use icrc_ledger_types::{icrc1::transfer::TransferError, icrc2::transfer_from::TransferFromError};
use serde::Deserialize;

use super::{super::TokenAccount, AmmText, TokenPair};

#[derive(Debug, Deserialize, CandidType)]
pub enum BusinessError {
    /// 跨罐子调用错误
    CallCanisterError((RejectionCode, String)),
    /// 代币转账错误
    TransferFromError(TransferFromError),
    /// 代币转账错误
    TransferError(TransferError),
    /// 不支持的代币罐子
    NotSupportedToken(CanisterId),
    /// 必须自己作为所有者才能调用
    NotOwner(UserId),
    /// 提现余额不足
    InsufficientBalance(Nat),
    /// 用户操作涉及到的账户被锁定，无法执行操作
    Locked(Vec<TokenAccount>),
    /// 无效的 Amm 算法
    InvalidAmm(AmmText),
    /// 代币池子已经存在
    TokenPairAmmExist((TokenPair, AmmText)),
}
