use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::proto;

mod pair;
pub use pair::*;

/// swap operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub enum SwapOperation {
    /// pair operation
    #[serde(rename = "pair")]
    Pair(PairOperation),
}

impl TryFrom<SwapOperation> for proto::SwapOperation {
    type Error = candid::Error;

    fn try_from(value: SwapOperation) -> Result<Self, Self::Error> {
        use proto::swap_operation::SwapOperation::*;

        let swap_operation = match value {
            SwapOperation::Pair(value) => Pair(value.try_into()?),
        };

        Ok(Self {
            swap_operation: Some(swap_operation),
        })
    }
}

impl TryFrom<proto::SwapOperation> for SwapOperation {
    type Error = String;

    fn try_from(value: proto::SwapOperation) -> Result<Self, Self::Error> {
        use proto::swap_operation::SwapOperation::*;

        let value = value
            .swap_operation
            .ok_or_else(|| "swap_operation can not be none".to_string())?;

        let value = match value {
            Pair(value) => SwapOperation::Pair(value.try_into()?),
        };

        Ok(value)
    }
}
