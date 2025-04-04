use candid::CandidType;
use ic_canister_kit::times::now;
use serde::{Deserialize, Serialize};

use crate::types::CheckArgs;

use super::BusinessError;

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct Deadline(u64);

impl CheckArgs for Deadline {
    type Result = ();

    fn check_args(&self) -> Result<Self::Result, super::BusinessError> {
        let now = now().into_inner();
        if self.0 as i128 <= now {
            return Err(BusinessError::Expired(now as u64));
        }
        Ok(())
    }
}
