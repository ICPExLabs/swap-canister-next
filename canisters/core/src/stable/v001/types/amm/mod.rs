use std::collections::HashMap;

use super::*;

use crate::types::PoolLp;

use super::{
    Amm, AmmText, BusinessError, DummyCanisterId, SelfCanister, SwapRatio, TokenBalances,
    TokenInfo, TokenPair, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess,
    TokenPairLiquidityRemoveArg, TokenPairLiquidityRemoveSuccess,
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
                SwapRatio::new(5, 10_000), // swap fee 0.05%
                token0.canister_id,
                token1.canister_id,
                lp,
            )),
            Amm::SwapV2T3 => Self::SwapV2(new_swap_v2_market_maker(
                subaccount,
                SwapRatio::new(3, 1_000), // swap fee 0.3%
                token0.canister_id,
                token1.canister_id,
                lp,
            )),
            Amm::SwapV2H1 => Self::SwapV2(new_swap_v2_market_maker(
                subaccount,
                SwapRatio::new(1, 100), // swap fee 1%
                token0.canister_id,
                token1.canister_id,
                lp,
            )),
        }
    }

    pub fn dummy_tokens(
        &self,
        tokens: &HashMap<CanisterId, TokenInfo>,
        pair: &TokenPair,
        amm: AmmText,
    ) -> Vec<TokenInfo> {
        match self {
            MarketMaker::SwapV2(value) => value.dummy_tokens(tokens, pair, &amm),
        }
    }

    pub fn accounts(&self, self_canister: &SelfCanister) -> Vec<Account> {
        match self {
            MarketMaker::SwapV2(value) => value.accounts(self_canister),
        }
    }

    pub fn dummy_canisters(&self) -> Vec<CanisterId> {
        match self {
            MarketMaker::SwapV2(value) => value.dummy_canisters(),
        }
    }

    pub fn add_liquidity(
        &mut self,
        guard: &mut TokenPairGuard<'_>,
        fee_to: Option<Account>,
        arg: ArgWithMeta<TokenPairLiquidityAddArg>,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        match self {
            MarketMaker::SwapV2(value) => value.add_liquidity(guard, fee_to, arg),
        }
    }

    pub fn check_liquidity_removable(
        &self,
        token_balances: &TokenBalances,
        from: &Account,
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        match self {
            MarketMaker::SwapV2(value) => {
                value.check_liquidity_removable(token_balances, from, liquidity)
            }
        }
    }

    pub fn remove_liquidity(
        &mut self,
        fee_to: Option<Account>,
        guard: &mut TokenBalancesGuard,
        self_canister: &SelfCanister,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        match self {
            MarketMaker::SwapV2(value) => value.remove_liquidity(fee_to, guard, self_canister, arg),
        }
    }

    pub fn get_amount_out(
        &self,
        self_canister: &SelfCanister,
        amount_in: &Nat,
        token_in: CanisterId,
        token_out: CanisterId,
    ) -> Result<(Account, Nat), BusinessError> {
        match self {
            MarketMaker::SwapV2(value) => {
                value.get_amount_out(self_canister, amount_in, token_in, token_out)
            }
        }
    }

    pub fn get_amount_in(
        &self,
        self_canister: &SelfCanister,
        amount_out: &Nat,
        token_in: CanisterId,
        token_out: CanisterId,
    ) -> Result<(Account, Nat), BusinessError> {
        match self {
            MarketMaker::SwapV2(value) => {
                value.get_amount_in(self_canister, amount_out, token_in, token_out)
            }
        }
    }

    pub fn swap(
        &mut self,
        guard: &mut TokenBalancesGuard,
        self_canister: &SelfCanister,
        amount0_out: Nat,
        amount1_out: Nat,
        to: Account,
    ) -> Result<(), BusinessError> {
        match self {
            MarketMaker::SwapV2(value) => {
                value.swap(guard, self_canister, amount0_out, amount1_out, to)
            }
        }
    }
}

fn new_swap_v2_market_maker(
    subaccount: Subaccount,
    fee_rate: SwapRatio,
    token0: CanisterId,
    token1: CanisterId,
    lp: PoolLp,
) -> SwapV2MarketMaker {
    SwapV2MarketMaker::new(
        subaccount,
        fee_rate,
        token0,
        token1,
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
