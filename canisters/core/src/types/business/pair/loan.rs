use super::super::*;

// ========================= swap by loan =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapByLoanArgs {
    pub from: Account,            // only for marking caller
    pub loan: Nat,                // pay loaned token
    pub path: Vec<TokenPairPool>, // pay exact tokens
    pub to: Account,
    pub deadline: Option<Deadline>,
}
