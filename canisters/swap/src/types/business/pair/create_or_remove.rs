use super::super::*;

// create or remove token pair pool
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairCreateOrRemoveArgs {
    pub pool: TokenPairPool,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct TokenPairCreateOrRemoveResult(Result<MarketMakerView, BusinessError>);

impl From<Result<MarketMakerView, BusinessError>> for TokenPairCreateOrRemoveResult {
    fn from(value: Result<MarketMakerView, BusinessError>) -> Self {
        Self(value)
    }
}
