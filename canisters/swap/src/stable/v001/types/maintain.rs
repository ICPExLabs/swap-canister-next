use std::collections::HashMap;

use candid::Nat;
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

use super::*;

// Default minimum threshold
const DEFAULT_MIN_CYCLES_THRESHOLD: u64 = 5_000_000_000_000; // 5 T cycles

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct MaintainArchives {
    /// Record the recharge amount of all canisters
    recharged: HashMap<CanisterId, Nat>,
    /// Minimum cycles for canister trigger recharge
    pub min_cycles_threshold: u64,
    /// Number of recharges per trigger
    pub recharge_cycles: u64,
    /// Check interval ns
    pub checking_interval_ns: u64,
    /// Last check time ns
    pub last_checked_timestamp: TimestampNanos,
}

impl Default for MaintainArchives {
    fn default() -> Self {
        Self {
            recharged: HashMap::new(),
            min_cycles_threshold: DEFAULT_MIN_CYCLES_THRESHOLD,
            recharge_cycles: DEFAULT_MIN_CYCLES_THRESHOLD,
            checking_interval_ns: 1_000_000 * 1000 * 3600 * 8, // Check every 8 hours
            last_checked_timestamp: TimestampNanos::from_inner(0),
        }
    }
}

impl MaintainArchives {
    pub fn update_config(&mut self, config: MaintainArchivesConfig) {
        self.min_cycles_threshold = config.min_cycles_threshold;
        self.recharge_cycles = config.recharge_cycles;
        self.checking_interval_ns = config.checking_interval_ns;
    }

    pub fn is_trigger(&mut self, now: TimestampNanos) -> bool {
        if self.last_checked_timestamp.into_inner() + self.checking_interval_ns < now.into_inner() {
            self.last_checked_timestamp = now;
            return true;
        }
        false
    }

    pub fn cycles_recharged(&mut self, canister_id: CanisterId, cycles: u128) {
        *(self.recharged.entry(canister_id).or_default()) += Nat::from(cycles);
    }
}
