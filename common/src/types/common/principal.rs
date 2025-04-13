use candid::CandidType;
use ic_canister_kit::{
    identity::self_canister_id,
    types::{CanisterId, UserId},
};
use serde::{Deserialize, Serialize};

use super::BusinessError;

/// 罐子自身
#[derive(Debug, Clone, Copy)]
pub struct SelfCanister(CanisterId);

impl SelfCanister {
    /// id
    pub fn id(&self) -> CanisterId {
        self.0
    }
}

/// 调用者
#[derive(Debug, Clone, Copy)]
pub struct Caller(UserId);

impl Caller {
    /// id
    #[allow(unused)]
    pub fn id(&self) -> UserId {
        self.0
    }
}

/// 检查 caller 是否一致，如果调用者是 self canister，则默认 owner 是正确的
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

/// 模拟池子的 LP 代币
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, CandidType)]
pub struct DummyCanisterId(CanisterId);
impl DummyCanisterId {
    /// 构建
    pub fn new(id: CanisterId) -> Self {
        Self(id)
    }

    /// 获取 id
    pub fn id(&self) -> CanisterId {
        self.0
    }
}
