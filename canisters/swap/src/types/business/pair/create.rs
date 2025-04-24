use super::super::*;

// create token pair pool
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairCreateArgs {
    pub pool: TokenPairPool,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct TokenPairCreateResult(Result<MarketMakerView, BusinessError>);

impl From<Result<MarketMakerView, BusinessError>> for TokenPairCreateResult {
    fn from(value: Result<MarketMakerView, BusinessError>) -> Self {
        Self(value)
    }
}
