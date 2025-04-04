use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

use super::TokenInfo;

#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub enum PoolLp {
    InnerLP(InnerLP),
    OuterLP(OuterLP),
}

// 内部存储 lp
#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct InnerLP {
    pub supply: Nat,  // 需要记录总 lp，新增和移除流动性时候需要按比例退回对应的代币
    pub decimals: u8, // 需要记录小数位数，显示需要
    pub fee: Nat,     // 需要记录手续费，转移时候需要用到
    pub minimum_liquidity: Nat, // 需要记录最小流动性，移除流动性时候需要检查是否达到最小流动性
}

// 外部存储 lp，是一个单独的罐子，有权限对其 mint 和 burn LP 代币，// ! 罐子手续费不应该销毁
#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct OuterLP {
    pub canister_id: CanisterId, // 需要记录外部的罐子 id

    pub supply: Nat,  // 需要记录总 lp，新增和移除流动性时候需要按比例退回对应的代币
    pub decimals: u8, // 需要记录小数位数，显示需要
    pub fee: Nat,     // 需要记录手续费，转移时候需要用到
    pub minimum_liquidity: Nat, // 需要记录最小流动性，移除流动性时候需要检查是否达到最小流动性
}

impl PoolLp {
    pub fn new_inner_lp(token0: &TokenInfo, token1: &TokenInfo) -> Self {
        let decimals = get_decimals(token0.decimals, token1.decimals);
        let fee = get_fee(&token0.fee, &token1.fee);

        // fee * 1000
        let minimum_liquidity = fee.clone() * Nat::from(1000_u64);

        Self::InnerLP(InnerLP {
            supply: Nat::from(0_u64),
            decimals,
            fee,
            minimum_liquidity,
        })
    }
}

fn get_decimals(decimals0: u8, decimals1: u8) -> u8 {
    let decimals = decimals0 + decimals1;
    decimals / 2 + if decimals % 2 == 0 { 0 } else { 1 }
}

fn get_fee(fee1: &Nat, fee2: &Nat) -> Nat {
    let fee1 = fee1.0.to_str_radix(10).len() - 1;
    let fee2 = fee2.0.to_str_radix(10).len() - 1;
    let size = fee1 + fee2;
    let size = size / 2 + if size % 2 == 0 { 0 } else { 1 };
    Nat::from(10_u64.pow(size as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let fee = get_fee(&Nat::from(10000_u64), &Nat::from(1000000_u64));
        assert_eq!(fee, Nat::from(100000_u64));

        let fee = get_fee(&Nat::from(10000_u64), &Nat::from(2000000_u64));
        assert_eq!(fee, Nat::from(100000_u64));
    }
}
