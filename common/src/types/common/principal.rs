use candid::CandidType;
#[cfg(feature = "cdk")]
use ic_canister_kit::identity::self_canister_id;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::types::{CanisterId, UserId};

#[allow(unused)]
use super::BusinessError;

/// The canister itself
#[derive(Debug, Clone, Copy, Serialize, Deserialize, CandidType)]
pub struct SelfCanister(CanisterId);

impl SelfCanister {
    /// id
    pub fn id(&self) -> CanisterId {
        self.0
    }
}

/// caller
#[derive(Debug, Clone, Copy, Serialize, Deserialize, CandidType)]
pub struct Caller(UserId);

impl Caller {
    /// get
    #[cfg(feature = "cdk")]
    pub fn get() -> Self {
        Self(ic_canister_kit::identity::caller())
    }

    /// id
    #[allow(unused)]
    pub fn id(&self) -> UserId {
        self.0
    }

    /// ! check token fee to required or not
    pub fn fee_to(&self, fee_to: Option<Account>, to: Account) -> Option<Account> {
        if fee_to.is_some_and(|fee_to| fee_to.owner == self.0 || fee_to == to) {
            return None;
        }
        fee_to
    }
}

/// Check whether the caller is consistent. If the caller is a self canister, the default owner is correct.
#[cfg(feature = "cdk")]
pub fn check_caller(owner: &UserId) -> Result<(SelfCanister, Caller), BusinessError> {
    let self_canister_id = self_canister_id();
    let mut caller = ic_canister_kit::identity::caller();
    if caller == self_canister_id {
        caller = owner.to_owned(); // swap canister is called on behalf of
    } else if caller != *owner {
        return Err(BusinessError::NotOwner(*owner));
    }
    Ok((SelfCanister(self_canister_id), Caller(caller)))
}

/// Simulate the pool's LP tokens
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, CandidType)]
pub struct DummyCanisterId(CanisterId);
impl DummyCanisterId {
    /// new
    pub fn new(id: CanisterId) -> Self {
        Self(id)
    }

    /// get id
    pub fn id(&self) -> CanisterId {
        self.0
    }
}
