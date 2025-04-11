use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::proto;

/// 转账交易
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct TransferToken {
    /// 源账户
    pub from: Account,
    /// 代币
    pub token: CanisterId,
    /// 转移数量，不包含手续费
    pub amount: Nat,
    /// 转移手续费及收取账户
    pub fee: Option<TransferFee>,
    /// 目标账户
    pub to: Account,
}

/// 转账手续费
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct TransferFee {
    /// 手续费数量
    pub fee: Nat,
    /// 手续费收取账户
    pub fee_to: Account,
}

impl TryFrom<TransferToken> for proto::TransferToken {
    type Error = candid::Error;

    fn try_from(value: TransferToken) -> Result<Self, Self::Error> {
        let from = value.from.into();
        let token = value.token.into();
        let amount = value.amount.try_into()?;
        let fee = value.fee.map(|fee| fee.try_into()).transpose()?;
        let to = value.to.into();
        Ok(Self {
            from: Some(from),
            token: Some(token),
            amount: Some(amount),
            fee,
            to: Some(to),
        })
    }
}

impl TryFrom<proto::TransferToken> for TransferToken {
    type Error = String;

    fn try_from(value: proto::TransferToken) -> Result<Self, Self::Error> {
        let from = value
            .from
            .ok_or_else(|| "from of withdraw can not be none".to_string())?
            .try_into()?;
        let token = value
            .token
            .ok_or_else(|| "token of withdraw can not be none".to_string())?
            .into();
        let amount = value
            .amount
            .ok_or_else(|| "from of withdraw can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount of withdraw failed".to_string())?;
        let fee = value.fee.map(|fee| fee.try_into()).transpose()?;
        let to = value
            .to
            .ok_or_else(|| "to of withdraw can not be none".to_string())?
            .try_into()?;

        Ok(Self {
            from,
            token,
            amount,
            fee,
            to,
        })
    }
}

impl TryFrom<TransferFee> for proto::TransferFee {
    type Error = candid::Error;

    fn try_from(value: TransferFee) -> Result<Self, Self::Error> {
        let fee = value.fee.try_into()?;
        let fee_to = value.fee_to.into();
        Ok(Self {
            fee: Some(fee),
            fee_to: Some(fee_to),
        })
    }
}

impl TryFrom<proto::TransferFee> for TransferFee {
    type Error = String;

    fn try_from(value: proto::TransferFee) -> Result<Self, Self::Error> {
        let fee = value
            .fee
            .ok_or_else(|| "from of transfer_fee can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore fee of transfer_fee failed".to_string())?;
        let fee_to = value
            .fee_to
            .ok_or_else(|| "fee_to of transfer_fee can not be none".to_string())?
            .try_into()?;

        Ok(Self { fee, fee_to })
    }
}
