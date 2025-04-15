use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{proto, types::TokenPairAmm};

// ==================== swap v2 mint fee ====================

/// SwapV2 Mint Fee
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct SwapV2MintFeeToken {
    /// 池子
    pub pa: TokenPairAmm,

    // got
    /// token
    pub token: CanisterId,
    /// amount
    pub amount: Nat,

    /// to 账户
    pub to: Account,
}

impl TryFrom<SwapV2MintFeeToken> for proto::SwapV2MintFeeToken {
    type Error = candid::Error;

    fn try_from(value: SwapV2MintFeeToken) -> Result<Self, Self::Error> {
        let pa = value.pa.into();
        let token = value.token.into();
        let amount = value.amount.try_into()?;
        let to = value.to.into();

        Ok(Self {
            pa: Some(pa),
            token: Some(token),
            amount: Some(amount),
            to: Some(to),
        })
    }
}

impl TryFrom<proto::SwapV2MintFeeToken> for SwapV2MintFeeToken {
    type Error = String;

    fn try_from(value: proto::SwapV2MintFeeToken) -> Result<Self, Self::Error> {
        let pa = value
            .pa
            .ok_or_else(|| "pa of swap v2 mint fee token can not be none".to_string())?
            .try_into()?;
        let token = value
            .token
            .ok_or_else(|| "token of swap v2 mint fee token can not be none".to_string())?
            .into();
        let amount = value
            .amount
            .ok_or_else(|| "amount of swap v2 mint fee token can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount a of swap v2 mint fee token failed".to_string())?;
        let to = value
            .to
            .ok_or_else(|| "from of swap v2 mint fee token can not be none".to_string())?
            .try_into()?;

        Ok(Self {
            pa,
            to,
            token,
            amount,
        })
    }
}
