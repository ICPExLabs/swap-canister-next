use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::proto;

mod common;
// pub use common::*;

mod create;
pub use create::*;

mod swap;
pub use swap::*;

mod amm;
pub use amm::*;

/// pair operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub enum PairOperation {
    /// create pair
    Create(PairCreate),
    /// swap
    Swap(PairSwapToken),
    /// swap v2
    SwapV2(SwapV2Operation),
}

impl TryFrom<PairOperation> for proto::PairOperation {
    type Error = candid::Error;

    fn try_from(value: PairOperation) -> Result<Self, Self::Error> {
        use proto::pair_operation::PairOperation::*;

        let pair_operation = match value {
            PairOperation::Create(value) => Create(value.into()),
            PairOperation::Swap(value) => Swap(value.try_into()?),
            PairOperation::SwapV2(value) => SwapV2(value.try_into()?),
        };

        Ok(Self {
            pair_operation: Some(pair_operation),
        })
    }
}

impl TryFrom<proto::PairOperation> for PairOperation {
    type Error = String;

    fn try_from(value: proto::PairOperation) -> Result<Self, Self::Error> {
        use proto::pair_operation::PairOperation::*;

        let value = value
            .pair_operation
            .ok_or_else(|| "pair_operation can not be none".to_string())?;

        let value = match value {
            Create(value) => PairOperation::Create(value.try_into()?),
            Swap(value) => PairOperation::Swap(value.try_into()?),
            SwapV2(value) => PairOperation::SwapV2(value.try_into()?),
        };

        Ok(value)
    }
}
