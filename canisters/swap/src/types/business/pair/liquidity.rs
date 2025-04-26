use super::super::*;

// ========================= liquidity add =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairLiquidityAddArgs {
    pub from: Account, // make caller, caller must be consistent with from

    pub swap_pair: SwapTokenPair,
    pub amount_desired: (Nat, Nat),
    pub amount_min: (Nat, Nat),
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairLiquidityAddSuccess {
    pub amount: (Nat, Nat),
    pub liquidity: Nat,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct TokenPairLiquidityAddResult(Result<TokenPairLiquidityAddSuccess, BusinessError>);

impl From<Result<TokenPairLiquidityAddSuccess, BusinessError>> for TokenPairLiquidityAddResult {
    fn from(value: Result<TokenPairLiquidityAddSuccess, BusinessError>) -> Self {
        Self(value)
    }
}

impl From<TokenPairLiquidityAddResult> for Result<TokenPairLiquidityAddSuccess, BusinessError> {
    fn from(value: TokenPairLiquidityAddResult) -> Self {
        value.0
    }
}

impl SelfCanisterArg for TokenPairLiquidityAddArg {
    fn get_self_canister(&self) -> SelfCanister {
        self.self_canister
    }
}

impl TokenPairArg for TokenPairLiquidityAddArg {
    fn get_pa(&self) -> &TokenPairAmm {
        &self.pa
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairLiquidityAddSuccessView {
    pub amount: (String, String),
    pub liquidity: String,
}
impl From<&TokenPairLiquidityAddSuccess> for TokenPairLiquidityAddSuccessView {
    fn from(value: &TokenPairLiquidityAddSuccess) -> Self {
        Self {
            amount: (value.amount.0.to_string(), value.amount.1.to_string()),
            liquidity: value.liquidity.to_string(),
        }
    }
}

// ========================= liquidity remove =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairLiquidityRemoveArgs {
    pub from: Account, // make caller, caller must be consistent with from

    pub swap_pair: SwapTokenPair,
    pub liquidity_without_fee: Nat, // Removing liquidity will directly destroy a fee, restricting users from witch attacks
    pub amount_min: (Nat, Nat),
    pub to: Account,
    pub deadline: Option<Deadline>,

    pub memo: Option<Vec<u8>>,
    pub created: Option<TimestampNanos>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairLiquidityRemoveSuccess {
    pub amount: (Nat, Nat),
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct TokenPairLiquidityRemoveResult(Result<TokenPairLiquidityRemoveSuccess, BusinessError>);

impl From<Result<TokenPairLiquidityRemoveSuccess, BusinessError>> for TokenPairLiquidityRemoveResult {
    fn from(value: Result<TokenPairLiquidityRemoveSuccess, BusinessError>) -> Self {
        Self(value)
    }
}

impl From<TokenPairLiquidityRemoveResult> for Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
    fn from(value: TokenPairLiquidityRemoveResult) -> Self {
        value.0
    }
}

impl SelfCanisterArg for TokenPairLiquidityRemoveArg {
    fn get_self_canister(&self) -> SelfCanister {
        self.self_canister
    }
}

impl TokenPairArg for TokenPairLiquidityRemoveArg {
    fn get_pa(&self) -> &TokenPairAmm {
        &self.pa
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairLiquidityRemoveSuccessView {
    pub amount: (String, String),
}
impl From<&TokenPairLiquidityRemoveSuccess> for TokenPairLiquidityRemoveSuccessView {
    fn from(value: &TokenPairLiquidityRemoveSuccess) -> Self {
        Self {
            amount: (value.amount.0.to_string(), value.amount.1.to_string()),
        }
    }
}
