use super::*;

// ========================= swap by got exact tokens =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensForExactTokensArgs {
    pub from: Account, // make caller, caller must be consistent with from

    pub amount_out: Nat,    // got
    pub amount_in_max: Nat, // max pay
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

impl SelfCanisterArg for TokenPairSwapTokensForExactTokensArg {
    fn get_self_canister(&self) -> SelfCanister {
        self.self_canister
    }
}

impl TokenPairSwapArg for TokenPairSwapTokensForExactTokensArg {
    fn get_pas(&self) -> &[TokenPairAmm] {
        &self.pas
    }

    fn get_path(&self) -> &[SwapTokenPair] {
        &self.path
    }
}
