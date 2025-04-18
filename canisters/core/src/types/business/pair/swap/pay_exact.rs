use super::*;

// ========================= swap by pay exact tokens =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapExactTokensForTokensArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub amount_in: Nat,      // pay
    pub amount_out_min: Nat, // min got
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

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

impl SelfCanisterArg for TokenPairSwapExactTokensForTokensArg {
    fn get_self_canister(&self) -> SelfCanister {
        self.self_canister
    }
}

impl TokenPairSwapArg for TokenPairSwapExactTokensForTokensArg {
    fn get_pas(&self) -> &[TokenPairAmm] {
        &self.pas
    }

    fn get_path(&self) -> &[SwapTokenPair] {
        &self.path
    }
}
