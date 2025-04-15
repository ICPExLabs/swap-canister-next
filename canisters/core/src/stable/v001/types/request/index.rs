use std::borrow::Cow;

use ic_canister_kit::types::{Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
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
