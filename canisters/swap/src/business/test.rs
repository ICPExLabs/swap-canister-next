#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== withdraw tokens ==============================

#[allow(clippy::unwrap_used)]
#[ic_cdk::update(guard = "has_business_config_maintaining")]
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
            .unwrap();
        if balance == 0_u32 {
            continue;
        }

        let fee = service.icrc_1_fee().await.unwrap();
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
            .unwrap();

        results.push(format!("fetch: {} {}", token.to_text(), amount));
    }

    results
}

#[allow(clippy::unwrap_used)]
#[ic_cdk::update(guard = "has_business_config_maintaining")]
async fn test_set_controller(canister_id: CanisterId) {
    let caller = caller();
    ic_canister_kit::canister::settings::update_settings(
        canister_id,
        ic_cdk::management_canister::CanisterSettings {
            controllers: Some(vec![caller]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            log_visibility: None,
            wasm_memory_limit: None,
            wasm_memory_threshold: None,
        },
    )
    .await
    .unwrap();
}

// ============================== current archiving ==============================

#[ic_cdk::update(guard = "has_business_config_maintaining")]
fn test_config_token_current_archiving_replace(archiving: CurrentArchiving) -> Option<CurrentArchiving> {
    with_mut_state(|s| s.business_config_token_current_archiving_replace(archiving))
}

#[ic_cdk::update(guard = "has_business_config_maintaining")]
fn test_config_swap_current_archiving_replace(archiving: CurrentArchiving) -> Option<CurrentArchiving> {
    with_mut_state(|s| s.business_config_swap_current_archiving_replace(archiving))
}
