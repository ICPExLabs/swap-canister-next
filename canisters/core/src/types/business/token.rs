use super::*;

// common

#[derive(Debug, Deserialize, CandidType)]
pub enum TokenChangedResult {
    Ok(candid::Nat), // height if deposit or withdraw, amount if inner transfer
    Err(BusinessError),
}
impl From<Result<candid::Nat, BusinessError>> for TokenChangedResult {
    fn from(r: Result<candid::Nat, BusinessError>) -> Self {
        match r {
            Ok(n) => TokenChangedResult::Ok(n),
            Err(e) => TokenChangedResult::Err(e),
        }
    }
}
impl From<TokenChangedResult> for Result<candid::Nat, BusinessError> {
    fn from(r: TokenChangedResult) -> Self {
        match r {
            TokenChangedResult::Ok(n) => Ok(n),
            TokenChangedResult::Err(e) => Err(e),
        }
    }
}

// deposit

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenDepositArgs {
    pub from: Account,
    pub token: CanisterId,
    pub amount_without_fee: candid::Nat,
}

// withdraw

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenWithdrawArgs {
    pub from: Account,
    pub token: CanisterId,
    pub amount_without_fee: candid::Nat,
    pub to: Account,
}

// inner transfer

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenTransferArgs {
    pub from: Account,
    pub token: CanisterId,
    pub amount_without_fee: candid::Nat,
    pub to: Account,
}
