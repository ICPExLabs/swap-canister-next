#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== withdraw tokens ==============================

#[allow(clippy::unwrap_used)]
#[ic_cdk::update(guard = "has_pause_replace")]
async fn test_withdraw_all_tokens(tokens: Vec<CanisterId>) -> Vec<String> {
    let caller = caller();
    let self_canister_id = self_canister_id();

    let mut results = vec![];
    for token in tokens {
        let service = crate::services::icrc2::Service(token);
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
        let amount = balance - fee;
        service
            .icrc_1_transfer(crate::services::icrc2::TransferArg {
                to: Account {
                    owner: caller,
                    subaccount: None,
                },
                fee: None,
                memo: None,
                from_subaccount: None,
                created_at_time: None,
                amount: amount.clone(),
            })
            .await
            .unwrap()
            .0
            .unwrap();

        results.push(format!("fetch: {} {}", token.to_text(), amount));
    }

    results
}

// ============================== current archiving ==============================

#[ic_cdk::update(guard = "has_pause_replace")]
fn test_config_token_current_archiving_replace(
    archiving: CurrentArchiving,
) -> Option<CurrentArchiving> {
    with_mut_state(|s| s.business_config_token_current_archiving_replace(archiving))
}
