use std::collections::HashSet;

use ic_canister_kit::types::Permission;

use crate::stable::ParsePermissionError;

use super::super::check_permission;

use super::types::{InnerState, ParsePermission};

// Permission constants
// General permissions
pub const ACTION_PAUSE_QUERY: &str = "PauseQuery"; // Query maintenance status
pub const ACTION_PAUSE_REPLACE: &str = "PauseReplace"; // Set maintenance status
pub const ACTION_PERMISSION_QUERY: &str = "PermissionQuery"; // Query personal permission information
pub const ACTION_PERMISSION_FIND: &str = "PermissionFind"; // Query other people's permissions
pub const ACTION_PERMISSION_UPDATE: &str = "PermissionUpdate"; // Set permissions
pub const ACTION_SCHEDULE_FIND: &str = "ScheduleFind"; // Query the timing status
pub const ACTION_SCHEDULE_REPLACE: &str = "ScheduleReplace"; // Set the timing frequency
pub const ACTION_SCHEDULE_TRIGGER: &str = "ScheduleTrigger"; // Trigger timing tasks

// Business permissions

// All permission list
#[allow(unused)]
pub const ACTIONS: [&str; 8] = [
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

            // Other errors
            _ => return Err(ParsePermissionError(name)),
        })
    }
}

// General permissions

pub fn has_pause_query() -> Result<(), String> {
    check_permission(ACTION_PAUSE_QUERY, false)
}

pub fn has_pause_replace() -> Result<(), String> {
    check_permission(ACTION_PAUSE_REPLACE, false)
}

pub fn has_permission_query() -> Result<(), String> {
    check_permission(ACTION_PERMISSION_QUERY, false)
}

pub fn has_permission_find() -> Result<(), String> {
    check_permission(ACTION_PERMISSION_FIND, false)
}

pub fn has_permission_update() -> Result<(), String> {
    check_permission(ACTION_PERMISSION_UPDATE, false)
}

pub fn has_schedule_find() -> Result<(), String> {
    check_permission(ACTION_SCHEDULE_FIND, true)
}

pub fn has_schedule_replace() -> Result<(), String> {
    check_permission(ACTION_SCHEDULE_REPLACE, true)
}

pub fn has_schedule_trigger() -> Result<(), String> {
    check_permission(ACTION_SCHEDULE_TRIGGER, true)
}

// Business permissions
