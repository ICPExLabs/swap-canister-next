use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{TimestampNanos, TokenPairAmm},
};

/// Accumulated price
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct SwapV2State {
    /// Token pairs and algorithms
    pub pa: TokenPairAmm,
    /// Timestamp
    pub block_timestamp: TimestampNanos,
    /// total supply of lp token
    pub supply: Nat,
    /// balance of token0
    pub reserve0: Nat,
    /// balance of token1
    pub reserve1: Nat,
    /// Cumulative unit exponent
    pub price_cumulative_exponent: u8,
    /// price0 cumulative
    pub price0_cumulative: Nat,
    /// price1 cumulative
    pub price1_cumulative: Nat,
}

impl TryFrom<SwapV2State> for proto::SwapV2State {
    type Error = candid::Error;

    fn try_from(value: SwapV2State) -> Result<Self, Self::Error> {
        let pa = value.pa.into();
        let block_timestamp = value.block_timestamp.into_inner();
        let supply = value.supply.try_into()?;
        let reserve0 = value.reserve0.try_into()?;
        let reserve1 = value.reserve1.try_into()?;
        let price_cumulative_exponent = value.price_cumulative_exponent as u32;
        let price0_cumulative = value.price0_cumulative.try_into()?;
        let price1_cumulative = value.price1_cumulative.try_into()?;

        Ok(Self {
            pa: Some(pa),
            block_timestamp,
            supply: Some(supply),
            reserve0: Some(reserve0),
            reserve1: Some(reserve1),
            price_cumulative_exponent,
            price0_cumulative: Some(price0_cumulative),
            price1_cumulative: Some(price1_cumulative),
        })
    }
}

impl TryFrom<proto::SwapV2State> for SwapV2State {
    type Error = String;

    fn try_from(value: proto::SwapV2State) -> Result<Self, Self::Error> {
        let pa = value
            .pa
            .ok_or_else(|| "pa of pair cumulative price can not be none".to_string())?
            .try_into()?;
        let block_timestamp = TimestampNanos::from_inner(value.block_timestamp);
        let supply = value
            .supply
            .ok_or_else(|| "supply of pair cumulative price can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore supply a of pair cumulative price failed".to_string())?;
        let reserve0 = value
            .reserve0
            .ok_or_else(|| "reserve0 of pair cumulative price can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore reserve0 a of pair cumulative price failed".to_string())?;
        let reserve1 = value
            .reserve1
            .ok_or_else(|| "reserve1 of pair cumulative price can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore reserve1 a of pair cumulative price failed".to_string())?;
        let price_cumulative_exponent = value.price_cumulative_exponent as u8;
        let price0_cumulative = value
            .price0_cumulative
            .ok_or_else(|| "price0_cumulative of pair cumulative price can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore price0_cumulative a of pair cumulative price failed".to_string())?;
        let price1_cumulative = value
            .price1_cumulative
            .ok_or_else(|| "price1_cumulative of pair cumulative price can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore price1_cumulative a of pair cumulative price failed".to_string())?;

        Ok(Self {
            pa,
            block_timestamp,
            supply,
            reserve0,
            reserve1,
            price_cumulative_exponent,
            price0_cumulative,
            price1_cumulative,
        })
    }
}
