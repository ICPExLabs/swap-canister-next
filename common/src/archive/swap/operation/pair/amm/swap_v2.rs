use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::proto;

mod cumulative;
pub use cumulative::*;

mod mint;
pub use mint::*;

mod burn;
pub use burn::*;

mod mint_fee;
pub use mint_fee::*;

mod transfer;
pub use transfer::*;

/// swap v2
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub enum SwapV2Operation {
    /// Price accumulation, used to calculate the average price
    #[serde(rename = "cumulative_price")]
    CumulativePrice(PairCumulativePrice),
    /// Add liquidity
    #[serde(rename = "mint")]
    Mint(SwapV2MintToken),
    /// Remove liquidity
    #[serde(rename = "burn")]
    Burn(SwapV2BurnToken),
    /// Charge handling fees for casting liquidity
    #[serde(rename = "mint_fee")]
    MintFee(SwapV2MintFeeToken),
    /// Transfer liquidity
    #[serde(rename = "transfer")]
    Transfer(SwapV2TransferToken),
}

impl TryFrom<SwapV2Operation> for proto::SwapV2Operation {
    type Error = candid::Error;

    fn try_from(value: SwapV2Operation) -> Result<Self, Self::Error> {
        use proto::swap_v2_operation::SwapV2Operation::*;

        let swap_v2_operation = match value {
            SwapV2Operation::CumulativePrice(value) => CumulativePrice(value.try_into()?),
            SwapV2Operation::Mint(value) => Mint(value.try_into()?),
            SwapV2Operation::Burn(value) => Burn(value.try_into()?),
            SwapV2Operation::MintFee(value) => MintFee(value.try_into()?),
            SwapV2Operation::Transfer(value) => Transfer(value.try_into()?),
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
            CumulativePrice(value) => SwapV2Operation::CumulativePrice(value.try_into()?),
            Mint(value) => SwapV2Operation::Mint(value.try_into()?),
            Burn(value) => SwapV2Operation::Burn(value.try_into()?),
            MintFee(value) => SwapV2Operation::MintFee(value.try_into()?),
            Transfer(value) => SwapV2Operation::Transfer(value.try_into()?),
        };

        Ok(value)
    }
}
