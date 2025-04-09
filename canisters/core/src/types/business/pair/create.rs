use super::*;

// create token pair pool

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairCreateArgs(pub TokenPairPool);

impl From<TokenPairPool> for TokenPairCreateArgs {
    fn from(value: TokenPairPool) -> Self {
        Self(value)
    }
}
