use super::super::*;

// create token pair pool
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairCreateArgs {
    pub pair_amm: TokenPairSwap,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}
