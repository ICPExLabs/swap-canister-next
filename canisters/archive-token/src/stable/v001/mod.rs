use ic_canister_kit::types::*;

pub mod types;

mod upgrade;

mod permission;

mod schedule;

mod business;

use types::*;

// initialization
// ! The first deployment will be executed
impl Initial<Option<InitArgV1>> for InnerState {
    fn init(&mut self, arg: Option<InitArgV1>) {
        let arg = arg.unwrap_or_default(); // ! Even if it is None, it must be executed once

        // Business data
        self.do_init(arg);
    }
}

// upgrade
// ! Execute during upgrade
impl Upgrade<Option<UpgradeArgV1>> for InnerState {
    fn upgrade(&mut self, arg: Option<UpgradeArgV1>) {
        let arg = match arg {
            Some(arg) => arg,
            None => return, // ! None means no data processing is required for upgrade
        };

        // Business data
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
