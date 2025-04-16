use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensSuccess {
    pub amounts: Vec<Nat>,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct TokenPairSwapTokensResult(Result<TokenPairSwapTokensSuccess, BusinessError>);

impl From<Result<TokenPairSwapTokensSuccess, BusinessError>> for TokenPairSwapTokensResult {
    fn from(value: Result<TokenPairSwapTokensSuccess, BusinessError>) -> Self {
        Self(value)
    }
}

impl From<TokenPairSwapTokensResult> for Result<TokenPairSwapTokensSuccess, BusinessError> {
    fn from(value: TokenPairSwapTokensResult) -> Self {
        value.0
    }
}

// ========================= swap by pay exact tokens =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapExactTokensForTokensArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub amount_in: Nat,      // pay
    pub amount_out_min: Nat, // min got
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

// ========================= swap by got exact tokens =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensForExactTokensArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub amount_out: Nat,    // got
    pub amount_in_max: Nat, // max pay
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}
