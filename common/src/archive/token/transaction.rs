use candid::CandidType;
use deposit::DepositToken;
use serde::{Deserialize, Serialize};
use transfer::TransferToken;
use withdraw::WithdrawToken;

use crate::proto;

/// 存入代币
pub mod deposit;

/// 提取代币
pub mod withdraw;

/// 转移代币
pub mod transfer;

/// 代币交易
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum TokenTransaction {
    /// 存入
    Deposit(DepositToken),
    /// 提取
    Withdraw(WithdrawToken),
    /// 转移
    Transfer(TransferToken),
}

impl TryFrom<TokenTransaction> for proto::TokenTransaction {
    type Error = candid::Error;

    fn try_from(value: TokenTransaction) -> Result<Self, Self::Error> {
        use proto::token_transaction::TokenTransaction::*;
        let token_transaction = match value {
            TokenTransaction::Deposit(value) => Deposit(value.try_into()?),
            TokenTransaction::Withdraw(value) => Withdraw(value.try_into()?),
            TokenTransaction::Transfer(value) => Transfer(value.try_into()?),
        };
        Ok(Self {
            token_transaction: Some(token_transaction),
        })
    }
}

impl TryFrom<proto::TokenTransaction> for TokenTransaction {
    type Error = String;

    fn try_from(value: proto::TokenTransaction) -> Result<Self, Self::Error> {
        let value = value
            .token_transaction
            .ok_or_else(|| "token_transaction can not be none".to_string())?;

        use proto::token_transaction::TokenTransaction::*;
        Ok(match value {
            Deposit(value) => TokenTransaction::Deposit(value.try_into()?),
            Withdraw(value) => TokenTransaction::Withdraw(value.try_into()?),
            Transfer(value) => TokenTransaction::Transfer(value.try_into()?),
        })
    }
}
