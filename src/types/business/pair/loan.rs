use super::*;

// ========================= swap by loan =========================

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairSwapByLoanArgs {
    pub from: Account,            // only for marking caller
    pub loan: Nat,                // pay loaned token
    pub path: Vec<TokenPairPool>, // pay exact tokens
    pub to: Account,
    pub deadline: Option<Deadline>,
}
