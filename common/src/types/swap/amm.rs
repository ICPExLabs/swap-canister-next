use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::types::BusinessError;

/// amm algorithm
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, CandidType)]
pub struct AmmText(String);
impl AsRef<str> for AmmText {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Amm algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, CandidType)]
pub enum Amm {
    /// fee 0.01%
    #[serde(rename = "swap_v2_0.01%")]
    SwapV2M100,
    /// fee 0.05%
    #[serde(rename = "swap_v2_0.05%")]
    SwapV2M500,
    /// fee 0.3%
    #[serde(rename = "swap_v2_0.3%")]
    SwapV2T3,
    /// fee 1%
    #[serde(rename = "swap_v2_1%")]
    SwapV2H1,
}

impl TryFrom<&str> for Amm {
    type Error = BusinessError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "swap_v2_0.01%" => Ok(Self::SwapV2M100),
            "swap_v2_0.05%" => Ok(Self::SwapV2M500),
            "swap_v2_0.3%" => Ok(Self::SwapV2T3),
            "swap_v2_1%" => Ok(Self::SwapV2H1),
            _ => Err(BusinessError::InvalidAmm(value.to_string())),
        }
    }
}

impl From<Amm> for AmmText {
    fn from(value: Amm) -> Self {
        match value {
            Amm::SwapV2M100 => Self("swap_v2_0.01%".to_string()),
            Amm::SwapV2M500 => Self("swap_v2_0.05%".to_string()),
            Amm::SwapV2T3 => Self("swap_v2_0.3%".to_string()),
            Amm::SwapV2H1 => Self("swap_v2_1%".to_string()),
        }
    }
}

impl From<Amm> for String {
    fn from(value: Amm) -> Self {
        let amm: AmmText = value.into();
        amm.0
    }
}

impl Amm {
    /// to text
    pub fn into_text(self) -> AmmText {
        self.into()
    }
}
