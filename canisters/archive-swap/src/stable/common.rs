use std::cell::RefCell;

use ic_canister_kit::types::*;

use super::{InitArgs, UpgradeArgs};
use super::{State, State::*};

// 默认值
impl Default for State {
    fn default() -> Self {
        // ? 初始化和升级会先进行迁移, 因此最初的版本无关紧要
        V0(Box::default())
    }
}

// ================= 需要持久化的数据 ================

thread_local! {
    static STATE: RefCell<State> = RefCell::default();// 存储系统数据
}

// ==================== 初始化方法 ====================

#[ic_cdk::init]
fn initial(args: Option<InitArgs>) {
    with_mut_state(|s| {
        s.upgrade(None); // upgrade to latest version
        s.init(args);
    })
}

// ==================== 升级时的恢复逻辑 ====================

#[ic_cdk::post_upgrade]
fn post_upgrade(args: Option<UpgradeArgs>) {
    STATE.with(|state| {
        let memory = ic_canister_kit::stable::get_upgrades_memory();
        let mut memory = ReadUpgradeMemory::new(&memory);

        let version = memory.read_u32(); // restore version
        let mut bytes = vec![0; memory.read_u64() as usize];
        memory.read(&mut bytes); // restore data

        // 利用版本号恢复升级前的版本
        let mut last_state = State::from_version(version);
        last_state.heap_from_bytes(&bytes); // 恢复数据
        *state.borrow_mut() = last_state;

        state.borrow_mut().upgrade(args); // ! 恢复后要进行升级到最新版本
    });
}

// ==================== 升级时的保存逻辑，下次升级执行 ====================

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

// ==================== 工具方法 ====================

/// 外界需要系统状态时
#[allow(unused)]
pub fn with_state<F, R>(callback: F) -> R
where
    F: FnOnce(&State) -> R,
{
    STATE.with(|state| {
        let state = state.borrow(); // 取得不可变对象
        callback(&state)
    })
}

/// 需要可变系统状态时
#[allow(unused)]
pub fn with_mut_state<F, R>(callback: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|state| {
        let mut state = state.borrow_mut(); // 取得可变对象
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
