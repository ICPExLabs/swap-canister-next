use crate::utils::math::ZERO;

use super::super::*;

// ========================= liquidity add =========================

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairLiquidityAddArgs {
    pub from: Account, // 标记来源，caller 务必和 from 一致

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

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairLiquidityAddArg {
    pub self_canister: SelfCanister,
    pub pa: TokenPairAmm,

    pub from: Account,
    pub token_a: CanisterId,
    pub token_b: CanisterId,
    pub amount_a_desired: Nat,
    pub amount_b_desired: Nat,
    pub amount_a_min: Nat,
    pub amount_b_min: Nat,
    pub to: Account,
}

// check amount
impl CheckArgs for TokenPairLiquidityAddArg {
    type Result = ();

    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check 0
        if self.amount_a_desired == *ZERO {
            return Err(BusinessError::Liquidity(
                "INSUFFICIENT_A_AMOUNT_DESIRED".into(),
            ));
        }
        if self.amount_b_desired == *ZERO {
            return Err(BusinessError::Liquidity(
                "INSUFFICIENT_B_AMOUNT_DESIRED".into(),
            ));
        }
        // ? useless checking
        // if self.amount_a_min == *ZERO {
        //     return Err(BusinessError::Liquidity("INSUFFICIENT_A_AMOUNT_MIN".into()));
        // }
        // if self.amount_b_min == *ZERO {
        //     return Err(BusinessError::Liquidity("INSUFFICIENT_B_AMOUNT_MIN".into()));
        // }

        // ? useless checking
        // // check min < desired
        // if self.amount_a_desired < self.amount_a_min {
        //     return Err(BusinessError::Liquidity(
        //         "A_AMOUNT_DESIRED_LESS_THAN_MIN".into(),
        //     ));
        // }
        // if self.amount_b_desired < self.amount_b_min {
        //     return Err(BusinessError::Liquidity(
        //         "B_AMOUNT_DESIRED_LESS_THAN_MIN".into(),
        //     ));
        // }

        Ok(())
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
    pub from: Account, // 标记来源，caller 务必和 from 一致

    pub swap_pair: SwapTokenPair,
    pub liquidity_without_fee: Nat, // 移除流动性会直接销毁一份 fee，限制用户进行女巫攻击
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

impl From<Result<TokenPairLiquidityRemoveSuccess, BusinessError>>
    for TokenPairLiquidityRemoveResult
{
    fn from(value: Result<TokenPairLiquidityRemoveSuccess, BusinessError>) -> Self {
        Self(value)
    }
}

impl From<TokenPairLiquidityRemoveResult>
    for Result<TokenPairLiquidityRemoveSuccess, BusinessError>
{
    fn from(value: TokenPairLiquidityRemoveResult) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct TokenPairLiquidityRemoveArg {
    pub self_canister: SelfCanister,
    pub pa: TokenPairAmm,

    pub from: Account,
    pub token_a: CanisterId,
    pub token_b: CanisterId,
    pub liquidity_without_fee: Nat,
    pub amount_a_min: Nat,
    pub amount_b_min: Nat,
    pub to: Account,
}

// check amount
impl CheckArgs for TokenPairLiquidityRemoveArg {
    type Result = ();

    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check 0
        if self.liquidity_without_fee == *ZERO {
            return Err(BusinessError::Liquidity("LIQUIDITY_TOO_SMALL".into()));
        }

        Ok(())
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
