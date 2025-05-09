use ic_canister_kit::common::option::{display_option, display_option_by};

use super::*;

// ========================= swap with deposit =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapWithDepositAndWithdrawArgs {
    pub from: Account, // make caller, caller must be consistent with from

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

impl Display for TokenPairSwapWithDepositAndWithdrawArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenPairSwapWithDepositAndWithdrawArgs {{ from: ({}), deposit_amount_without_fee: {}, deposit_fee: {}, amount_out_min: {}, path: [{}], to: {}, deadline: {}, withdraw_fee: {}, memo: {}, created: {} }}",
            display_account(&self.from),
            self.deposit_amount_without_fee,
            display_option(&self.deposit_fee),
            self.amount_out_min,
            self.path.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "),
            self.to,
            display_option_by(&self.deadline, |deadline| deadline.as_ref().to_string()),
            display_option(&self.withdraw_fee),
            display_option_by(&self.memo, |memo| hex::encode(memo)),
            display_option_by(&self.created, |created| created.into_inner().to_string()),
        )
    }
}
