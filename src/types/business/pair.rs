use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{
    types::{Amm, BusinessError, CheckArgs, Deadline, TokenPair},
    utils::math::ZERO,
};

use super::super::AmmText;

/// (token0, token1, amm)
#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairPool {
    pub pair: (CanisterId, CanisterId),
    pub amm: AmmText,
}

impl TokenPair {
    pub fn to_pool(self, amm: &Amm) -> TokenPairPool {
        TokenPairPool {
            pair: (self.token0, self.token1),
            amm: amm.into(),
        }
    }
}

// create token pair pool

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct TokenPairCreateArgs(pub TokenPairPool);

impl From<TokenPairPool> for TokenPairCreateArgs {
    fn from(value: TokenPairPool) -> Self {
        Self(value)
    }
}

// liquidity add
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
pub enum TokenPairLiquidityAddResult {
    Ok(TokenPairLiquidityAddSuccess),
    Err(BusinessError),
}

impl From<Result<TokenPairLiquidityAddSuccess, BusinessError>> for TokenPairLiquidityAddResult {
    fn from(r: Result<TokenPairLiquidityAddSuccess, BusinessError>) -> Self {
        match r {
            Ok(n) => TokenPairLiquidityAddResult::Ok(n),
            Err(e) => TokenPairLiquidityAddResult::Err(e),
        }
    }
}

impl From<TokenPairLiquidityAddResult> for Result<TokenPairLiquidityAddSuccess, BusinessError> {
    fn from(r: TokenPairLiquidityAddResult) -> Self {
        match r {
            TokenPairLiquidityAddResult::Ok(n) => Ok(n),
            TokenPairLiquidityAddResult::Err(e) => Err(e),
        }
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
