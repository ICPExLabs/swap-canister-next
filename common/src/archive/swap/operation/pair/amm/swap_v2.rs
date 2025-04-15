use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::proto;

mod mint;
pub use mint::*;

mod burn;
pub use burn::*;

mod fee;
pub use fee::*;

mod cumulative;
pub use cumulative::*;

/// swap v2
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub enum SwapV2Operation {
    /// 添加流动性
    #[serde(rename = "mint")]
    Mint(SwapV2MintToken),
    /// 移除流动性
    #[serde(rename = "burn")]
    Burn(SwapV2BurnToken),
    /// 费用流动性
    #[serde(rename = "mint_fee")]
    MintFee(SwapV2MintFeeToken),
    /// 价格累计，用于计算平均价
    #[serde(rename = "cumulative_price")]
    CumulativePrice(PairCumulativePrice),
}

impl TryFrom<SwapV2Operation> for proto::SwapV2Operation {
    type Error = candid::Error;

    fn try_from(value: SwapV2Operation) -> Result<Self, Self::Error> {
        use proto::swap_v2_operation::SwapV2Operation::*;

        let swap_v2_operation = match value {
            SwapV2Operation::Mint(value) => Mint(value.try_into()?),
            SwapV2Operation::Burn(value) => Burn(value.try_into()?),
            SwapV2Operation::MintFee(value) => MintFee(value.try_into()?),
            SwapV2Operation::CumulativePrice(value) => CumulativePrice(value.try_into()?),
        };

        Ok(Self {
            swap_v2_operation: Some(swap_v2_operation),
        })
    }
}

impl TryFrom<proto::SwapV2Operation> for SwapV2Operation {
    type Error = String;

    fn try_from(value: proto::SwapV2Operation) -> Result<Self, Self::Error> {
        use proto::swap_v2_operation::SwapV2Operation::*;

        let value = value
            .swap_v2_operation
            .ok_or_else(|| "swap_v2_operation can not be none".to_string())?;

        let value = match value {
            Mint(value) => SwapV2Operation::Mint(value.try_into()?),
            Burn(value) => SwapV2Operation::Burn(value.try_into()?),
            MintFee(value) => SwapV2Operation::MintFee(value.try_into()?),
            CumulativePrice(value) => SwapV2Operation::CumulativePrice(value.try_into()?),
        };

        Ok(value)
    }
}
