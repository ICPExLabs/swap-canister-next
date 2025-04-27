use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::{
    types::{BurnFee, BusinessError, CanisterId, CheckArgs, SelfCanister, TokenPairAmm},
    utils::math::ZERO,
};

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<BurnFee>,
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
