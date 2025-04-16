// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Nat, Principal};

use crate::types::{
    BusinessError, TokenChangedResult, TokenDepositArgs, TokenPairLiquidityAddArgs,
    TokenPairLiquidityAddResult, TokenPairLiquidityAddSuccess, TokenPairLiquidityRemoveArgs,
    TokenPairLiquidityRemoveResult, TokenPairLiquidityRemoveSuccess, TokenPairSwapByLoanArgs,
    TokenPairSwapExactTokensForTokensArgs, TokenPairSwapTokensForExactTokensArgs,
    TokenPairSwapTokensResult, TokenPairSwapTokensSuccess, TokenTransferArgs, TokenWithdrawArgs,
};

pub struct Service(pub Principal);
impl Service {
    // token
    pub async fn token_deposit(
        &self,
        args: TokenDepositArgs,
        retries: Option<u8>,
    ) -> Result<Nat, BusinessError> {
        ic_cdk::call::<_, (TokenChangedResult,)>(self.0, "token_deposit", (args, retries))
            .await?
            .0
            .into()
    }
    pub async fn token_withdraw(
        &self,
        args: TokenWithdrawArgs,
        retries: Option<u8>,
    ) -> Result<Nat, BusinessError> {
        ic_cdk::call::<_, (TokenChangedResult,)>(self.0, "token_withdraw", (args, retries))
            .await?
            .0
            .into()
    }
    pub async fn token_transfer(
        &self,
        args: TokenTransferArgs,
        retries: Option<u8>,
    ) -> Result<Nat, BusinessError> {
        ic_cdk::call::<_, (TokenChangedResult,)>(self.0, "token_transfer", (args, retries))
            .await?
            .0
            .into()
    }

    // pair liquidity
    pub async fn pair_liquidity_add(
        &self,
        args: TokenPairLiquidityAddArgs,
        retries: Option<u8>,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        ic_cdk::call::<_, (TokenPairLiquidityAddResult,)>(
            self.0,
            "pair_liquidity_add",
            (args, retries),
        )
        .await?
        .0
        .into()
    }
    pub async fn pair_liquidity_remove(
        &self,
        args: TokenPairLiquidityRemoveArgs,
        retries: Option<u8>,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        ic_cdk::call::<_, (TokenPairLiquidityRemoveResult,)>(
            self.0,
            "pair_liquidity_remove",
            (args, retries),
        )
        .await?
        .0
        .into()
    }

    // pair swap
    pub async fn pair_swap_exact_tokens_for_tokens(
        &self,
        args: TokenPairSwapExactTokensForTokensArgs,
        retries: Option<u8>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        ic_cdk::call::<_, (TokenPairSwapTokensResult,)>(
            self.0,
            "pair_swap_exact_tokens_for_tokens",
            (args, retries),
        )
        .await?
        .0
        .into()
    }
    pub async fn pair_swap_tokens_for_exact_tokens(
        &self,
        args: TokenPairSwapTokensForExactTokensArgs,
        retries: Option<u8>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        ic_cdk::call::<_, (TokenPairSwapTokensResult,)>(
            self.0,
            "pair_swap_tokens_for_exact_tokens",
            (args, retries),
        )
        .await?
        .0
        .into()
    }
    pub async fn pair_swap_by_loan(
        &self,
        args: TokenPairSwapByLoanArgs,
        retries: Option<u8>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        ic_cdk::call::<_, (TokenPairSwapTokensResult,)>(
            self.0,
            "pair_swap_by_loan",
            (args, retries),
        )
        .await?
        .0
        .into()
    }
}
