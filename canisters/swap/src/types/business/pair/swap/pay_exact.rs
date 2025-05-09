use ic_canister_kit::common::option::display_option_by;

use super::*;

// ========================= swap by pay exact tokens =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairSwapExactTokensForTokensArgs {
    pub from: Account, // make caller, caller must be consistent with from

    pub amount_in: Nat,      // pay
    pub amount_out_min: Nat, // min got
    pub path: Vec<SwapTokenPair>,
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
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

impl Display for TokenPairSwapExactTokensForTokensArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenPairSwapExactTokensForTokensArgs {{ from: ({}), amount_in: {}, amount_out_min: {}, path: [{}], to: ({}), deadline: {}, memo: {}, created: {} }}",
            display_account(&self.from),
            self.amount_in,
            self.amount_out_min,
            self.path.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "),
            display_account(&self.to),
            display_option_by(&self.deadline, |deadline| deadline.as_ref().to_string()),
            display_option_by(&self.memo, |memo| hex::encode(memo)),
            display_option_by(&self.created, |created| created.into_inner().to_string()),
        )
    }
}
