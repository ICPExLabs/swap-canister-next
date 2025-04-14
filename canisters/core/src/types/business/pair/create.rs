use super::super::*;

// create token pair pool

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairCreateArgs {
    pub pool: TokenPairPool,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}
