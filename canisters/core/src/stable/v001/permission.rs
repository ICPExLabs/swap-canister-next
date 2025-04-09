use std::collections::HashSet;

use ic_canister_kit::types::Permission;

use crate::stable::ParsePermissionError;

use super::super::check_permission;

use super::types::{InnerState, ParsePermission};

// 权限常量
// 通用权限
pub use super::super::v000::types::{
    ACTION_PAUSE_QUERY, ACTION_PAUSE_REPLACE, ACTION_PERMISSION_FIND, ACTION_PERMISSION_QUERY,
    ACTION_PERMISSION_UPDATE, ACTION_RECORD_FIND, ACTION_RECORD_MIGRATE, ACTION_SCHEDULE_FIND,
    ACTION_SCHEDULE_REPLACE, ACTION_SCHEDULE_TRIGGER,
};

// 业务权限
// config
pub const ACTION_BUSINESS_CONFIG_FEE_TO: &str = "BusinessConfigFeeTo"; // 查询和设置手续费接收地址权限
// token
pub const ACTION_BUSINESS_TOKEN_BALANCE_BY: &str = "BusinessTokenBalanceBy"; // 查询指定账户余额权限
pub const ACTION_BUSINESS_TOKEN_DEPOSIT: &str = "BusinessTokenDeposit"; // 存入代币权限
pub const ACTION_BUSINESS_TOKEN_WITHDRAW: &str = "BusinessTokenWithdraw"; // 提取代币权限
pub const ACTION_BUSINESS_TOKEN_TRANSFER: &str = "BusinessTokenTransfer"; // 代币内部转移权限
// pair
pub const ACTION_BUSINESS_TOKEN_PAIR_CREATE: &str = "BusinessTokenPairCreate"; // 创建代币对池子权限
pub const ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_ADD: &str = "BusinessTokenPairLiquidityAdd"; // 向代币对池子添加流动性权限
pub const ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_REMOVE: &str = "BusinessTokenPairLiquidityRemove"; // 向代币对池子移除流动性权限
pub const ACTION_BUSINESS_TOKEN_PAIR_SWAP: &str = "BusinessTokenPairSwap"; // 代币对池子交换代币权限
// example
pub const ACTION_BUSINESS_EXAMPLE_QUERY: &str = "BusinessExampleQuery"; // 业务查询权限
pub const ACTION_BUSINESS_EXAMPLE_SET: &str = "BusinessExampleSet"; // 业务更新权限

// 所有权限列表
#[allow(unused)]
pub const ACTIONS: [&str; 21] = [
    // 通用权限
    ACTION_PAUSE_QUERY,
    ACTION_PAUSE_REPLACE,
    ACTION_PERMISSION_QUERY,
    ACTION_PERMISSION_FIND,
    ACTION_PERMISSION_UPDATE,
    ACTION_RECORD_FIND,
    ACTION_RECORD_MIGRATE,
    ACTION_SCHEDULE_FIND,
    ACTION_SCHEDULE_REPLACE,
    ACTION_SCHEDULE_TRIGGER,
    // 业务权限
    // config
    ACTION_BUSINESS_CONFIG_FEE_TO,
    // token
    ACTION_BUSINESS_TOKEN_BALANCE_BY,
    ACTION_BUSINESS_TOKEN_DEPOSIT,
    ACTION_BUSINESS_TOKEN_WITHDRAW,
    ACTION_BUSINESS_TOKEN_TRANSFER,
    // pair
    ACTION_BUSINESS_TOKEN_PAIR_CREATE,
    ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_ADD,
    ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_REMOVE,
    ACTION_BUSINESS_TOKEN_PAIR_SWAP,
    // example
    ACTION_BUSINESS_EXAMPLE_QUERY,
    ACTION_BUSINESS_EXAMPLE_SET,
];

