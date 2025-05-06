// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Nat, Principal};
use common::types::{BusinessError, EncodedBlock};
use ic_canister_kit::types::UserId;

type CallResult<T> = Result<T, BusinessError>;

pub struct Service(pub Principal);
impl Service {
    pub async fn set_maintainers(&self, maintainers: Option<Vec<UserId>>) -> CallResult<()> {
        ic_cdk::call::Call::unbounded_wait(self.0, "set_maintainers")
            .with_arg(maintainers)
            .await?
            .candid::<()>()?;
        Ok(())
    }
    pub async fn set_max_memory_size_bytes(&self, max_memory_size_bytes: u64) -> CallResult<()> {
        ic_cdk::call::Call::unbounded_wait(self.0, "set_max_memory_size_bytes")
            .with_arg(max_memory_size_bytes)
            .await?
            .candid::<()>()?;
        Ok(())
    }
    pub async fn append_blocks(&self, args: Vec<EncodedBlock>) -> CallResult<()> {
        ic_cdk::call::Call::unbounded_wait(self.0, "append_blocks")
            .with_arg(args)
            .await?
            .candid::<()>()?;
        Ok(())
    }
}
