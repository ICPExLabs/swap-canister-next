use ic_canister_kit::{
    identity::self_canister_id,
    types::{CanisterId, UserId},
};

use super::BusinessError;

#[derive(Debug, Clone, Copy)]
pub struct SelfCanister(CanisterId);

impl SelfCanister {
    pub fn id(&self) -> CanisterId {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Caller(UserId);

impl Caller {
    #[allow(unused)]
    pub fn id(&self) -> UserId {
        self.0
    }
}

pub fn check_caller(owner: &UserId) -> Result<(SelfCanister, Caller), BusinessError> {
    let self_canister_id = self_canister_id();
    let mut caller = ic_canister_kit::identity::caller();
    if caller == self_canister_id {
        caller = owner.to_owned(); // swap canister 代为调用
    } else if caller != *owner {
        return Err(BusinessError::NotOwner(*owner));
    }
    Ok((SelfCanister(self_canister_id), Caller(caller)))
}
