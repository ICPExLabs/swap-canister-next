use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

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
}

// 外部存储 lp，是一个单独的罐子，有权限对其 mint 和 burn LP 代币，// ! 罐子手续费不应该销毁
#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct OuterLP {
    pub canister_id: CanisterId, // 需要记录外部的罐子 id

    pub supply: Nat,  // 需要记录总 lp，新增和移除流动性时候需要按比例退回对应的代币
    pub decimals: u8, // 需要记录小数位数，显示需要
    pub fee: Nat,     // 需要记录手续费，转移时候需要用到
}
