use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use super::super::BusinessError;

// common

#[derive(Debug, Deserialize, CandidType)]
pub enum TokenTransferResult {
    Ok(candid::Nat),
    Err(BusinessError),
}
impl From<Result<candid::Nat, BusinessError>> for TokenTransferResult {
    fn from(r: Result<candid::Nat, BusinessError>) -> Self {
        match r {
            Ok(n) => TokenTransferResult::Ok(n),
            Err(e) => TokenTransferResult::Err(e),
        }
    }
}
impl From<TokenTransferResult> for Result<candid::Nat, BusinessError> {
    fn from(r: TokenTransferResult) -> Self {
        match r {
            TokenTransferResult::Ok(n) => Ok(n),
            TokenTransferResult::Err(e) => Err(e),
        }
    }
}

// deposit

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenDepositArgs {
    pub canister_id: CanisterId,
    pub from: Account,
    pub amount_without_fee: candid::Nat,
}

// withdraw

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenWithdrawArgs {
    pub canister_id: CanisterId,
    pub from: Account,
    pub amount_without_fee: candid::Nat,
    pub to: Account,
}
