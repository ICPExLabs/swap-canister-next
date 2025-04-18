use super::*;

// ========================= swap by got exact tokens =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapTokensForExactTokensArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub amount_out: Nat,    // got
    pub amount_in_max: Nat, // max pay
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

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
