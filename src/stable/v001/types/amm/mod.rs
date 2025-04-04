use candid::CandidType;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub enum MarketMaker {
    SwapV2(SwapV2MarketMaker),
}

// impl MarketMaker {
//     pub fn new(subaccount: Subaccount, amm: &Amm) -> Self {
//         match amm {
//             Amm::SwapV2M500 => todo!(),
//             Amm::SwapV2M3000 => todo!(),
//             Amm::SwapV2M10000 => todo!(),
//         }
//     }
// }

// fn new_swap_v2_market_maker(subaccount: Subaccount) -> SwapV2MarketMaker {
//     todo!()
// }
