use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::types::{SelfCanister, SwapTokenPair, TokenPairAmm};

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapByLoanArg {
    pub self_canister: SelfCanister,
    pub pas: Vec<TokenPairAmm>,

    pub from: Account,
    pub loan: Nat,                // pay loaned token
    pub path: Vec<SwapTokenPair>, // pay exact tokens
    pub to: Account,
}
