use std::{
    borrow::Cow,
    ops::{Add, AddAssign},
};

use candid::CandidType;
use ic_canister_kit::{
    common::trap,
    types::{Bound, Storable},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, CandidType)]
pub struct RequestIndex(u64);

impl Storable for RequestIndex {
    fn to_bytes(&self) -> Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(u64::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 8,
        is_fixed_size: true,
    };
}

impl RequestIndex {
    pub(super) fn previous(&self, length: u64) -> Self {
        let previous = trap(self.0.checked_sub(length).ok_or(format!(
            "can not get previous request index by next: {} and length: {length}",
            self.0
        )));
        Self(previous)
    }

    pub(super) fn next(&mut self) -> Self {
        let current = self.0;
        self.0 += 1;
        Self(current)
    }
}

impl AsRef<u64> for RequestIndex {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl Add<u64> for RequestIndex {
    type Output = RequestIndex;
    fn add(mut self, rhs: u64) -> Self::Output {
        self.0 += rhs;
        self
    }
}

impl AddAssign<u64> for RequestIndex {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}
