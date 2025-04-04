use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct SwapRatio {
    pub numerator: u32,
    pub denominator: u32,
}

impl SwapRatio {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        #[allow(clippy::panic)] // ? SAFETY
        if denominator == 0 {
            panic!("Denominator cannot be zero");
        }
        #[allow(clippy::panic)] // ? SAFETY
        if denominator < numerator {
            panic!("Denominator cannot be less than numerator");
        }
        Self {
            numerator,
            denominator,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }
}

// ========================== view ==========================

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct SwapRatioView(String);

impl From<SwapRatio> for SwapRatioView {
    fn from(value: SwapRatio) -> Self {
        Self(format!("{}/{}", value.numerator, value.denominator))
    }
}
