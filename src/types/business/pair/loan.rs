use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::types::Deadline;

use super::TokenPairPool;

// ========================= swap by loan =========================

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairSwapByLoanArgs {
    pub from: Account,            // only for marking caller
    pub loan: Nat,                // pay loaned token
    pub path: Vec<TokenPairPool>, // pay exact tokens
    pub to: Account,
    pub deadline: Option<Deadline>,
}
