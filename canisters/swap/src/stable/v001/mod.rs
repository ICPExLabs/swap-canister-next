use std::collections::{HashMap, HashSet};

use ic_canister_kit::{functions::permission::basic::supers_updated, identity::caller, types::*};

pub mod types;

mod upgrade;

mod permission;

mod schedule;

mod business;

use types::*;

// initialization
// ! The first deployment will be executed
impl Initial<Option<InitArg>> for InnerState {
    fn init(&mut self, arg: Option<InitArg>) {
        let arg = arg.unwrap_or_default(); // ! Even if it is None, it must be executed once

        // Maintenance personnel initialization
        let maintainers = arg.maintainers.clone().unwrap_or_else(|| {
            vec![caller()] // The default caller is the maintenance person
        });

        let permissions = get_all_permissions(|n| self.parse_permission(n));
        let updated = supers_updated(&maintainers, &permissions);

        // Refresh permissions
        self.permission_reset(permissions);
        // Maintenance personnel grant all permissions
        assert!(self.permission_update(updated).is_ok()); // Insert permissions

        // Timing tasks
        self.schedule_replace(arg.schedule);

        // Business data
        self.do_init(arg);
    }
}

// upgrade
// ! Execute during upgrade
impl Upgrade<Option<UpgradeArg>> for InnerState {
    fn upgrade(&mut self, arg: Option<UpgradeArg>) {
        let arg = match arg {
            Some(arg) => arg,
            None => return, // ! None means no data processing is required for upgrade
        };

        // Maintenance personnel initialization
        let maintainers = arg.maintainers.clone();

        let permissions = get_all_permissions(|n| self.parse_permission(n));
        let updated = maintainers
            .as_ref()
            .map(|maintainers| supers_updated(maintainers, &permissions));

        // Refresh permissions
        self.permission_reset(permissions);
        // Maintenance personnel grant all permissions
        if let Some(updated) = updated {
            assert!(self.permission_update(updated).is_ok()); // Insert permissions
        }

        // Timing tasks
        self.schedule_replace(arg.schedule);

        // Business data
        self.do_upgrade(arg);
    }
}

impl Pausable<PauseReason> for InnerState {
    // query
    fn pause_query(&self) -> &Option<PauseReason> {
        self.canister_kit.pause.pause_query()
    }
    //  update
    fn pause_replace(&mut self, reason: Option<PauseReason>) {
        self.canister_kit.pause.pause_replace(reason)
    }
}

impl Permissable<Permission> for InnerState {
    // query
    fn permission_users(&self) -> HashSet<&UserId> {
        self.canister_kit.permissions.permission_users()
    }
    fn permission_roles(&self) -> HashSet<&String> {
        self.canister_kit.permissions.permission_roles()
    }
    fn permission_assigned(&self, user_id: &UserId) -> Option<&HashSet<Permission>> {
        self.canister_kit.permissions.permission_assigned(user_id)
    }
    fn permission_role_assigned(&self, role: &str) -> Option<&HashSet<Permission>> {
        self.canister_kit.permissions.permission_role_assigned(role)
    }
    fn permission_user_roles(&self, user_id: &UserId) -> Option<&HashSet<String>> {
        self.canister_kit.permissions.permission_user_roles(user_id)
    }
    fn permission_has(&self, user_id: &UserId, permission: &Permission) -> bool {
        self.canister_kit
            .permissions
            .permission_has(user_id, permission)
    }
    fn permission_owned(&self, user_id: &UserId) -> HashMap<&Permission, bool> {
        self.canister_kit.permissions.permission_owned(user_id)
    }

    //  update
    fn permission_reset(&mut self, permissions: HashSet<Permission>) {
        self.canister_kit.permissions.permission_reset(permissions)
    }
    fn permission_update(
        &mut self,
        args: Vec<PermissionUpdatedArg<Permission>>,
    ) -> Result<(), PermissionUpdatedError<Permission>> {
        self.canister_kit.permissions.permission_update(args)
    }
}

impl Schedulable for InnerState {
    // query
    fn schedule_find(&self) -> Option<DurationNanos> {
        self.canister_kit.schedule.schedule_find()
    }
    //  update
    fn schedule_replace(&mut self, schedule: Option<DurationNanos>) {
        self.canister_kit.schedule.schedule_replace(schedule)
    }
}

impl ScheduleTask for InnerState {}

impl StableHeap for InnerState {
    fn heap_to_bytes(&self) -> Vec<u8> {
        let bytes = ic_canister_kit::functions::stable::to_bytes(self);
        ic_canister_kit::common::trap(bytes)
    }

    fn heap_from_bytes(&mut self, bytes: &[u8]) {
        let state = ic_canister_kit::functions::stable::from_bytes(bytes);
        *self = ic_canister_kit::common::trap(state);
    }
}
