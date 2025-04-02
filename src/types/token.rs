use candid::CandidType;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use super::BusinessError;

// common

#[derive(Debug, Deserialize, CandidType)]
pub enum TokenTransferResut {
    Ok(candid::Nat),
    Err(BusinessError),
}
impl From<Result<candid::Nat, BusinessError>> for TokenTransferResut {
    fn from(r: Result<candid::Nat, BusinessError>) -> Self {
        match r {
            Ok(n) => TokenTransferResut::Ok(n),
            Err(e) => TokenTransferResut::Err(e),
        }
    }
}
impl From<TokenTransferResut> for Result<candid::Nat, BusinessError> {
    fn from(r: TokenTransferResut) -> Self {
        match r {
            TokenTransferResut::Ok(n) => Ok(n),
            TokenTransferResut::Err(e) => Err(e),
        }
    }
}

// deposit

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenDepositArgs {
    pub canister_id: CanisterId,
    pub from: Account,
    pub amount: candid::Nat,
}

// withdraw

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenWithdrawArgs {
    pub canister_id: CanisterId,
    pub from: Account,
    pub amount: candid::Nat,
    pub to: Account,
}
