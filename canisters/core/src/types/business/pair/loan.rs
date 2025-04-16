use super::super::*;

// ========================= swap by loan =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapByLoanArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub loan: Nat,                // pay loaned token
    pub path: Vec<TokenPairPool>, // pay exact tokens
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}
