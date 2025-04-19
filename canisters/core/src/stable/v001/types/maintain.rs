use std::collections::HashMap;

use candid::Nat;
use ic_canister_kit::types::CanisterId;
use serde::{Deserialize, Serialize};

use super::*;

// 默认最小阈值
const DEFAULT_MIN_CYCLES_THRESHOLD: u64 = 5_000_000_000_000; // 5 T cycles

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintainArchives {
    /// 记录所有罐子的充值数量
    recharged: HashMap<CanisterId, Nat>,
    /// 创建归档罐子的初始 cycles
    initial_cycles: u64,
    /// 罐子充值的最小 cycles
    min_cycles_threshold: u64,
    /// 每次触发充值数量
    recharge_cycles: u64,
    /// 检查的间隔时间 ns
    checking_interval_ns: u64,
    /// 上次检查时间 ns
    last_checked_timestamp: TimestampNanos,
}

impl Default for MaintainArchives {
    fn default() -> Self {
        Self {
            recharged: HashMap::new(),
            initial_cycles: DEFAULT_MIN_CYCLES_THRESHOLD,
            min_cycles_threshold: DEFAULT_MIN_CYCLES_THRESHOLD,
            recharge_cycles: DEFAULT_MIN_CYCLES_THRESHOLD,
            checking_interval_ns: 1_000_000 * 1000 * 3600 * 8, // 每 8 小时检查一次
            last_checked_timestamp: TimestampNanos::from_inner(0),
        }
    }
}

impl MaintainArchives {
    pub fn cycles_recharged(&mut self, canister_id: CanisterId, cycles: u128) {
        *(self.recharged.entry(canister_id).or_default()) += Nat::from(cycles);
    }
}
