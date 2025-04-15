use serde::{Deserialize, Serialize};

use super::super::{ArgWithMeta, DepositToken, TokenPairLiquidityAddArg};

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestArgs {
    // token
    #[serde(rename = "token_deposit")]
    TokenDeposit(Box<ArgWithMeta<DepositToken>>),
    // pair liquidity
    #[serde(rename = "pair_liquidity_add")]
    PairLiquidityAdd(Box<ArgWithMeta<TokenPairLiquidityAddArg>>),
}

impl From<ArgWithMeta<DepositToken>> for RequestArgs {
    fn from(value: ArgWithMeta<DepositToken>) -> Self {
        Self::TokenDeposit(Box::new(value))
    }
}
