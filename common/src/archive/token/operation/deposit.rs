use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::proto;

/// 存入交易
#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub struct DepositToken {
    /// 代币
    pub token: CanisterId,
    /// 源账户
    pub from: Account,
    /// 转入数量，实际转入数量，转入过程中代币罐子扣除的手续费不计入
    pub amount: Nat,
}

impl TryFrom<DepositToken> for proto::DepositToken {
    type Error = candid::Error;

    fn try_from(value: DepositToken) -> Result<Self, Self::Error> {
        let token = value.token.into();
        let from = value.from.into();
        let amount = value.amount.try_into()?;
        Ok(Self {
            token: Some(token),
            from: Some(from),
            amount: Some(amount),
        })
    }
}

impl TryFrom<proto::DepositToken> for DepositToken {
    type Error = String;

    fn try_from(value: proto::DepositToken) -> Result<Self, Self::Error> {
        let token = value
            .token
            .ok_or_else(|| "token of deposit can not be none".to_string())?
            .into();
        let from = value
            .from
            .ok_or_else(|| "from of deposit can not be none".to_string())?
            .try_into()?;
        let amount = value
            .amount
            .ok_or_else(|| "from of deposit can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount of deposit failed".to_string())?;

        Ok(Self {
            token,
            from,
            amount,
        })
    }
}
