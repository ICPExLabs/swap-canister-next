use ic_canister_kit::types::{CanisterId, UserId};

use crate::proto;

impl From<CanisterId> for proto::CanisterId {
    fn from(value: CanisterId) -> Self {
        proto::CanisterId {
            bytes: value.as_slice().to_vec().into(),
        }
    }
}

impl From<proto::CanisterId> for CanisterId {
    fn from(value: proto::CanisterId) -> Self {
        CanisterId::from_slice(&value.bytes)
    }
}

impl From<UserId> for proto::UserId {
    fn from(value: UserId) -> Self {
        proto::UserId {
            bytes: value.as_slice().to_vec().into(),
        }
    }
}

impl From<proto::UserId> for UserId {
    fn from(value: proto::UserId) -> Self {
        UserId::from_slice(&value.bytes)
    }
}
