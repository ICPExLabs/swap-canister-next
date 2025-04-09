use super::*;

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairSwapTokensSuccess {
    pub amounts: Vec<Nat>,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub enum TokenPairSwapTokensResult {
    Ok(TokenPairSwapTokensSuccess),
    Err(BusinessError),
}

impl From<Result<TokenPairSwapTokensSuccess, BusinessError>> for TokenPairSwapTokensResult {
    fn from(r: Result<TokenPairSwapTokensSuccess, BusinessError>) -> Self {
        match r {
            Ok(n) => TokenPairSwapTokensResult::Ok(n),
            Err(e) => TokenPairSwapTokensResult::Err(e),
        }
    }
}

impl From<TokenPairSwapTokensResult> for Result<TokenPairSwapTokensSuccess, BusinessError> {
    fn from(r: TokenPairSwapTokensResult) -> Self {
        match r {
            TokenPairSwapTokensResult::Ok(n) => Ok(n),
            TokenPairSwapTokensResult::Err(e) => Err(e),
        }
    }
}

// ========================= swap by pay exact tokens =========================

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairSwapExactTokensForTokensArgs {
    pub from: Account,
    pub amount_in: Nat,      // pay
    pub amount_out_min: Nat, // min got
    pub path: Vec<TokenPairPool>,
    pub to: Account,
    pub deadline: Option<Deadline>,
}

// ========================= swap by got exact tokens =========================

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairSwapTokensForExactTokensArgs {
    pub from: Account,
    pub amount_out: Nat,    // got
    pub amount_in_max: Nat, // max pay
    pub path: Vec<TokenPairPool>,
    pub to: Account,
    pub deadline: Option<Deadline>,
}
