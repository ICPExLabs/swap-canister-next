use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, CandidType)]
pub struct FeeTo {
    /// 防止女巫攻击收取的手续费账户
    /// token 的 transfer 使用
    /// liquidity 的 burn 使用
    pub token_fee_to: Option<Account>,
    /// 收取 swap 协议费使用，1/6
    pub swap_fee_to: Option<Account>,
}
