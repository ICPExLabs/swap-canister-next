use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::super::{
    ArgWithMeta, DepositToken, TokenPairLiquidityAddArg, TransferToken, WithdrawToken,
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
    // pair liquidity
    #[serde(rename = "pair_liquidity_add")]
    PairLiquidityAdd(Box<PairLiquidityAddArgWithMeta>),
}

// ============================= wrap =============================

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenDepositArgWithMeta(ArgWithMeta<DepositToken>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenWithdrawArgWithMeta(ArgWithMeta<WithdrawToken>);
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TokenTransferArgWithMeta(ArgWithMeta<TransferToken>);

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PairLiquidityAddArgWithMeta(ArgWithMeta<TokenPairLiquidityAddArg>);

// ============================= from =============================
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
