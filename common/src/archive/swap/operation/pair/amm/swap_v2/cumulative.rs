use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::{
    proto,
    types::{TimestampNanos, TokenPairAmm},
};

/// 累计价格
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct PairCumulativePrice {
    /// 代币对和算法
    pub pa: TokenPairAmm,
    /// 时间戳
    pub block_timestamp: TimestampNanos,
    /// 累计单位指数
    pub price_cumulative_exponent: u8,
    /// price0 cumulative
    pub price0_cumulative: Nat,
    /// price1 cumulative
    pub price1_cumulative: Nat,
}

impl TryFrom<PairCumulativePrice> for proto::PairCumulativePrice {
    type Error = candid::Error;

    fn try_from(value: PairCumulativePrice) -> Result<Self, Self::Error> {
        let pa = value.pa.into();
        let block_timestamp = value.block_timestamp.into_inner();
        let price_cumulative_exponent = value.price_cumulative_exponent as u32;
        let price0_cumulative = value.price0_cumulative.try_into()?;
        let price1_cumulative = value.price1_cumulative.try_into()?;

        Ok(Self {
            pa: Some(pa),
            block_timestamp,
            price_cumulative_exponent,
            price0_cumulative: Some(price0_cumulative),
            price1_cumulative: Some(price1_cumulative),
        })
    }
}

impl TryFrom<proto::PairCumulativePrice> for PairCumulativePrice {
    type Error = String;

    fn try_from(value: proto::PairCumulativePrice) -> Result<Self, Self::Error> {
        let pa = value
            .pa
            .ok_or_else(|| "pa of pair cumulative price can not be none".to_string())?
            .try_into()?;
        let block_timestamp = TimestampNanos::from_inner(value.block_timestamp);
        let price_cumulative_exponent = value.price_cumulative_exponent as u8;
        let price0_cumulative = value
            .price0_cumulative
            .ok_or_else(|| {
                "price0_cumulative of pair cumulative price can not be none".to_string()
            })?
            .try_into()
            .map_err(|_| {
                "restore price0_cumulative a of pair cumulative price failed".to_string()
            })?;
        let price1_cumulative = value
            .price1_cumulative
            .ok_or_else(|| {
                "price1_cumulative of pair cumulative price can not be none".to_string()
            })?
            .try_into()
            .map_err(|_| {
                "restore price1_cumulative a of pair cumulative price failed".to_string()
            })?;

        Ok(Self {
            pa,
            block_timestamp,
            price_cumulative_exponent,
            price0_cumulative,
            price1_cumulative,
        })
    }
}
