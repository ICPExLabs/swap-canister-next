use std::collections::HashSet;

use ic_canister_kit::types::Permission;

use crate::stable::ParsePermissionError;

use super::super::check_permission;

use super::types::{InnerState, ParsePermission};

// Permission constants
// General permissions
pub use super::super::v000::types::{
    ACTION_PAUSE_QUERY, ACTION_PAUSE_REPLACE, ACTION_PERMISSION_FIND, ACTION_PERMISSION_QUERY,
    ACTION_PERMISSION_UPDATE, ACTION_SCHEDULE_FIND, ACTION_SCHEDULE_REPLACE, ACTION_SCHEDULE_TRIGGER,
};

// Business permissions
// config
pub const ACTION_BUSINESS_CONFIG_FEE_TO: &str = "BusinessConfigFeeTo"; // Query and set the handling fee receiving address permissions
pub const ACTION_BUSINESS_CONFIG_CUSTOM_TOKEN: &str = "BusinessConfigCustomToken"; // put custom token
pub const ACTION_BUSINESS_CONFIG_MAINTAINING: &str = "BusinessConfigMaintaining"; // Maintain permissions
// token
pub const ACTION_BUSINESS_TOKEN_BALANCE_BY: &str = "BusinessTokenBalanceBy"; // Query the permissions for the specified account balance
pub const ACTION_BUSINESS_TOKEN_DEPOSIT: &str = "BusinessTokenDeposit"; // Deposit token permission
pub const ACTION_BUSINESS_TOKEN_WITHDRAW: &str = "BusinessTokenWithdraw"; // Retrieve token permissions
pub const ACTION_BUSINESS_TOKEN_TRANSFER: &str = "BusinessTokenTransfer"; // Internal transfer permissions for tokens
// pair
pub const ACTION_BUSINESS_TOKEN_PAIR_CREATE_OR_REMOVE: &str = "BusinessTokenPairCreateOrRemove"; // Create or Remove token pair pool permissions
pub const ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_ADD: &str = "BusinessTokenPairLiquidityAdd"; // Add liquidity permissions to the token pair pool
pub const ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_REMOVE: &str = "BusinessTokenPairLiquidityRemove"; // Remove liquidity permissions from token pair pools
pub const ACTION_BUSINESS_TOKEN_PAIR_SWAP: &str = "BusinessTokenPairSwap"; // Token exchange permissions for tokens on pools

// All permission list
#[allow(unused)]
pub const ACTIONS: [&str; 19] = [
    // General permissions
    ACTION_PAUSE_QUERY,
    ACTION_PAUSE_REPLACE,
    ACTION_PERMISSION_QUERY,
    ACTION_PERMISSION_FIND,
    ACTION_PERMISSION_UPDATE,
    ACTION_SCHEDULE_FIND,
    ACTION_SCHEDULE_REPLACE,
    ACTION_SCHEDULE_TRIGGER,
    // Business permissions
    // config
    ACTION_BUSINESS_CONFIG_FEE_TO,
    ACTION_BUSINESS_CONFIG_CUSTOM_TOKEN,
    ACTION_BUSINESS_CONFIG_MAINTAINING,
    // token
    ACTION_BUSINESS_TOKEN_BALANCE_BY,
    ACTION_BUSINESS_TOKEN_DEPOSIT,
    ACTION_BUSINESS_TOKEN_WITHDRAW,
    ACTION_BUSINESS_TOKEN_TRANSFER,
    // pair
    ACTION_BUSINESS_TOKEN_PAIR_CREATE_OR_REMOVE,
    ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_ADD,
    ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_REMOVE,
    ACTION_BUSINESS_TOKEN_PAIR_SWAP,
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

// Permission default status
impl ParsePermission for InnerState {
    fn parse_permission<'a>(&self, name: &'a str) -> Result<Permission, ParsePermissionError<'a>> {
        Ok(match name {
            // General permissions
            ACTION_PAUSE_QUERY => Permission::by_forbid(name),
            ACTION_PAUSE_REPLACE => Permission::by_permit(name),
            ACTION_PERMISSION_QUERY => Permission::by_forbid(name),
            ACTION_PERMISSION_FIND => Permission::by_permit(name),
            ACTION_PERMISSION_UPDATE => Permission::by_permit(name),
            ACTION_SCHEDULE_FIND => Permission::by_permit(name),
            ACTION_SCHEDULE_REPLACE => Permission::by_permit(name),
            ACTION_SCHEDULE_TRIGGER => Permission::by_permit(name),
            // Business permissions
            // config
            ACTION_BUSINESS_CONFIG_FEE_TO => Permission::by_permit(name),
            ACTION_BUSINESS_CONFIG_CUSTOM_TOKEN => Permission::by_permit(name),
            ACTION_BUSINESS_CONFIG_MAINTAINING => Permission::by_permit(name),
            // token
            ACTION_BUSINESS_TOKEN_BALANCE_BY => Permission::by_permit(name),
            ACTION_BUSINESS_TOKEN_DEPOSIT => Permission::by_forbid(name), // default anyone
            ACTION_BUSINESS_TOKEN_WITHDRAW => Permission::by_forbid(name), // default anyone
            ACTION_BUSINESS_TOKEN_TRANSFER => Permission::by_forbid(name), // default anyone
            // pair
            ACTION_BUSINESS_TOKEN_PAIR_CREATE_OR_REMOVE => Permission::by_permit(name),
            ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_ADD => Permission::by_forbid(name), // default anyone
            ACTION_BUSINESS_TOKEN_PAIR_LIQUIDITY_REMOVE => Permission::by_forbid(name), // default anyone
            ACTION_BUSINESS_TOKEN_PAIR_SWAP => Permission::by_forbid(name),          // default anyone
            // Other errors
            _ => return Err(ParsePermissionError(name)),
        })
    }
}

// General permissions
#[allow(unused)]
pub use super::super::v000::types::{
    has_pause_query, has_pause_replace, has_permission_find, has_permission_query, has_permission_update,
    has_schedule_find, has_schedule_replace, has_schedule_trigger,
};

// Business permissions
pub fn has_business_token_queryable() -> Result<(), String> {
    use super::super::Business;
    let caller = ic_canister_kit::identity::caller();
    crate::types::with_state(|s| s.business_token_queryable(&caller))
}
pub fn has_business_swap_queryable() -> Result<(), String> {
    use super::super::Business;
    let caller = ic_canister_kit::identity::caller();
    crate::types::with_state(|s| s.business_swap_queryable(&caller))
}

// config
#[allow(unused)]
pub fn has_business_config_fee_to() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_CONFIG_FEE_TO, false)
}
#[allow(unused)]
pub fn has_business_config_custom_token() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_CONFIG_CUSTOM_TOKEN, false)
}
#[allow(unused)]
pub fn has_business_config_maintaining() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_CONFIG_MAINTAINING, false)
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
pub fn has_business_token_pair_create_or_remove() -> Result<(), String> {
    check_permission(ACTION_BUSINESS_TOKEN_PAIR_CREATE_OR_REMOVE, true) // ! required running, not paused
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
