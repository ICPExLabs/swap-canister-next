use ic_canister_kit::types::*;

pub mod types;

mod upgrade;

mod permission;

mod schedule;

mod business;

use types::*;

// 初始化
// ! 第一次部署会执行
impl Initial<Option<InitArg>> for InnerState {
    fn init(&mut self, arg: Option<InitArg>) {
        let arg = arg.unwrap_or_default(); // ! 就算是 None，也要执行一次

        // 业务数据
        self.do_init(arg);
    }
}

// 升级
// ! 升级时执行
impl Upgrade<Option<UpgradeArg>> for InnerState {
    fn upgrade(&mut self, arg: Option<UpgradeArg>) {
        let arg = match arg {
            Some(arg) => arg,
            None => return, // ! None 表示升级无需处理数据
        };

        // 业务数据
        self.do_upgrade(arg);
    }
}

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
