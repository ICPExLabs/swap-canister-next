use candid::CandidType;
use serde::{Deserialize, Serialize};

mod common;
pub use common::*;

mod business;
pub use business::*;

// ==================== The following code needs to be modified for the update version ====================

mod v000;
mod v001;

// ! It should be the latest version here
// !     👇👇 UPGRADE WARNING: Must be the current version of the code
pub use v001::types::*;

pub enum State {
    V0(Box<v000::types::InnerState>),
    V1(Box<v001::types::InnerState>),
    // *    👆👆 UPGRADE WARNING: import the new version
}
use State::*;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum InitArgs {
    V0(Box<v000::types::InitArg>),
    V1(Box<v001::types::InitArgV1>),
    // *    👆👆 UPGRADE WARNING: import the new version
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum UpgradeArgs {
    V0(Box<v000::types::UpgradeArg>),
    V1(Box<v001::types::UpgradeArgV1>),
    // *    👆👆 UPGRADE WARNING: import the new version
}

// initialization
impl Initial<Option<InitArgs>> for State {
    fn init(&mut self, args: Option<InitArgs>) {
        match args {
            Some(args) => match (self, args) {
                (V0(s), InitArgs::V0(arg)) => s.init(Some(*arg)),
                (V1(s), InitArgs::V1(arg)) => s.init(Some(*arg)),
                // ! 👆👆 The new version requires the default data to be added
                _ => ic_cdk::trap("version mismatched"),
            },
            None => match self {
                V0(s) => s.init(None),
                V1(s) => s.init(None),
            },
        }
    }
}

// upgrade
impl Upgrade<Option<UpgradeArgs>> for State {
    fn upgrade(&mut self, args: Option<UpgradeArgs>) {
        'outer: loop {
            // Perform upgrade operations and continue to upgrade to the next version
            match self {
                V0(s) => *self = V1(std::mem::take(&mut *s).into()), // -> V1
                V1(_) => break 'outer,                               // same version do nothing
            }
        }

        // handle args
        match args {
            Some(args) => {
                match (self, args) {
                    (V0(s), UpgradeArgs::V0(arg)) => s.upgrade(Some(*arg)),
                    (V1(s), UpgradeArgs::V1(arg)) => s.upgrade(Some(*arg)),
                    // ! 👆👆 The new version requires the default data to be added
                    _ => ic_cdk::trap("version mismatched"),
                }
            }
            None => match self {
                V0(s) => s.upgrade(None),
                V1(s) => s.upgrade(None),
            },
        }
    }
}

impl StateUpgrade<Option<UpgradeArgs>> for State {
    fn version(&self) -> u32 {
        // Version number of each version
        match self {
            V0(_) => 0,
            V1(_) => 1,
            // *   👆👆! The version number needs to be added here for the upgrade
        }
    }

    fn from_version(version: u32) -> Self {
        match version {
            0 => V0(Box::default()), // * initialization
            1 => V1(Box::default()), // * initialization
            // ! 👆👆 The new version requires the default data to be added
            _ => ic_cdk::trap("unsupported version"),
        }
    }
}

// ================== get ==================

impl State {
    pub fn get(&self) -> &dyn Business {
        match self {
            V0(s) => s.as_ref(), // * Get immutable state
            V1(s) => s.as_ref(), // * Get immutable state
        }
    }
    pub fn get_mut(&mut self) -> &mut dyn Business {
        match self {
            V0(s) => s.as_mut(), // * Get mutable state
            V1(s) => s.as_mut(), // * Get mutable state
        }
    }
}
