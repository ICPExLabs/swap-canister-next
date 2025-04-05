// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};

use crate::types::{
    BusinessError, TokenDepositArgs, TokenPairLiquidityAddResult, TokenPairLiquidityAddSuccess,
    TokenTransferResult, TokenWithdrawArgs,
};

pub struct Service(pub Principal);
impl Service {
    // token
    pub async fn token_deposit(
        &self,
        args: TokenDepositArgs,
        retries: Option<u8>,
    ) -> Result<candid::Nat, BusinessError> {
        ic_cdk::call::<_, (TokenTransferResult,)>(self.0, "token_deposit", (args, retries))
            .await
            .map_err(BusinessError::CallCanisterError)?
            .0
            .into()
    }
    pub async fn token_withdraw(
        &self,
        args: TokenWithdrawArgs,
        retries: Option<u8>,
    ) -> Result<candid::Nat, BusinessError> {
        ic_cdk::call::<_, (TokenTransferResult,)>(self.0, "token_withdraw", (args, retries))
            .await
            .map_err(BusinessError::CallCanisterError)?
            .0
            .into()
    }

    // pair liquidity
    pub async fn pair_liquidity_add(
        &self,
        args: crate::types::TokenPairLiquidityAddArgs,
        retries: Option<u8>,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        ic_cdk::call::<_, (TokenPairLiquidityAddResult,)>(
            self.0,
            "pair_liquidity_add",
            (args, retries),
        )
        .await
        .map_err(BusinessError::CallCanisterError)?
        .0
        .into()
    }
}
