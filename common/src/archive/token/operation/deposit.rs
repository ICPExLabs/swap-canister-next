use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::proto;

/// Deposit to transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct DepositToken {
    /// Tokens
    pub token: CanisterId,
    /// from
    pub from: Account,
    /// Transfer quantity, actual transfer quantity, the handling fee deducted from token canister during transfer process will not be included
    pub amount: Nat,
    /// to account
    pub to: Account,
}

impl TryFrom<DepositToken> for proto::DepositToken {
    type Error = candid::Error;

    fn try_from(value: DepositToken) -> Result<Self, Self::Error> {
        let token = value.token.into();
        let from = value.from.into();
        let amount = value.amount.try_into()?;
        let to = value.to.into();

        Ok(Self {
            token: Some(token),
            from: Some(from),
            amount: Some(amount),
            to: Some(to),
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
            .ok_or_else(|| "amount of deposit can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount of deposit failed".to_string())?;
        let to = value
            .to
            .ok_or_else(|| "to of deposit can not be none".to_string())?
            .try_into()?;

        Ok(Self {
            token,
            from,
            amount,
            to,
        })
    }
}
