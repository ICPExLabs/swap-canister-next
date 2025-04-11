use crate::utils::math::ZERO;

use super::*;

use super::TokenPairPool;

// ========================= liquidity add =========================

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairLiquidityAddArgs {
    pub from: Account,
    pub pool: TokenPairPool,
    pub amount_desired: (Nat, Nat),
    pub amount_min: (Nat, Nat),
    pub to: Account,
    pub deadline: Option<Deadline>,
}

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
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

pub struct TokenPairLiquidityAddArg {
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

// ========================= liquidity remove =========================

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairLiquidityRemoveArgs {
    pub from: Account,
    pub pool: TokenPairPool,
    pub liquidity: Nat,
    pub amount_min: (Nat, Nat),
    pub to: Account,
    pub deadline: Option<Deadline>,
}

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
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

pub struct TokenPairLiquidityRemoveArg {
    pub from: Account,
    pub token_a: CanisterId,
    pub token_b: CanisterId,
    pub liquidity: Nat,
    pub amount_a_min: Nat,
    pub amount_b_min: Nat,
    pub to: Account,
}

// check amount
impl CheckArgs for TokenPairLiquidityRemoveArg {
    type Result = ();

    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        // check 0
        if self.liquidity == *ZERO {
            return Err(BusinessError::Liquidity("LIQUIDITY_TOO_SMALL".into()));
        }

        Ok(())
    }
}
