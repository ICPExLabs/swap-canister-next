use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::BusinessError;

#[derive(Debug, Serialize, Deserialize, CandidType, Clone, PartialEq, Eq, Hash)]
pub struct AmmText(String);
impl AsRef<str> for AmmText {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, CandidType, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Amm {
    #[serde(rename = "swap_v2_0.05%")]
    SwapV2M500, // fee 0.05%
    #[serde(rename = "swap_v2_0.3%")]
    SwapV2T3, // fee 0.3%
    #[serde(rename = "swap_v2_1%")]
    SwapV2H1, // fee 1%
}

impl TryFrom<&AmmText> for Amm {
    type Error = BusinessError;

    fn try_from(value: &AmmText) -> Result<Self, Self::Error> {
        match value.0.as_str() {
            "swap_v2_0.05%" => Ok(Self::SwapV2M500),
            "swap_v2_0.3%" => Ok(Self::SwapV2T3),
            "swap_v2_1%" => Ok(Self::SwapV2H1),
            _ => Err(BusinessError::InvalidAmm(value.clone())),
        }
    }
}

impl From<&Amm> for AmmText {
    fn from(value: &Amm) -> Self {
        match value {
            Amm::SwapV2M500 => Self("swap_v2_0.05%".to_string()),
            Amm::SwapV2T3 => Self("swap_v2_0.3%".to_string()),
            Amm::SwapV2H1 => Self("swap_v2_1%".to_string()),
        }
    }
}
