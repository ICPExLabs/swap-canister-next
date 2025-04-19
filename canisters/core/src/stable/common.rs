use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use ic_canister_kit::types::*;

use super::{InitArgs, ParsePermission, ParsePermissionError, UpgradeArgs, schedule_task};
use super::{State, State::*};

impl Default for State {
    fn default() -> Self {
        // ? Initialization and upgrade will be migrated first, so the initial version does not matter
        V0(Box::default())
    }
}

/// Check if you have a certain permission
pub fn check_permission(
    permission: &str,
    running: bool, // Whether it is required to be in normal operation
) -> Result<(), String> {
    let caller = ic_canister_kit::identity::caller();
    with_state(|s| {
        let _permission = s.parse_permission(permission).map_err(|e| e.to_string())?;
        if s.permission_has(&caller, &_permission) {
            if running {
                s.pause_must_be_running()?;
            }
            return Ok(());
        }
        Err(format!("Permission '{}' is required", permission))
    })
}

// ================= Data that needs to be persisted ================

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

// ==================== initial ====================

#[ic_cdk::init]
fn initial(args: Option<InitArgs>) {
    with_mut_state(|s| {
        s.upgrade(None); // upgrade to latest version
        s.init(args);
        s.schedule_reload(); // * Reset timing tasks
    })
}

// ==================== post upgrade ====================

#[ic_cdk::post_upgrade]
fn post_upgrade(args: Option<UpgradeArgs>) {
    STATE.with(|state| {
        let memory = ic_canister_kit::stable::get_upgrades_memory();
        let mut memory = ReadUpgradeMemory::new(&memory);

        let version = memory.read_u32(); // restore version
        let mut bytes = vec![0; memory.read_u64() as usize];
        memory.read(&mut bytes); // restore data

        // Restore the previous version using the version number
        let mut last_state = State::from_version(version);
        last_state.heap_from_bytes(&bytes); // Recovery data
        *state.borrow_mut() = last_state;

        state.borrow_mut().upgrade(args); // ! After recovery, upgrade to the latest version
        state.borrow_mut().schedule_reload(); // * Reset timing tasks
    });
}

// ==================== Save data before upgrade, would be execute next upgrade ====================

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        use ic_canister_kit::common::trap;
        trap(state.borrow().pause_must_be_paused()); // ! Must be in maintenance state before upgrading
        state.borrow_mut().schedule_stop(); // * Stop timing tasks

        let version = state.borrow().version();
        let bytes = state.borrow().heap_to_bytes();

        let mut memory = ic_canister_kit::stable::get_upgrades_memory();
        let mut memory = WriteUpgradeMemory::new(&mut memory);

        trap(memory.write_u32(version)); // store version
        trap(memory.write_u64(bytes.len() as u64)); // store heap data length
        trap(memory.write(&bytes)); // store heap data length
    });
}

// ==================== utils ====================

/// immutable system data
#[allow(unused)]
pub fn with_state<F, R>(callback: F) -> R
where
    F: FnOnce(&State) -> R,
{
    STATE.with(|state| {
        let state = state.borrow(); // immutable data
        callback(&state)
    })
}

///  mutable system data
#[allow(unused)]
pub fn with_mut_state<F, R>(callback: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|state| {
        let mut state = state.borrow_mut(); // mutable data
        callback(&mut state)
    })
}

impl Pausable<PauseReason> for State {
    // get
    fn pause_query(&self) -> &Option<PauseReason> {
        self.get().pause_query()
    }
    // replace
    fn pause_replace(&mut self, reason: Option<PauseReason>) {
        self.get_mut().pause_replace(reason)
    }
}

impl ParsePermission for State {
    fn parse_permission<'a>(&self, name: &'a str) -> Result<Permission, ParsePermissionError<'a>> {
        self.get().parse_permission(name)
    }
}

impl Permissable<Permission> for State {
    // query
    fn permission_users(&self) -> HashSet<&UserId> {
        self.get().permission_users()
    }
    fn permission_roles(&self) -> HashSet<&String> {
        self.get().permission_roles()
    }
    fn permission_assigned(&self, user_id: &UserId) -> Option<&HashSet<Permission>> {
        self.get().permission_assigned(user_id)
    }
    fn permission_role_assigned(&self, role: &str) -> Option<&HashSet<Permission>> {
        self.get().permission_role_assigned(role)
    }
    fn permission_user_roles(&self, user_id: &UserId) -> Option<&HashSet<String>> {
        self.get().permission_user_roles(user_id)
    }
    fn permission_has(&self, user_id: &UserId, permission: &Permission) -> bool {
        self.get().permission_has(user_id, permission)
    }
    fn permission_owned(&self, user_id: &UserId) -> HashMap<&Permission, bool> {
        self.get().permission_owned(user_id)
    }

    // update
    fn permission_reset(&mut self, permissions: HashSet<Permission>) {
        self.get_mut().permission_reset(permissions)
    }
    fn permission_update(
        &mut self,
        args: Vec<PermissionUpdatedArg<Permission>>,
    ) -> Result<(), PermissionUpdatedError<Permission>> {
        self.get_mut().permission_update(args)
    }
}

impl Schedulable for State {
    // query
    fn schedule_find(&self) -> Option<DurationNanos> {
        self.get().schedule_find()
    }
    // update
    fn schedule_replace(&mut self, schedule: Option<DurationNanos>) {
        self.get_mut().schedule_replace(schedule)
    }
}

#[allow(unused)]
fn static_schedule_task() {
    if with_state(|s| s.pause_is_paused()) {
        return; // Tasks are not allowed during maintenance
    }

    ic_cdk::spawn(async move { schedule_task(None).await });
}

pub trait ScheduleTask: Schedulable {
    fn schedule_stop(&self) {
        ic_canister_kit::functions::schedule::stop_schedule();
    }
    fn schedule_reload(&mut self) {
        let schedule = self.schedule_find();
        ic_canister_kit::functions::schedule::start_schedule(&schedule, static_schedule_task);
    }
}

impl ScheduleTask for State {}

impl StableHeap for State {
    fn heap_to_bytes(&self) -> Vec<u8> {
        self.get().heap_to_bytes()
    }

    fn heap_from_bytes(&mut self, bytes: &[u8]) {
        self.get_mut().heap_from_bytes(bytes)
    }
}
