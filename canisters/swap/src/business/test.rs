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

// ============================== test many tokens ==============================

#[ic_cdk::update(guard = "has_business_config_maintaining")]
fn test_push_test_tokens() {
    let tokens = with_state(|s| {
        s.business_tokens_query()
            .into_iter()
            .map(|(token, info)| (token, info.into_owned()))
            .collect::<Vec<_>>()
    });

    let tokens = tokens
        .iter()
        .filter_map(|(token, info)| {
            if info.is_lp_token {
                return None;
            }
            let canister_id = ::common::utils::hash::hash_sha256(token.as_slice());
            let canister_id = Principal::from_slice(&canister_id[..29]);
            if tokens.iter().any(|(t, _)| t == &canister_id) {
                return None;
            }
            let info = TokenInfo {
                canister_id,
                name: format!("{} (Test)", info.name),
                symbol: format!("{}TEST", info.symbol),
                decimals: info.decimals,
                fee: info.fee.clone(),
                is_lp_token: false,
            };
            Some(info)
        })
        .collect::<Vec<_>>();
    for token in tokens {
        let arg = ArgWithMeta::data(token);
        with_mut_state(|s| s.business_config_token_custom_put(arg));
    }
}

