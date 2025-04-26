use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::types::{SelfCanister, SwapTokenPair, TokenPairAmm};

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapExactTokensForTokensArg {
    pub self_canister: SelfCanister,
    pub pas: Vec<TokenPairAmm>,

    pub from: Account,
    pub amount_in: Nat,      // pay
    pub amount_out_min: Nat, // min got
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
}
