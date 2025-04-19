use std::cell::RefCell;

use ic_canister_kit::types::*;

use super::{InitArgs, UpgradeArgs};
use super::{State, State::*};

impl Default for State {
    fn default() -> Self {
        // ? Initialization and upgrade will be migrated first, so the initial version does not matter
        V0(Box::default())
    }
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
    });
}

// ==================== Save data before upgrade, would be execute next upgrade ====================

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        use ic_canister_kit::common::trap;

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

impl StableHeap for State {
    fn heap_to_bytes(&self) -> Vec<u8> {
        self.get().heap_to_bytes()
    }

    fn heap_from_bytes(&mut self, bytes: &[u8]) {
        self.get_mut().heap_from_bytes(bytes)
    }
}
