use candid::CandidType;
use common::types::TokenPairAmm;
use serde::{Deserialize, Serialize};

use super::super::{
    ArgWithMeta, DepositToken, TokenPairLiquidityAddArg, TokenPairLiquidityRemoveArg,
    TokenPairSwapByLoanArg, TokenPairSwapExactTokensForTokensArg,
    TokenPairSwapTokensForExactTokensArg, TransferToken, WithdrawToken,
};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum RequestArgs {
    // token
    #[serde(rename = "token_deposit")]
    TokenDeposit(Box<TokenDepositArgWithMeta>),
    #[serde(rename = "token_withdraw")]
    TokenWithdraw(Box<TokenWithdrawArgWithMeta>),
    #[serde(rename = "token_transfer")]
    TokenTransfer(Box<TokenTransferArgWithMeta>),
    // pair create
    #[serde(rename = "pair_liquidity_create")]
    PairLiquidityCreate(Box<PairLiquidityCreateArgWithMeta>),
    // pair liquidity
    #[serde(rename = "pair_liquidity_add")]
    PairLiquidityAdd(Box<PairLiquidityAddArgWithMeta>),
    #[serde(rename = "pair_liquidity_remove")]
    PairLiquidityRemove(Box<PairLiquidityRemoveArgWithMeta>),
    // pair swap
    #[serde(rename = "pair_swap_exact_tokens_for_tokens")]
    PairSwapExactTokensForTokens(Box<PairSwapExactTokensForTokensArgWithMeta>),
    #[serde(rename = "pair_swap_tokens_for_exact_tokens")]
    PairSwapTokensForExactTokens(Box<PairSwapTokensForExactTokensArgWithMeta>),
    #[serde(rename = "pair_swap_by_loan")]
    PairSwapByLoan(Box<PairSwapByLoanArgWithMeta>),
}

// ============================= wrap =============================

// token
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenDepositArgWithMeta(ArgWithMeta<DepositToken>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenWithdrawArgWithMeta(ArgWithMeta<WithdrawToken>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenTransferArgWithMeta(ArgWithMeta<TransferToken>);
// pair create
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairLiquidityCreateArgWithMeta(ArgWithMeta<TokenPairAmm>);
// pair liquidity
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairLiquidityAddArgWithMeta(ArgWithMeta<TokenPairLiquidityAddArg>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairLiquidityRemoveArgWithMeta(ArgWithMeta<TokenPairLiquidityRemoveArg>);
// pair swap
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairSwapExactTokensForTokensArgWithMeta(
    ArgWithMeta<TokenPairSwapExactTokensForTokensArg>,
);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairSwapTokensForExactTokensArgWithMeta(
    ArgWithMeta<TokenPairSwapTokensForExactTokensArg>,
);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairSwapByLoanArgWithMeta(ArgWithMeta<TokenPairSwapByLoanArg>);

// ============================= from =============================

// token
impl From<ArgWithMeta<DepositToken>> for RequestArgs {
    fn from(value: ArgWithMeta<DepositToken>) -> Self {
        Self::TokenDeposit(Box::new(TokenDepositArgWithMeta(value)))
    }
}
impl From<ArgWithMeta<WithdrawToken>> for RequestArgs {
    fn from(value: ArgWithMeta<WithdrawToken>) -> Self {
        Self::TokenWithdraw(Box::new(TokenWithdrawArgWithMeta(value)))
    }
}
impl From<ArgWithMeta<TransferToken>> for RequestArgs {
    fn from(value: ArgWithMeta<TransferToken>) -> Self {
        Self::TokenTransfer(Box::new(TokenTransferArgWithMeta(value)))
    }
}

// pair create
impl From<ArgWithMeta<TokenPairAmm>> for RequestArgs {
    fn from(value: ArgWithMeta<TokenPairAmm>) -> Self {
        Self::PairLiquidityCreate(Box::new(PairLiquidityCreateArgWithMeta(value)))
    }
}

// pair liquidity
impl From<ArgWithMeta<TokenPairLiquidityAddArg>> for RequestArgs {
    fn from(value: ArgWithMeta<TokenPairLiquidityAddArg>) -> Self {
        Self::PairLiquidityAdd(Box::new(PairLiquidityAddArgWithMeta(value)))
    }
}
impl From<ArgWithMeta<TokenPairLiquidityRemoveArg>> for RequestArgs {
    fn from(value: ArgWithMeta<TokenPairLiquidityRemoveArg>) -> Self {
        Self::PairLiquidityRemove(Box::new(PairLiquidityRemoveArgWithMeta(value)))
    }
}

// pair swap
impl From<ArgWithMeta<TokenPairSwapExactTokensForTokensArg>> for RequestArgs {
    fn from(value: ArgWithMeta<TokenPairSwapExactTokensForTokensArg>) -> Self {
        Self::PairSwapExactTokensForTokens(Box::new(PairSwapExactTokensForTokensArgWithMeta(value)))
    }
}
impl From<ArgWithMeta<TokenPairSwapTokensForExactTokensArg>> for RequestArgs {
    fn from(value: ArgWithMeta<TokenPairSwapTokensForExactTokensArg>) -> Self {
        Self::PairSwapTokensForExactTokens(Box::new(PairSwapTokensForExactTokensArgWithMeta(value)))
    }
}
impl From<ArgWithMeta<TokenPairSwapByLoanArg>> for RequestArgs {
    fn from(value: ArgWithMeta<TokenPairSwapByLoanArg>) -> Self {
        Self::PairSwapByLoan(Box::new(PairSwapByLoanArgWithMeta(value)))
    }
}
