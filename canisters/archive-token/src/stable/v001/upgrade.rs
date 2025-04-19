use super::super::v000::types::{CanisterKit as LastCanisterKit, InnerState as LastState};

use super::types::*;

impl From<Box<LastState>> for Box<InnerState> {
    fn from(value: Box<LastState>) -> Self {
        let state = InnerState::default(); // ? initialization

        // ! Every time you upgrade a new version, be sure to compare the upgrade method of each data.
        // ! If the data structure is not modified, you can directly assign a value and upgrade it
        // ! If the data structure is modified, the code must be processed to upgrade the data

        // 1. Restore previous data
        let LastCanisterKit {} = value.canister_kit;

        Box::new(state)
    }
}
