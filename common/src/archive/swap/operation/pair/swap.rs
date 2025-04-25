use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{Amm, CanisterId},
};

/// Create a pool
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct PairSwapToken {
    /// Pay tokens
    pub token_a: CanisterId,
    /// Obtain tokens
    pub token_b: CanisterId,
    /// amm
    pub amm: Amm,
    /// account pay
    pub from: Account,
    /// account got
    pub to: Account,
    /// amount pay
    pub amount_a: Nat,
    /// amount got
    pub amount_b: Nat,
}

impl TryFrom<PairSwapToken> for proto::PairSwapToken {
    type Error = candid::Error;

    fn try_from(value: PairSwapToken) -> Result<Self, Self::Error> {
        let token_a = value.token_a.into();
        let token_b = value.token_b.into();
        let amm = value.amm.into();
        let from = value.from.into();
        let to = value.to.into();
        let amount_a = value.amount_a.try_into()?;
        let amount_b = value.amount_b.try_into()?;

        Ok(Self {
            token_a: Some(token_a),
            token_b: Some(token_b),
            amm,
            from: Some(from),
            to: Some(to),
            amount_a: Some(amount_a),
            amount_b: Some(amount_b),
        })
    }
}

impl TryFrom<proto::PairSwapToken> for PairSwapToken {
    type Error = String;

    fn try_from(value: proto::PairSwapToken) -> Result<Self, Self::Error> {
        let token_a = value
            .token_a
            .ok_or_else(|| "token_a of pair swap token can not be none".to_string())?
            .into();
        let token_b = value
            .token_b
            .ok_or_else(|| "token_b of pair swap token can not be none".to_string())?
            .into();
        let amm = value.amm.as_str().try_into().map_err(|err| format!("{err:?}"))?;
        let from = value
            .from
            .ok_or_else(|| "from of pair swap token can not be none".to_string())?
            .try_into()?;
        let to = value
            .to
            .ok_or_else(|| "to of pair swap token can not be none".to_string())?
            .try_into()?;
        let amount_a = value
            .amount_a
            .ok_or_else(|| "amount_a of pair swap token can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount_a a of pair swap token failed".to_string())?;
        let amount_b = value
            .amount_b
            .ok_or_else(|| "amount_b of pair swap token can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount_b b of pair swap token failed".to_string())?;

        Ok(Self {
            token_a,
            token_b,
            amm,
            from,
            to,
            amount_a,
            amount_b,
        })
    }
}
