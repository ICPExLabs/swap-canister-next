use serde::{Deserialize, Serialize};

use crate::types::{ArgWithMeta, TokenPairLiquidityAddArg};

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestArgs {
    // pair liquidity
    #[serde(rename = "pair_liquidity_add")]
    PairLiquidityAdd(ArgWithMeta<TokenPairLiquidityAddArg>),
}
