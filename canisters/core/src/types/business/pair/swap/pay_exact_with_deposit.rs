use super::*;

// ========================= swap with deposit =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapWithDepositAndWithdrawArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub deposit_amount_without_fee: candid::Nat, // amount_in
    pub deposit_fee: Option<candid::Nat>,

    pub amount_out_min: Nat, // min got
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub withdraw_fee: Option<candid::Nat>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

impl From<&TokenPairSwapWithDepositAndWithdrawArgs> for TokenDepositArgs {
    fn from(value: &TokenPairSwapWithDepositAndWithdrawArgs) -> Self {
        TokenDepositArgs {
            token: value.path[0].token.0,
            from: value.from,
            deposit_amount_without_fee: value.deposit_amount_without_fee.clone(),
            to: value.from,
            fee: value.deposit_fee.clone(),
            memo: None,
            created: None,
        }
    }
}

impl From<&TokenPairSwapWithDepositAndWithdrawArgs> for TokenPairSwapExactTokensForTokensArgs {
    fn from(value: &TokenPairSwapWithDepositAndWithdrawArgs) -> Self {
        TokenPairSwapExactTokensForTokensArgs {
            from: value.from,
            amount_in: value.deposit_amount_without_fee.clone(),
            amount_out_min: value.amount_out_min.clone(),
            path: value.path.clone(),
            to: value.from,
            deadline: value.deadline,
            memo: value.memo.clone(),
            created: value.created,
        }
    }
}

impl TokenPairSwapWithDepositAndWithdrawArgs {
    pub fn to_withdraw_args(&self, token: CanisterId, amount_out: Nat) -> TokenWithdrawArgs {
        TokenWithdrawArgs {
            token,
            from: self.from,
            withdraw_amount_without_fee: amount_out,
            to: self.to,
            fee: self.withdraw_fee.clone(),
            memo: None,
            created: None,
        }
    }
}
