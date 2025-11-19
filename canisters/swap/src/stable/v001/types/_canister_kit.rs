use ic_canister_kit::types::*;
use serde::{Deserialize, Serialize};

// Data structures required by the framework
#[derive(Serialize, Deserialize, Default)]
pub struct CanisterKit {
    pub pause: Pause,             // Record maintenance status //  ? Heap memory Serialization
    pub permissions: Permissions, // Record your own permissions //  ? Heap memory Serialization
    pub schedule: Schedule,       // Record timing tasks //  ? Heap memory Serialization
}
