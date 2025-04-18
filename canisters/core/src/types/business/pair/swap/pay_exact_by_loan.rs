use super::*;

// ========================= swap by loan =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapByLoanArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub loan: Nat,                // pay loaned token
    pub path: Vec<SwapTokenPair>, // pay exact tokens
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapByLoanArg {
    pub self_canister: SelfCanister,
    pub pas: Vec<TokenPairAmm>,

    pub from: Account,
    pub loan: Nat,                // pay loaned token
    pub path: Vec<SwapTokenPair>, // pay exact tokens
    pub to: Account,
}

impl SelfCanisterArg for TokenPairSwapByLoanArg {
    fn get_self_canister(&self) -> SelfCanister {
        self.self_canister
    }
}

impl TokenPairSwapArg for TokenPairSwapByLoanArg {
    fn get_pas(&self) -> &[TokenPairAmm] {
        &self.pas
    }

    fn get_path(&self) -> &[SwapTokenPair] {
        &self.path
    }
}
