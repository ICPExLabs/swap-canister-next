use candid::CandidType;
use serde::{Deserialize, Serialize};

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SwapRatio {
    pub numerator: u32,
    pub denominator: u32,
}

impl SwapRatio {
    /// new
    pub fn new(numerator: u32, denominator: u32) -> Self {
        assert!(0 < denominator, "Denominator cannot be zero");
        assert!(numerator < denominator, "Denominator cannot be less than numerator");
        Self { numerator, denominator }
    }

    /// zero
    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }
}

// ========================== view ==========================

/// ratio view
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SwapRatioView(String);

impl From<SwapRatio> for SwapRatioView {
    fn from(value: SwapRatio) -> Self {
        Self(format!("{}/{}", value.numerator, value.denominator))
    }
}
