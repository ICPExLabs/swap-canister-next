use candid::CandidType;
use common::types::BusinessError;
use ic_canister_kit::times::now;
use serde::{Deserialize, Serialize};

use crate::types::CheckArgs;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, CandidType)]
pub struct Deadline(u64);

impl CheckArgs for Deadline {
    type Result = ();

    fn check_args(&self) -> Result<Self::Result, BusinessError> {
        let now = now().into_inner();
        if self.0 as i128 <= now {
            return Err(BusinessError::Expired {
                system: now as u64,
                deadline: self.0,
            });
        }
        Ok(())
    }
}
