use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::proto;

/// Deposit tokens
mod deposit;
pub use deposit::*;

/// Retrieve tokens
mod withdraw;
pub use withdraw::*;

/// Transfer tokens
/// Liquidity addition and removal
mod transfer;
pub use transfer::*;

/// Token operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub enum TokenOperation {
    /// Save
    #[serde(rename = "deposit")]
    Deposit(DepositToken),
    /// extract
    #[serde(rename = "withdraw")]
    Withdraw(WithdrawToken),
    /// Transfer
    #[serde(rename = "transfer")]
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

        let value = match value {
            Deposit(value) => TokenOperation::Deposit(value.try_into()?),
            Withdraw(value) => TokenOperation::Withdraw(value.try_into()?),
            Transfer(value) => TokenOperation::Transfer(value.try_into()?),
        };

        Ok(value)
    }
}
