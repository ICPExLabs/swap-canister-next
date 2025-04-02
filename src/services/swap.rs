// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};

use crate::types::{BusinessError, TokenDepositArgs, TokenTransferResut, TokenWithdrawArgs};

pub struct Service(pub Principal);
impl Service {
    pub async fn token_deposit(
        &self,
        args: TokenDepositArgs,
        retries: Option<u8>,
    ) -> Result<candid::Nat, BusinessError> {
        ic_cdk::call::<_, (TokenTransferResut,)>(self.0, "token_deposit", (args, retries))
            .await
            .map_err(BusinessError::CallCanisterError)?
            .0
            .into()
    }
    pub async fn token_withdraw(
        &self,
        args: TokenWithdrawArgs,
        retries: Option<u8>,
    ) -> Result<candid::Nat, BusinessError> {
        ic_cdk::call::<_, (TokenTransferResut,)>(self.0, "token_withdraw", (args, retries))
            .await
            .map_err(BusinessError::CallCanisterError)?
            .0
            .into()
    }
}
