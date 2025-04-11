use super::*;

// common

#[derive(Debug, Deserialize, CandidType)]
pub struct TokenChangedResult(Result<candid::Nat, BusinessError>);

impl From<Result<candid::Nat, BusinessError>> for TokenChangedResult {
    fn from(value: Result<candid::Nat, BusinessError>) -> Self {
        Self(value)
    }
}
impl From<TokenChangedResult> for Result<candid::Nat, BusinessError> {
    fn from(value: TokenChangedResult) -> Self {
        value.0
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
