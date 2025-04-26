use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{
    types::{BusinessError, CanisterId, CheckArgs, SelfCanister, TokenPairAmm},
    utils::math::ZERO,
};

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
            return Err(BusinessError::Liquidity("INSUFFICIENT_A_AMOUNT_DESIRED".into()));
        }
        if self.amount_b_desired == *ZERO {
            return Err(BusinessError::Liquidity("INSUFFICIENT_B_AMOUNT_DESIRED".into()));
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
