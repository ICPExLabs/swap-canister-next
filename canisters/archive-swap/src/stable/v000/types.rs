use serde::{Deserialize, Serialize};

#[allow(unused)]
pub use super::super::Business;

#[allow(unused)]
pub use super::super::business::*;
#[allow(unused)]
pub use super::business::*;
#[allow(unused)]
pub use super::permission::*;

// Initialization parameters
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArg {}

// Upgrade parameters
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType)]
pub struct UpgradeArg {}

// Data structures required by the framework
#[derive(Serialize, Deserialize, Default)]
pub struct CanisterKit {}

// Put together those that can be serialized and those that cannot be serialized
// The following annotations are used for serialization
// #[serde(skip)] Default initialization method
// #[serde(skip, default="init_xxx_data")] Specify the initialization method
// ! If you use the stable memory provided by ic-stable-structures, the usage type of memory_id cannot be changed, otherwise each version will be incompatible and the data will be cleared
#[derive(Serialize, Deserialize)]
pub struct InnerState {
    pub canister_kit: CanisterKit, // Data required by the framework // ? Heap memory Serialization
}

impl Default for InnerState {
    fn default() -> Self {
        ic_cdk::println!("InnerState::default()");
        Self {
            canister_kit: Default::default(),
        }
    }
}

impl InnerState {
    pub fn do_init(&mut self, _arg: InitArg) {
        // maybe do something
    }

    pub fn do_upgrade(&mut self, _arg: UpgradeArg) {
        // maybe do something
    }
}
