use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{BurnFee, TokenPairAmm},
};

// ==================== swap v2 burn ====================

/// SwapV2 Burn
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct SwapV2BurnToken {
    /// 池子
    pub pa: TokenPairAmm,
    /// 操作账户
    pub from: Account,

    // pay
    /// token
    pub token: CanisterId,
    /// amount
    pub amount: Nat,

    // got
    /// token0
    pub token0: CanisterId,
    /// token1
    pub token1: CanisterId,
    /// amount0
    pub amount0: Nat,
    /// amount1
    pub amount1: Nat,

    /// 操作账户
    pub to: Account,

    /// 销毁手续费
    pub fee: Option<BurnFee>,
}

impl TryFrom<SwapV2BurnToken> for proto::SwapV2BurnToken {
    type Error = candid::Error;

    fn try_from(value: SwapV2BurnToken) -> Result<Self, Self::Error> {
        let pa = value.pa.into();
        let from = value.from.into();
        let token = value.token.into();
        let amount = value.amount.try_into()?;
        let token0 = value.token0.into();
        let token1 = value.token1.into();
        let amount0 = value.amount0.try_into()?;
        let amount1 = value.amount1.try_into()?;
        let to = value.to.into();
        let fee = value.fee.map(|fee| fee.try_into()).transpose()?;

        Ok(Self {
            pa: Some(pa),
            from: Some(from),
            token: Some(token),
            amount: Some(amount),
            token0: Some(token0),
            token1: Some(token1),
            amount0: Some(amount0),
            amount1: Some(amount1),
            to: Some(to),
            fee,
        })
    }
}

impl TryFrom<proto::SwapV2BurnToken> for SwapV2BurnToken {
    type Error = String;

    fn try_from(value: proto::SwapV2BurnToken) -> Result<Self, Self::Error> {
        let pa = value
            .pa
            .ok_or_else(|| "pa of swap v2 burn token can not be none".to_string())?
            .try_into()?;
        let from = value
            .from
            .ok_or_else(|| "from of swap v2 burn token can not be none".to_string())?
            .try_into()?;
        let token = value
            .token
            .ok_or_else(|| "token of swap v2 burn token can not be none".to_string())?
            .into();
        let amount = value
            .amount
            .ok_or_else(|| "amount of swap v2 burn token can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount a of swap v2 burn token failed".to_string())?;
        let token0 = value
            .token0
            .ok_or_else(|| "token0 of swap v2 burn token can not be none".to_string())?
            .into();
        let token1 = value
            .token1
            .ok_or_else(|| "token1 of swap v2 burn token can not be none".to_string())?
            .into();
        let amount0 = value
            .amount0
            .ok_or_else(|| "amount0 of swap v2 burn token can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount0 a of swap v2 burn token failed".to_string())?;
        let amount1 = value
            .amount1
            .ok_or_else(|| "amount1 of swap v2 burn token can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount1 a of swap v2 burn token failed".to_string())?;
        let to = value
            .to
            .ok_or_else(|| "to of swap v2 burn token can not be none".to_string())?
            .try_into()?;
        let fee = value.fee.map(|fee| fee.try_into()).transpose()?;

        Ok(Self {
            pa,
            from,
            token,
            amount,
            token0,
            token1,
            amount0,
            amount1,
            to,
            fee,
        })
    }
}
