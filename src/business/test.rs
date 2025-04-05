#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

use crate::services::icrc2::Service;
use crate::services::icrc2::TransferArg;
#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

#[allow(clippy::unwrap_used)]
#[ic_cdk::update(guard = "has_pause_replace")]
async fn test_withdraw_all_tokens(tokens: Vec<CanisterId>) {
    let caller = caller();
    let self_canister_id = self_canister_id();

    for token in tokens {
        let service = Service(token);
        let balance = service
            .icrc_1_balance_of(Account {
                owner: self_canister_id,
                subaccount: None,
            })
            .await
            .unwrap()
            .0;
        if balance == 0_u32 {
            continue;
        }

        let fee = service.icrc_1_fee().await.unwrap().0;
        service
            .icrc_1_transfer(TransferArg {
                to: Account {
                    owner: caller,
                    subaccount: None,
                },
                fee: None,
                memo: None,
                from_subaccount: None,
                created_at_time: None,
                amount: balance - fee,
            })
            .await
            .unwrap()
            .0
            .unwrap();
    }
}
