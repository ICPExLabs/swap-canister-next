#[cfg(feature = "cdk")]
use std::collections::HashMap;

use candid::{CandidType, Nat};
#[cfg(feature = "cdk")]
use ic_canister_kit::types::{Bound, Cow, Storable};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::{Deserialize, Serialize};

/// Automated Market Maker
mod amm_constant_product;
#[allow(unused)]
pub use amm_constant_product::*;

#[allow(unused)]
use crate::types::{Amm, CanisterId, TokenPairAmm};

use super::{BusinessError, DummyCanisterId, PoolLp, SelfCanister, SwapRatio, TokenInfo};

// Proactive Market Maker
// https://docs.dodoex.io/zh/product/pmm-algorithm/details-about-pmm
// https://dodoex.github.io/cn/docs/
// mod pmm_v1;
// pmm v1
// #[allow(unused)]
// pub use pmm_v1::*;

/// market maker
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub enum MarketMaker {
    /// swap v2
    #[serde(rename = "swap_v2")]
    SwapV2(SwapV2MarketMaker),
}

#[cfg(feature = "cdk")]
impl Storable for MarketMaker {
    fn to_bytes(&self) -> Cow<[u8]> {
        use ic_canister_kit::common::trap;
        Cow::Owned(trap(ic_canister_kit::functions::stable::to_bytes(self)))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        use ic_canister_kit::common::trap;
        trap(ic_canister_kit::functions::stable::from_bytes(&bytes))
    }

    const BOUND: Bound = Bound::Unbounded;
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
            Amm::SwapV2M100 => Self::SwapV2(new_swap_v2_market_maker(
                subaccount,
                SwapRatio::new(1, 10_000), // swap fee 0.01%
                token0.canister_id,
                token1.canister_id,
                lp,
            )),
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

    #[cfg(feature = "cdk")]
    pub fn dummy_tokens(&self, tokens: &HashMap<CanisterId, Cow<'_, TokenInfo>>, pa: &TokenPairAmm) -> Vec<TokenInfo> {
        match self {
            MarketMaker::SwapV2(value) => value.dummy_tokens(tokens, pa),
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

    pub fn check_liquidity_removable<F>(
        &self,
        token_balance_of: F,
        from: &Account,
        liquidity_without_fee: &Nat,
        fee_to: Option<Account>,
    ) -> Result<(), BusinessError>
    where
        F: Fn(CanisterId, Account) -> Result<Nat, BusinessError>,
    {
        match self {
            MarketMaker::SwapV2(value) => {
                value.check_liquidity_removable(token_balance_of, from, liquidity_without_fee, fee_to)
            }
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
            MarketMaker::SwapV2(value) => value.get_amount_out(self_canister, amount_in, token_in, token_out),
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
            MarketMaker::SwapV2(value) => value.get_amount_in(self_canister, amount_out, token_in, token_out),
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

/// market maker view
#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub enum MarketMakerView {
    /// swap v2
    #[serde(rename = "swap_v2")]
    SwapV2(SwapV2MarketMakerView),
}

impl From<MarketMaker> for MarketMakerView {
    fn from(value: MarketMaker) -> Self {
        match value {
            MarketMaker::SwapV2(value) => Self::SwapV2(value.into()),
        }
    }
}
