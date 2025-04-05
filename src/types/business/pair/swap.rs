use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::types::{BusinessError, Deadline};

use super::TokenPairPool;

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

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairSwapExactTokensForTokensSuccess {
    pub amounts: Vec<Nat>,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub enum TokenPairSwapExactTokensForTokensResult {
    Ok(TokenPairSwapExactTokensForTokensSuccess),
    Err(BusinessError),
}

impl From<Result<TokenPairSwapExactTokensForTokensSuccess, BusinessError>>
    for TokenPairSwapExactTokensForTokensResult
{
    fn from(r: Result<TokenPairSwapExactTokensForTokensSuccess, BusinessError>) -> Self {
        match r {
            Ok(n) => TokenPairSwapExactTokensForTokensResult::Ok(n),
            Err(e) => TokenPairSwapExactTokensForTokensResult::Err(e),
        }
    }
}

impl From<TokenPairSwapExactTokensForTokensResult>
    for Result<TokenPairSwapExactTokensForTokensSuccess, BusinessError>
{
    fn from(r: TokenPairSwapExactTokensForTokensResult) -> Self {
        match r {
            TokenPairSwapExactTokensForTokensResult::Ok(n) => Ok(n),
            TokenPairSwapExactTokensForTokensResult::Err(e) => Err(e),
        }
    }
}
