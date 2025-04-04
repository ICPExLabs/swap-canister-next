use candid::CandidType;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::{Deserialize, Serialize};

use crate::types::PoolLp;

use super::{
    Amm, BusinessError, DummyCanisterId, PairAmm, SelfCanister, SwapRatio, TokenBalances,
    TokenInfo, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess,
};

/// Automated Market Maker 自动化做市商
mod amm_constant_product;
#[allow(unused)]
pub use amm_constant_product::*;

/// Proactive Market Maker 自动化做市商
/// https://docs.dodoex.io/zh/product/pmm-algorithm/details-about-pmm
/// https://dodoex.github.io/cn/docs/
mod pmm_v1;
#[allow(unused)]
pub use pmm_v1::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MarketMaker {
    SwapV2(SwapV2MarketMaker),
}

impl MarketMaker {
    pub fn new_by_pair(
        amm: &Amm,
        subaccount: Subaccount,
        dummy_canister_id: DummyCanisterId,
        token0: &TokenInfo,
        token1: &TokenInfo,
    ) -> Self {
        let lp = PoolLp::new_inner_lp(dummy_canister_id, token0, token1);
        match amm {
            Amm::SwapV2M500 => Self::SwapV2(new_swap_v2_market_maker(
                subaccount,
                lp,
                SwapRatio::new(5, 10_000), // swap fee 0.05%
            )),
            Amm::SwapV2T3 => Self::SwapV2(new_swap_v2_market_maker(
                subaccount,
                lp,
                SwapRatio::new(3, 1_000), // swap fee 0.3%
            )),
            Amm::SwapV2H1 => Self::SwapV2(new_swap_v2_market_maker(
                subaccount,
                lp,
                SwapRatio::new(1, 100), // swap fee 1%
            )),
        }
    }

    pub fn accounts(&self, self_canister: &SelfCanister) -> Vec<Account> {
        match self {
            MarketMaker::SwapV2(value) => value.accounts(self_canister),
        }
    }

    pub fn add_liquidity(
        &mut self,
        fee_to: Option<Account>,
        token_balances: &mut TokenBalances,
        self_canister: SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        match self {
            MarketMaker::SwapV2(value) => {
                value.add_liquidity(fee_to, token_balances, self_canister, pa, arg)
            }
        }
    }
}

fn new_swap_v2_market_maker(
    subaccount: Subaccount,
    lp: PoolLp,
    swap_fee: SwapRatio,
) -> SwapV2MarketMaker {
    SwapV2MarketMaker::new(
        subaccount,
        swap_fee,
        lp,
        Some(SwapRatio::new(1, 6)), // protocol fee 1/6
    )
}

// ========================== view ==========================

#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub enum MarketMakerView {
    SwapV2(SwapV2MarketMakerView),
}

impl From<MarketMaker> for MarketMakerView {
    fn from(value: MarketMaker) -> Self {
        match value {
            MarketMaker::SwapV2(value) => Self::SwapV2(value.into()),
        }
    }
}
