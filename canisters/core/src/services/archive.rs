// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Nat, Principal};
use common::types::{BusinessError, EncodedBlock};
use ic_canister_kit::types::UserId;

pub struct Service(pub Principal);
impl Service {
    pub async fn set_maintainers(
        &self,
        maintainers: Option<Vec<UserId>>,
    ) -> Result<(), BusinessError> {
        ic_cdk::call::<_, ()>(self.0, "set_maintainers", (maintainers,)).await?;
        Ok(())
    }
    pub async fn set_max_memory_size_bytes(
        &self,
        max_memory_size_bytes: u64,
    ) -> Result<(), BusinessError> {
        ic_cdk::call::<_, ()>(
            self.0,
            "set_max_memory_size_bytes",
            (max_memory_size_bytes,),
        )
        .await?;
        Ok(())
    }
    pub async fn append_blocks(&self, args: Vec<EncodedBlock>) -> Result<(), BusinessError> {
        ic_cdk::call::<_, ()>(self.0, "append_blocks", (args,)).await?;
        Ok(())
    }
}
