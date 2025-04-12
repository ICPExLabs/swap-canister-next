use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    common::{DoHash, HashOf},
    proto,
    utils::{hash::hash_sha256, pb::to_proto_bytes},
};

/// 存入代币
mod deposit;
pub use deposit::*;

/// 提取代币
mod withdraw;
pub use withdraw::*;

/// 转移代币
mod transfer;
pub use transfer::*;

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

impl DoHash for TokenTransaction {
    fn do_hash(&self) -> Result<HashOf<TokenTransaction>, String> {
        let transaction: proto::TokenTransaction =
            self.clone().try_into().map_err(|err| format!("{err:?}"))?;
        let bytes = to_proto_bytes(&transaction)?;
        let hash = hash_sha256(&bytes);
        Ok(HashOf::new(hash))
    }
}
