use ic_canister_kit::identity::caller;

use crate::stable::*;
use crate::types::*;

// ================== general apis ==================

#[ic_cdk::query]
pub fn wallet_balance() -> candid::Nat {
    ic_canister_kit::canister::cycles::wallet_balance()
}

#[ic_cdk::update]
pub fn wallet_receive() -> candid::Nat {
    ic_canister_kit::canister::cycles::wallet_receive(|_accepted| {})
}

// ================== current data version ==================

// current data version
#[ic_cdk::query]
fn version() -> u32 {
    with_state(|s| s.version())
}

// query memory
#[ic_cdk::query]
fn memory_size_heap() -> Nat {
    candid::Nat::from(::common::utils::runtime::heap_memory_size_bytes() as u64)
}

// query memory
#[ic_cdk::query]
fn memory_size_stable() -> Nat {
    candid::Nat::from(ic_cdk::stable::stable_size() * 64 * 1024)
}

// ================== pause apis ==================

// Query maintenance status
#[ic_cdk::query(guard = "has_pause_query")]
fn pause_query() -> bool {
    with_state(|s| s.pause_is_paused())
}

// Query maintenance status
#[ic_cdk::query(guard = "has_pause_query")]
fn pause_query_reason() -> Option<PauseReason> {
    with_state(|s| s.pause_query().clone())
}

// Set maintenance status
#[ic_cdk::update(guard = "has_pause_replace")]
fn pause_replace(reason: Option<String>) {
    let old = with_state(|s| s.pause_query().clone());

    if old.is_none() && reason.is_none() {
        return; // Unchanged content
    }

    with_mut_state(|s| {
        s.pause_replace(reason.map(PauseReason::new));
    })
}

// ================== permission apis ==================

// all permissions
#[ic_cdk::query(guard = "has_permission_query")]
fn permission_all() -> Vec<Permission> {
    use ic_canister_kit::common::trap;
    use ic_canister_kit::functions::permission::basic::parse_all_permissions;
    with_state(|s| {
        let permissions = parse_all_permissions(&ACTIONS, |name| s.parse_permission(name));
        trap(permissions.map_err(|e| e.to_string()))
    })
}

// Query your own permissions
#[ic_cdk::query(guard = "has_permission_query")]
fn permission_query() -> Vec<&'static str> {
    permission_find_by_user(ic_canister_kit::identity::caller())
}

// Query other people's permissions
#[ic_cdk::query(guard = "has_permission_find")]
fn permission_find_by_user(user_id: UserId) -> Vec<&'static str> {
    with_state(|s| {
        use ic_canister_kit::common::trap;
        use ic_canister_kit::functions::permission::basic::parse_all_permissions;
        let permissions = parse_all_permissions(&ACTIONS, |name| s.parse_permission(name));
        trap(permissions)
            .iter()
            .zip(ACTIONS)
            .filter(|(permission, _)| s.permission_has(&user_id, permission))
            .map(|(_, p)| p)
            .collect()
    })
}

// Query your own specified permissions
#[ic_cdk::query(guard = "has_permission_query")]
fn permission_assigned_query() -> Option<HashSet<Permission>> {
    permission_assigned_by_user(ic_canister_kit::identity::caller())
}

// Query other people's specified permissions
#[ic_cdk::query(guard = "has_permission_find")]
fn permission_assigned_by_user(user_id: UserId) -> Option<HashSet<Permission>> {
    with_state(|s| s.permission_assigned(&user_id).cloned())
}

// all roles
#[ic_cdk::query(guard = "has_permission_query")]
fn permission_roles_all() -> HashMap<String, HashSet<Permission>> {
    with_state(|s| {
        s.permission_roles()
            .into_iter()
            .map(|role| {
                (
                    role.to_owned(),
                    s.permission_role_assigned(role).cloned().unwrap_or_default(),
                )
            })
            .collect()
    })
}

// Query your role
#[ic_cdk::query(guard = "has_permission_query")]
fn permission_roles_query() -> Option<HashSet<String>> {
    permission_roles_by_user(ic_canister_kit::identity::caller())
}

// Query other people's roles
#[ic_cdk::query(guard = "has_permission_find")]
fn permission_roles_by_user(user_id: UserId) -> Option<HashSet<String>> {
    with_state(|s| s.permission_user_roles(&user_id).cloned())
}

// Update permissions
#[ic_cdk::update(guard = "has_permission_update")]
fn permission_update(args: Vec<PermissionUpdatedArg<String>>) {
    with_mut_state(|s| {
        use ic_canister_kit::common::trap;
        let args = args
            .into_iter()
            .map(|a| a.parse_permission(|n| s.parse_permission(n).map_err(|e| e.to_string())))
            .collect::<Result<Vec<_>, _>>();
        let args = trap(args);
        trap(s.permission_update(args))
    })
}

// ================== Schedules tasks ==================

// Query the timing status
#[ic_cdk::query(guard = "has_schedule_find")]
fn schedule_find() -> Option<u64> {
    with_state(|s| s.schedule_find().map(|s| s.into_inner() as u64))
}

// Set the timing status
#[ic_cdk::update(guard = "has_schedule_replace")]
fn schedule_replace(schedule: Option<u64>) {
    with_mut_state(|s| {
        s.schedule_replace(schedule.map(|s| (s as u128).into()));
        s.schedule_reload(); // * Reset timing tasks
    })
}

// Manually trigger timing tasks
#[ic_cdk::update(guard = "has_schedule_trigger")]
async fn schedule_trigger() {
    let caller = caller();

    assert!(with_mut_state(|s| { s.pause_must_be_running() }).is_ok()); // Tasks are not allowed during maintenance

    schedule_task(Some(caller)).await;
}