pub(super) fn get_all_permissions<'a, F>(parse: F) -> HashSet<Permission>
where
    F: Fn(&'a str) -> Result<Permission, ParsePermissionError<'a>>,
{
    use ic_canister_kit::functions::permission::basic::parse_all_permissions;
    let permissions = parse_all_permissions(&ACTIONS, parse);
    let permissions = ic_canister_kit::common::trap(permissions);
    permissions.into_iter().collect()
}

// 权限默认状态
impl ParsePermission for InnerState {
    fn parse_permission<'a>(&self, name: &'a str) -> Result<Permission, ParsePermissionError<'a>> {
        Ok(match name {
            // 通用权限
            ACTION_PAUSE_QUERY => Permission::by_forbid(name),
            ACTION_PAUSE_REPLACE => Permission::by_permit(name),
            ACTION_PERMISSION_QUERY => Permission::by_forbid(name),
            ACTION_PERMISSION_FIND => Permission::by_permit(name),
            ACTION_PERMISSION_UPDATE => Permission::by_permit(name),
            ACTION_RECORD_FIND => Permission::by_permit(name),
            ACTION_RECORD_MIGRATE => Permission::by_permit(name),
            ACTION_SCHEDULE_FIND => Permission::by_permit(name),
            ACTION_SCHEDULE_REPLACE => Permission::by_permit(name),
            ACTION_SCHEDULE_TRIGGER => Permission::by_permit(name),
            // 业务权限
            // config
            ACTION_BUSINESS_CONFIG_FEE_TO => Permission::by_permit(name),
            // token
            ACTION_BUSINESS_TOKEN_BALANCE_BY => Permission::by_permit(name),
            ACTION_BUSINESS_TOKEN_DEPOSIT => Permission::by_forbid(name), // default anyone
            ACTION_BUSINESS_TOKEN_WITHDRAW => Permission::by_forbid(name), // default anyone
            ACTION_BUSINESS_TOKEN_TRANSFER => Permission::by_forbid(name), // default anyone
            // pair
            ACTION_BUSINESS_TOKEN_PAIR_CREATE => Permission::by_permit(name),
            ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_ADD => Permission::by_forbid(name), // default anyone
            ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_REMOVE => Permission::by_forbid(name), // default anyone
            ACTION_BUSINESS_TOKEN_PAIR_SWAP => Permission::by_forbid(name), // default anyone
            // example
            ACTION_BUSINESS_EXAMPLE_QUERY => Permission::by_forbid(name),
            ACTION_BUSINESS_EXAMPLE_SET => Permission::by_permit(name),
            // 其他错误
            _ => return Err(ParsePermissionError(name)),
        })
    }
}

// 通用权限
#[allow(unused)]
pub use super::super::v000::types::{
    has_pause_query, has_pause_replace, has_permission_find, has_permission_query,
    has_permission_update, has_record_find, has_record_migrate, has_schedule_find,
    has_schedule_replace, has_schedule_trigger,
};

// 业务权限

// config
#[allow(unused)]
pub fn has_business_config_fee_to() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_CONFIG_FEE_TO, false)
}

// token
#[allow(unused)]
pub fn has_business_token_balance_by() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_TOKEN_BALANCE_BY, false)
}
#[allow(unused)]
pub fn has_business_token_deposit() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_TOKEN_DEPOSIT, true) // ! required running, not paused
}
#[allow(unused)]
pub fn has_business_token_withdraw() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_TOKEN_WITHDRAW, true) // ! required running, not paused
}
#[allow(unused)]
pub fn has_business_token_transfer() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_TOKEN_TRANSFER, true) // ! required running, not paused
}

// pair
#[allow(unused)]
pub fn has_business_token_pair_create() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_TOKEN_PAIR_CREATE, true) // ! required running, not paused
}
#[allow(unused)]
pub fn has_business_token_pair_liquidity_add() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_ADD, true) // ! required running, not paused
}
#[allow(unused)]
pub fn has_business_token_pair_liquidity_remove() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_REMOVE, true) // ! required running, not paused
}
#[allow(unused)]
pub fn has_business_token_pair_swap() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_TOKEN_PAIR_SWAP, true) // ! required running, not paused
}

// example
#[allow(unused)]
pub fn has_business_example_query() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_EXAMPLE_QUERY, false)
}
#[allow(unused)]
pub fn has_business_example_set() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_EXAMPLE_SET, true)
}
