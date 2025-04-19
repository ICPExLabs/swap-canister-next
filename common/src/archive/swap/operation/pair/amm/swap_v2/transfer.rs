use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{TokenPairAmm, TransferFee},
};

/// transfer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct SwapV2TransferToken {
    /// token pair pool
    pub pa: TokenPairAmm,
    /// from
    pub from: Account,
    /// Transfer quantity, not including handling fee
    pub amount: Nat,
    /// Target account
    pub to: Account,
    /// Transfer fees and account collection
    pub fee: Option<TransferFee>,
}

impl TryFrom<SwapV2TransferToken> for proto::SwapV2TransferToken {
    type Error = candid::Error;

    fn try_from(value: SwapV2TransferToken) -> Result<Self, Self::Error> {
        let pa = value.pa.into();
        let from = value.from.into();
        let amount = value.amount.try_into()?;
        let to = value.to.into();
        let fee = value.fee.map(|fee| fee.try_into()).transpose()?;

        Ok(Self {
            pa: Some(pa),
            from: Some(from),
            amount: Some(amount),
            to: Some(to),
            fee,
        })
    }
}

impl TryFrom<proto::SwapV2TransferToken> for SwapV2TransferToken {
    type Error = String;

    fn try_from(value: proto::SwapV2TransferToken) -> Result<Self, Self::Error> {
        let pa = value
            .pa
            .ok_or_else(|| "pa of swap v2 burn token can not be none".to_string())?
            .try_into()?;
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
            pa,
            from,
            amount,
            to,
            fee,
        })
    }
}
