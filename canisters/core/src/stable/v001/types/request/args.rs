use serde::{Deserialize, Serialize};

use crate::types::{ArgWithMeta, TokenPairLiquidityAddArg};

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestArgs {
    // pair liquidity
    PairLiquidityAdd(ArgWithMeta<TokenPairLiquidityAddArg>),
}
