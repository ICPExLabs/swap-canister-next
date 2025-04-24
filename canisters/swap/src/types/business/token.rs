use ::common::types::TimestampNanos;

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

// many

#[derive(Debug, Deserialize, CandidType)]
pub struct ManyTokenChangedResult(Result<Vec<Result<candid::Nat, BusinessError>>, BusinessError>);

impl From<Result<Vec<Result<candid::Nat, BusinessError>>, BusinessError>> for ManyTokenChangedResult {
    fn from(value: Result<Vec<Result<candid::Nat, BusinessError>>, BusinessError>) -> Self {
        Self(value)
    }
}
impl From<ManyTokenChangedResult> for Result<Vec<Result<candid::Nat, BusinessError>>, BusinessError> {
    fn from(value: ManyTokenChangedResult) -> Self {
        value.0
    }
}

// deposit

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenDepositArgs {
    pub token: CanisterId,
    pub from: Account, // make caller, caller must be consistent with from
    pub deposit_amount_without_fee: candid::Nat,
    pub to: Account,
    pub fee: Option<candid::Nat>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

// withdraw
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenWithdrawArgs {
    pub token: CanisterId,
    pub from: Account, // make caller, caller must be consistent with from
    pub withdraw_amount_without_fee: candid::Nat,
    pub to: Account,
    pub fee: Option<candid::Nat>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

// inner transfer

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenTransferArgs {
    pub token: CanisterId,
    pub from: Account, // make caller, caller must be consistent with from
    pub transfer_amount_without_fee: candid::Nat,
    pub to: Account,
    pub fee: Option<candid::Nat>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

// ======================== many ========================

// withdraw
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenWithdrawManyArgs {
    pub args: Vec<TokenWithdrawArgs>,
}
