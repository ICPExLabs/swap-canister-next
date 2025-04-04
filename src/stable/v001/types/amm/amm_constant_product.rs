/// 常数乘积做市商（Constant Product AMM）
/// - 核心公式：x * y = k（x、y 为两资产数量，k 为常数）
/// - 代表项目：Uniswap V2/V3、SushiSwap、PancakeSwap
/// - 特点
///   - 简单高效，适合大多数交易场景。
///   - 大额交易时滑点显著（价格变化剧烈）。
///   - Uniswap V3 引入「集中流动性」，允许 LP 设定价格区间，提升资本效率。
use candid::CandidType;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};

use crate::types::{PoolLp, SwapFee, SwapFeeView};

/// 当前算法手续费下，需要的数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapV2MarketMaker {
    subaccount: Subaccount, // ! fixed. 资金余额存放位置 self_canister_id.subaccount
    swap_fee: SwapFee,      // ! fixed. 交易费率

    lp: PoolLp,            // lp 代币信息, 一旦新建池子成功，除了 supply，其他数据不可更改
    lp_fee: SwapFee,       // lp 分享的手续费
    protocol_fee: SwapFee, // 协议分享的手续费 和 lp fee 累计应该等于 1
}

impl SwapV2MarketMaker {
    pub fn new(
        subaccount: Subaccount,
        swap_fee: SwapFee,
        lp: PoolLp,
        lp_fee: SwapFee,
        protocol_fee: SwapFee,
    ) -> Self {
        #[allow(clippy::panic)] // ? SAFETY
        if lp_fee.numerator * protocol_fee.denominator + protocol_fee.numerator * lp_fee.denominator
            != lp_fee.denominator * protocol_fee.denominator
        {
            panic!("lp_fee + protocol_fee must be 1")
        }
        Self {
            subaccount,
            swap_fee,
            lp,
            lp_fee,
            protocol_fee,
        }
    }
}

// ========================== view ==========================

#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct SwapV2MarketMakerView {
    subaccount: String,
    swap_fee: SwapFeeView,

    lp: PoolLp,
    lp_fee: SwapFeeView,
    protocol_fee: SwapFeeView,
}

impl From<SwapV2MarketMaker> for SwapV2MarketMakerView {
    fn from(value: SwapV2MarketMaker) -> Self {
        Self {
            subaccount: hex::encode(value.subaccount),
            swap_fee: value.swap_fee.into(),
            lp: value.lp,
            lp_fee: value.lp_fee.into(),
            protocol_fee: value.protocol_fee.into(),
        }
    }
}
