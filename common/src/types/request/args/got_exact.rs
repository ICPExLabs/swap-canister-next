use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::types::{SelfCanister, SwapTokenPair, TokenPairAmm};

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensForExactTokensArg {
    pub self_canister: SelfCanister,
    pub pas: Vec<TokenPairAmm>,

    pub from: Account,
    pub amount_out: Nat,    // got
    pub amount_in_max: Nat, // max pay
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
}
