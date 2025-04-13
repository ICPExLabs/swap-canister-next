use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::proto;

/// 存入代币
mod deposit;
pub use deposit::*;

/// 提取代币
mod withdraw;
pub use withdraw::*;

/// 转移代币
/// 流动性添加和移除
mod transfer;
pub use transfer::*;

/// 代币操作
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum TokenOperation {
    /// 存入
    Deposit(DepositToken),
    /// 提取
    Withdraw(WithdrawToken),
    /// 转移
    Transfer(TransferToken),
}

impl TryFrom<TokenOperation> for proto::TokenOperation {
    type Error = candid::Error;

    fn try_from(value: TokenOperation) -> Result<Self, Self::Error> {
        use proto::token_operation::TokenOperation::*;

        let token_operation = match value {
            TokenOperation::Deposit(value) => Deposit(value.try_into()?),
            TokenOperation::Withdraw(value) => Withdraw(value.try_into()?),
            TokenOperation::Transfer(value) => Transfer(value.try_into()?),
        };

        Ok(Self {
            token_operation: Some(token_operation),
        })
    }
}

impl TryFrom<proto::TokenOperation> for TokenOperation {
    type Error = String;

    fn try_from(value: proto::TokenOperation) -> Result<Self, Self::Error> {
        use proto::token_operation::TokenOperation::*;

        let value = value
            .token_operation
            .ok_or_else(|| "token_operation can not be none".to_string())?;

        Ok(match value {
            Deposit(value) => TokenOperation::Deposit(value.try_into()?),
            Withdraw(value) => TokenOperation::Withdraw(value.try_into()?),
            Transfer(value) => TokenOperation::Transfer(value.try_into()?),
        })
    }
}
