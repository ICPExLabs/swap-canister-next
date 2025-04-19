use super::super::*;

mod pay_exact;
pub use pay_exact::*;

mod got_exact;
pub use got_exact::*;

mod pay_exact_by_loan;
pub use pay_exact_by_loan::*;

mod pay_exact_with_deposit;
pub use pay_exact_with_deposit::*;

// ================================== general ==================================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensSuccess {
    pub amounts: Vec<Nat>,
}
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensSuccessView {
    pub amounts: Vec<String>,
}
impl From<&TokenPairSwapTokensSuccess> for TokenPairSwapTokensSuccessView {
    fn from(value: &TokenPairSwapTokensSuccess) -> Self {
        Self {
            amounts: value.amounts.iter().map(|a| a.to_string()).collect(),
        }
    }
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

/// Check whether the coin pair is connected to the end
/// The result of the exchange of the previous coin pair is exactly the input of the next coin pair.
pub fn check_path(path: &[SwapTokenPair]) -> Result<(), BusinessError> {
    // check path
    if path.is_empty() {
        return Err(BusinessError::Swap("INVALID_PATH".into()));
    }
    if 1 < path.len() {
        // Check whether tokens are connected in a loop
        let mut i = 1;
        loop {
            if path.len() <= i {
                break;
            }

            let path0 = &path[i - 1];
            let path1 = &path[i];

            if path0.token.1 != path1.token.0 {
                return Err(BusinessError::Swap("INVALID_PATH".into()));
            }

            i += 1;
        }
    }
    Ok(())
}
