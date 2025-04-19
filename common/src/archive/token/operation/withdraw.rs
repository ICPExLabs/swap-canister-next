use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::proto;

/// Withdrawal transactions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct WithdrawToken {
    /// Tokens
    pub token: CanisterId,
    /// from
    pub from: Account,
    /// The transfer quantity, actual consumption quantity, and the handling fee deducted by the token can during the transfer process must be included
    pub amount: Nat,
    /// to account
    pub to: Account,
}

impl TryFrom<WithdrawToken> for proto::WithdrawToken {
    type Error = candid::Error;

    fn try_from(value: WithdrawToken) -> Result<Self, Self::Error> {
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

impl TryFrom<proto::WithdrawToken> for WithdrawToken {
    type Error = String;

    fn try_from(value: proto::WithdrawToken) -> Result<Self, Self::Error> {
        let token = value
            .token
            .ok_or_else(|| "token of withdraw can not be none".to_string())?
            .into();
        let from = value
            .from
            .ok_or_else(|| "from of withdraw can not be none".to_string())?
            .try_into()?;
        let amount = value
            .amount
            .ok_or_else(|| "amount of withdraw can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount of withdraw failed".to_string())?;
        let to = value
            .to
            .ok_or_else(|| "to of withdraw can not be none".to_string())?
            .try_into()?;

        Ok(Self {
            from,
            token,
            amount,
            to,
        })
    }
}
