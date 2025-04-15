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
    pub timestamp: TimestampNanos,
    /// 累计单位指数
    pub exponent: u8,
    /// price0 cumulative
    pub price0: Nat,
    /// price1 cumulative
    pub price1: Nat,
}

impl TryFrom<PairCumulativePrice> for proto::PairCumulativePrice {
    type Error = candid::Error;

    fn try_from(value: PairCumulativePrice) -> Result<Self, Self::Error> {
        let pa = value.pa.into();
        let timestamp = value.timestamp.into_inner();
        let exponent = value.exponent as u32;
        let price0 = value.price0.try_into()?;
        let price1 = value.price1.try_into()?;

        Ok(Self {
            pa: Some(pa),
            timestamp,
            exponent,
            price0: Some(price0),
            price1: Some(price1),
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
        let timestamp = TimestampNanos::from_inner(value.timestamp);
        let exponent = value.exponent as u8;
        let price0 = value
            .price0
            .ok_or_else(|| "price0 of pair cumulative price can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore price0 a of pair cumulative price failed".to_string())?;
        let price1 = value
            .price1
            .ok_or_else(|| "price1 of pair cumulative price can not be none".to_string())?
            .try_into()
            .map_err(|_| "restore price1 a of pair cumulative price failed".to_string())?;

        Ok(Self {
            pa,
            timestamp,
            exponent,
            price0,
            price1,
        })
    }
}
