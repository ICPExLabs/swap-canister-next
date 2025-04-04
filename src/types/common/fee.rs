use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct SwapFee {
    pub numerator: u32,
    pub denominator: u32,
}

impl SwapFee {
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
}

// ========================== view ==========================

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct SwapFeeView(String);

impl From<SwapFee> for SwapFeeView {
    fn from(value: SwapFee) -> Self {
        Self(format!("{}/{}", value.numerator, value.denominator))
    }
}
