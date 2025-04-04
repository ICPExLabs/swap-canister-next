use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct SwapFee {
    numerator: u32,
    denominator: u32,
}
