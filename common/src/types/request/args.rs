use candid::CandidType;
use serde::{Deserialize, Serialize};

#[cfg(feature = "archive-token")]
use crate::archive::token::{DepositToken, TransferToken, WithdrawToken};
use crate::types::{ArgWithMeta, CanisterId, TokenInfo, TokenPairAmm};

mod frozen;
pub use frozen::*;

mod liquidity_add;
pub use liquidity_add::*;

mod liquidity_remove;
pub use liquidity_remove::*;

mod pay_exact;
pub use pay_exact::*;

mod got_exact;
pub use got_exact::*;

mod pay_exact_by_loan;
pub use pay_exact_by_loan::*;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum RequestArgs {
    // no arg
    #[serde(rename = "token_block_push")]
    TokenBlockPush,
    #[serde(rename = "swap_block_push")]
    SwapBlockPush,
    #[serde(rename = "blocks_maintaining")]
    CanistersMaintaining,
    // config
    #[serde(rename = "token_frozen")]
    TokenFrozen(Box<TokenFrozenArgWithMeta>),
    #[serde(rename = "token_custom_put")]
    TokenCustomPut(Box<TokenCustomPutArgWithMeta>),
    #[serde(rename = "token_custom_remove")]
    TokenCustomRemove(Box<TokenCustomRemoveArgWithMeta>),
    // token
    #[cfg(feature = "archive-token")]
    #[serde(rename = "token_deposit")]
    TokenDeposit(Box<TokenDepositArgWithMeta>),
    #[cfg(feature = "archive-token")]
    #[serde(rename = "token_withdraw")]
    TokenWithdraw(Box<TokenWithdrawArgWithMeta>),
    #[cfg(feature = "archive-token")]
    #[serde(rename = "token_transfer")]
    TokenTransfer(Box<TokenTransferArgWithMeta>),
    // pair create
    #[serde(rename = "pair_create")]
    PairCreate(Box<PairCreateArgWithMeta>),
    #[serde(rename = "pair_remove")]
    PairRemove(Box<PairRemoveArgWithMeta>),
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

// config
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenFrozenArgWithMeta(ArgWithMeta<TokenFrozenArg>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenCustomPutArgWithMeta(ArgWithMeta<TokenInfo>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenCustomRemoveArgWithMeta(ArgWithMeta<CanisterId>);
// token
#[cfg(feature = "archive-token")]
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenDepositArgWithMeta(ArgWithMeta<DepositToken>);
#[cfg(feature = "archive-token")]
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenWithdrawArgWithMeta(ArgWithMeta<WithdrawToken>);
#[cfg(feature = "archive-token")]
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenTransferArgWithMeta(ArgWithMeta<TransferToken>);
// pair create
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairCreateArgWithMeta(ArgWithMeta<TokenPairAmm>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairRemoveArgWithMeta(ArgWithMeta<TokenPairAmm>);
// pair liquidity
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairLiquidityAddArgWithMeta(ArgWithMeta<TokenPairLiquidityAddArg>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairLiquidityRemoveArgWithMeta(ArgWithMeta<TokenPairLiquidityRemoveArg>);
// pair swap
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairSwapExactTokensForTokensArgWithMeta(ArgWithMeta<TokenPairSwapExactTokensForTokensArg>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairSwapTokensForExactTokensArgWithMeta(ArgWithMeta<TokenPairSwapTokensForExactTokensArg>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairSwapByLoanArgWithMeta(ArgWithMeta<TokenPairSwapByLoanArg>);

// ============================= from =============================

// config
impl From<ArgWithMeta<TokenFrozenArg>> for RequestArgs {
    fn from(value: ArgWithMeta<TokenFrozenArg>) -> Self {
        Self::TokenFrozen(Box::new(TokenFrozenArgWithMeta(value)))
    }
}
impl From<ArgWithMeta<TokenInfo>> for RequestArgs {
    fn from(value: ArgWithMeta<TokenInfo>) -> Self {
        Self::TokenCustomPut(Box::new(TokenCustomPutArgWithMeta(value)))
    }
}
impl From<ArgWithMeta<CanisterId>> for RequestArgs {
    fn from(value: ArgWithMeta<CanisterId>) -> Self {
        Self::TokenCustomRemove(Box::new(TokenCustomRemoveArgWithMeta(value)))
    }
}

// token
#[cfg(feature = "archive-token")]
impl From<ArgWithMeta<DepositToken>> for RequestArgs {
    fn from(value: ArgWithMeta<DepositToken>) -> Self {
        Self::TokenDeposit(Box::new(TokenDepositArgWithMeta(value)))
    }
}
#[cfg(feature = "archive-token")]
impl From<ArgWithMeta<WithdrawToken>> for RequestArgs {
    fn from(value: ArgWithMeta<WithdrawToken>) -> Self {
        Self::TokenWithdraw(Box::new(TokenWithdrawArgWithMeta(value)))
    }
}
#[cfg(feature = "archive-token")]
impl From<ArgWithMeta<TransferToken>> for RequestArgs {
    fn from(value: ArgWithMeta<TransferToken>) -> Self {
        Self::TokenTransfer(Box::new(TokenTransferArgWithMeta(value)))
    }
}

// pair create
impl ArgWithMeta<TokenPairAmm> {
    pub fn into_pair_create_request_args(self) -> RequestArgs {
        RequestArgs::PairCreate(Box::new(PairCreateArgWithMeta(self)))
    }
    pub fn into_pair_remove_request_args(self) -> RequestArgs {
        RequestArgs::PairRemove(Box::new(PairRemoveArgWithMeta(self)))
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
