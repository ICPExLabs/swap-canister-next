use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{CanisterId, TransferFee},
};

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

impl TokenOperation {
    /// token
    pub fn get_token(&self) -> CanisterId {
        match self {
            TokenOperation::Deposit(value) => value.token,
            TokenOperation::Withdraw(value) => value.token,
            TokenOperation::Transfer(value) => value.token,
        }
    }

    /// from
    pub fn get_from(&self) -> Account {
        match self {
            TokenOperation::Deposit(value) => value.from,
            TokenOperation::Withdraw(value) => value.from,
            TokenOperation::Transfer(value) => value.from,
        }
    }

    /// amount
    pub fn get_amount(&self) -> &Nat {
        match self {
            TokenOperation::Deposit(value) => &value.amount,
            TokenOperation::Withdraw(value) => &value.amount,
            TokenOperation::Transfer(value) => &value.amount,
        }
    }

    /// to
    pub fn get_to(&self) -> Account {
        match self {
            TokenOperation::Deposit(value) => value.to,
            TokenOperation::Withdraw(value) => value.to,
            TokenOperation::Transfer(value) => value.to,
        }
    }

    /// fee
    pub fn get_transfer_fee(&self) -> Option<&TransferFee> {
        match self {
            TokenOperation::Deposit(_) => None,
            TokenOperation::Withdraw(_) => None,
            TokenOperation::Transfer(value) => value.fee.as_ref(),
        }
    }
}
