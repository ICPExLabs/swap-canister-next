// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Nat, Principal};

use crate::types::{
    BusinessError, ManyTokenChangedResult, TokenChangedResult, TokenDepositArgs, TokenPairLiquidityAddArgs,
    TokenPairLiquidityAddResult, TokenPairLiquidityAddSuccess, TokenPairLiquidityRemoveArgs,
    TokenPairLiquidityRemoveResult, TokenPairLiquidityRemoveSuccess, TokenPairSwapByLoanArgs,
    TokenPairSwapExactTokensForTokensArgs, TokenPairSwapTokensForExactTokensArgs, TokenPairSwapTokensResult,
    TokenPairSwapTokensSuccess, TokenTransferArgs, TokenWithdrawArgs, TokenWithdrawManyArgs,
};

type CallResult<T> = Result<T, BusinessError>;

pub struct Service(pub Principal);
impl Service {
    // token
    pub async fn token_deposit(&self, args: TokenDepositArgs, retries: Option<u8>) -> CallResult<Nat> {
        Ok(ic_cdk::call::Call::unbounded_wait(self.0, "token_deposit")
            .with_args(&(args, retries))
            .await?
            .candid()?)
    }
    pub async fn token_withdraw(&self, args: TokenWithdrawArgs, retries: Option<u8>) -> CallResult<Nat> {
        Ok(ic_cdk::call::Call::unbounded_wait(self.0, "token_withdraw")
            .with_args(&(args, retries))
            .await?
            .candid()?)
    }
    pub async fn token_withdraw_many(
        &self,
        args: TokenWithdrawManyArgs,
        retries: Option<u8>,
    ) -> CallResult<Vec<CallResult<Nat>>> {
        Ok(ic_cdk::call::Call::unbounded_wait(self.0, "token_withdraw_many")
            .with_args(&(args, retries))
            .await?
            .candid()?)
    }
    pub async fn token_transfer(&self, args: TokenTransferArgs, retries: Option<u8>) -> CallResult<Nat> {
        Ok(ic_cdk::call::Call::unbounded_wait(self.0, "token_transfer")
            .with_args(&(args, retries))
            .await?
            .candid()?)
    }

    // pair liquidity
    pub async fn pair_liquidity_add(
        &self,
        args: TokenPairLiquidityAddArgs,
        retries: Option<u8>,
    ) -> CallResult<TokenPairLiquidityAddSuccess> {
        Ok(ic_cdk::call::Call::unbounded_wait(self.0, "pair_liquidity_add")
            .with_args(&(args, retries))
            .await?
            .candid()?)
    }
    pub async fn pair_liquidity_remove(
        &self,
        args: TokenPairLiquidityRemoveArgs,
        retries: Option<u8>,
    ) -> CallResult<TokenPairLiquidityRemoveSuccess> {
        Ok(ic_cdk::call::Call::unbounded_wait(self.0, "pair_liquidity_remove")
            .with_args(&(args, retries))
            .await?
            .candid()?)
    }

    // pair swap
    pub async fn pair_swap_exact_tokens_for_tokens(
        &self,
        args: TokenPairSwapExactTokensForTokensArgs,
        retries: Option<u8>,
    ) -> CallResult<TokenPairSwapTokensSuccess> {
        Ok(
            ic_cdk::call::Call::unbounded_wait(self.0, "pair_swap_exact_tokens_for_tokens")
                .with_args(&(args, retries))
                .await?
                .candid()?,
        )
    }
    pub async fn pair_swap_tokens_for_exact_tokens(
        &self,
        args: TokenPairSwapTokensForExactTokensArgs,
        retries: Option<u8>,
    ) -> CallResult<TokenPairSwapTokensSuccess> {
        Ok(
            ic_cdk::call::Call::unbounded_wait(self.0, "pair_swap_tokens_for_exact_tokens")
                .with_args(&(args, retries))
                .await?
                .candid()?,
        )
    }
    pub async fn pair_swap_by_loan(
        &self,
        args: TokenPairSwapByLoanArgs,
        retries: Option<u8>,
    ) -> CallResult<TokenPairSwapTokensSuccess> {
        Ok(ic_cdk::call::Call::unbounded_wait(self.0, "pair_swap_by_loan")
            .with_args(&(args, retries))
            .await?
            .candid()?)
    }
}
