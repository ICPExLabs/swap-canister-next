use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{CanisterId, TransferFee},
};

/// Transfer transactions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct TransferToken {
    /// Tokens
    pub token: CanisterId,
    /// Source Account
    pub from: Account,
    /// Transfer quantity, not including handling fee
    pub amount: Nat,
    /// Target account
    pub to: Account,
    /// Transfer fees and account collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<TransferFee>,
}

impl TryFrom<TransferToken> for proto::TransferToken {
    type Error = candid::Error;

    fn try_from(value: TransferToken) -> Result<Self, Self::Error> {
        let token = value.token.into();
        let from = value.from.into();
        let amount = value.amount.try_into()?;
        let to = value.to.into();
        let fee = value.fee.map(|fee| fee.try_into()).transpose()?;

        Ok(Self {
            token: Some(token),
            from: Some(from),
            amount: Some(amount),
            to: Some(to),
            fee,
        })
    }
}

impl TryFrom<proto::TransferToken> for TransferToken {
    type Error = String;

    fn try_from(value: proto::TransferToken) -> Result<Self, Self::Error> {
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
        let fee = value.fee.map(|fee| fee.try_into()).transpose()?;

        Ok(Self {
            token,
            from,
            amount,
            to,
            fee,
        })
    }
}
