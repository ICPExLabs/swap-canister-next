use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{proto, types::TokenPairAmm};

// ==================== swap v2 mint ====================

/// SwapV2 Mint
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct SwapV2MintToken {
    /// 池子
    pub pa: TokenPairAmm,
    /// 操作账户
    pub from: Account,

    // pay
    /// token0
    pub token0: CanisterId,
    /// token1
    pub token1: CanisterId,
    /// amount0
    pub amount0: Nat,
    /// amount1
    pub amount1: Nat,

    // got
    /// token
    pub token: CanisterId,
    /// amount
    pub amount: Nat,

    /// 操作账户
    pub to: Account,
}

impl TryFrom<SwapV2MintToken> for proto::SwapV2MintToken {
    type Error = candid::Error;

    fn try_from(value: SwapV2MintToken) -> Result<Self, Self::Error> {
        let pa = value.pa.into();
        let from = value.from.into();
        let token0 = value.token0.into();
        let token1 = value.token1.into();
        let amount0 = value.amount0.try_into()?;
        let amount1 = value.amount1.try_into()?;
        let token = value.token.into();
        let amount = value.amount.try_into()?;
        let to = value.to.into();

        Ok(Self {
            pa: Some(pa),
            from: Some(from),
            token0: Some(token0),
            token1: Some(token1),
            amount0: Some(amount0),
            amount1: Some(amount1),
            token: Some(token),
            amount: Some(amount),
            to: Some(to),
        })
    }
}

impl TryFrom<proto::SwapV2MintToken> for SwapV2MintToken {
    type Error = String;

    fn try_from(value: proto::SwapV2MintToken) -> Result<Self, Self::Error> {
        let pa = value
            .pa
            .ok_or_else(|| "pa of swap v2 mint token can not be none".to_string())?
            .try_into()?;
        let from = value
            .from
            .ok_or_else(|| "from of swap v2 mint token can not be none".to_string())?
            .try_into()?;
        let token0 = value
            .token0
            .ok_or_else(|| "token0 of swap v2 mint token can not be none".to_string())?
            .into();
        let token1 = value
            .token1
            .ok_or_else(|| "token1 of swap v2 mint token can not be none".to_string())?
            .into();
        let amount0 = value
            .amount0
            .ok_or_else(|| "amount0 of swap v2 mint token can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount0 a of swap v2 mint token failed".to_string())?;
        let amount1 = value
            .amount1
            .ok_or_else(|| "amount1 of swap v2 mint token can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount1 a of swap v2 mint token failed".to_string())?;
        let token = value
            .token
            .ok_or_else(|| "token of swap v2 mint token can not be none".to_string())?
            .into();
        let amount = value
            .amount
            .ok_or_else(|| "amount of swap v2 mint token can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore amount a of swap v2 mint token failed".to_string())?;
        let to = value
            .to
            .ok_or_else(|| "to of swap v2 mint token can not be none".to_string())?
            .try_into()?;

        Ok(Self {
            pa,
            from,
            token0,
            token1,
            amount0,
            amount1,
            token,
            amount,
            to,
        })
    }
}
