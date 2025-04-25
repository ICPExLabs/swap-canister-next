use super::*;

use super::{
    BusinessError, SelfCanister, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess, TokenPairLiquidityRemoveArg,
    TokenPairLiquidityRemoveSuccess,
};

/// Automated Market Maker
mod amm_constant_product;
#[allow(unused)]
pub use amm_constant_product::*;

/// Proactive Market Maker
/// https://docs.dodoex.io/zh/product/pmm-algorithm/details-about-pmm
/// https://dodoex.github.io/cn/docs/
mod pmm_v1;
#[allow(unused)]
pub use pmm_v1::*;

pub fn add_liquidity(
    _self: &mut MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg>,
) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
    match _self {
        MarketMaker::SwapV2(value) => amm_constant_product::add_liquidity(value, guard),
    }
}

pub fn remove_liquidity(
    _self: &mut MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityRemoveArg>,
) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
    match _self {
        MarketMaker::SwapV2(value) => amm_constant_product::remove_liquidity(value, guard),
    }
}

#[inline]
#[allow(clippy::too_many_arguments)]
pub fn swap<T: TokenPairArg>(
    _self: &mut MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, T>,
    transaction: SwapTransaction,
    trace: String,
    self_canister: &SelfCanister,
    amount0_out: Nat,
    amount1_out: Nat,
    to: Account,
) -> Result<(), BusinessError> {
    match _self {
        MarketMaker::SwapV2(value) => amm_constant_product::swap(
            value,
            guard,
            transaction,
            trace,
            self_canister,
            amount0_out,
            amount1_out,
            to,
        ),
    }
}
