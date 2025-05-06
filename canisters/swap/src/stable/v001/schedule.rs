use ic_canister_kit::{common::option::display_option_by, times::now};

use super::super::*;
#[allow(unused)]
use super::types::*;

#[allow(unused)]
#[allow(unused_variables)]
pub async fn schedule_task(record_by: Option<CallerId>) {
    // If there is a scheduled task
    ic_cdk::println!(
        "{}: do schedule task... ({})",
        display_option_by(&record_by, |p| p.to_text()),
        now()
    );

    // ! To ensure the integrity of the record, panic should not occur
    inner_task(record_by).await;
}

async fn inner_task(caller: Option<CallerId>) {
    ic_cdk::println!("do something: {:?}", caller.map(|c| c.to_text()));

    let now = TimestampNanos::now();
    if with_mut_state(|s| s.business_config_maintain_trigger(now)) {
        let mut trace = RequestTrace::from_args(RequestArgs::CanistersMaintaining);
        let result = maintaining_canisters(&mut trace).await;
        with_mut_state(|s| s.business_request_trace_insert(trace));
        if let Err(err) = result {
            ic_cdk::println!("maintaining_canisters err: {err:?}");
        }
    }
}

async fn maintaining_canisters(trace: &mut RequestTrace) -> Result<(), BusinessError> {
    // 0. Check maintenance status
    if let Err(err) = with_state(|s| s.pause_must_be_running()) {
        trace.failed(format!("system paused: {err:?}"));
        return Err(BusinessError::system_error(err));
    }

    // 1. Query all canisters that need to be managed
    let config = with_state(|s| s.business_config_maintain_archives_query().clone());
    let threshold = Nat::from(config.min_cycles_threshold);
    let canisters = with_state(|s| s.business_config_maintain_canisters());
    trace.trace(format!(
        "found canisters: [{}]",
        canisters
            .iter()
            .map(|c| format!("\"{}\"", c.to_text()))
            .collect::<Vec<_>>()
            .join(",")
    ));

    // 2. Traverse each canister to determine whether it is necessary to recharge
    for (i, &canister_id) in canisters.iter().enumerate() {
        let prefix = format!("# {} *[{}]* ", i + 1, canister_id.to_text());
        let status: ic_cdk::management_canister::CanisterStatusResult =
            match ic_cdk::call::Call::unbounded_wait(canister_id, "canister_status").await {
                Ok(status) => status.candid()?,
                Err(err) => {
                    let err: BusinessError = err.into();
                    let message = format!("{prefix}`query canister status failed: {err}`");
                    ic_cdk::println!("{}", message);
                    trace.trace(message);
                    continue;
                }
            };
        let cycles_balance = status.cycles;
        trace.trace(format!("{prefix}`cycles_balance: {cycles_balance}`"));
        if cycles_balance <= threshold {
            // do recharge
            let recharge = Nat::from(config.recharge_cycles);
            if let Err(err) = ic_cdk::management_canister::deposit_cycles(
                &ic_cdk::management_canister::DepositCyclesArgs { canister_id },
                config.recharge_cycles as u128,
            )
            .await
            {
                let err: BusinessError = err.into();
                let message = format!("{prefix}`recharge cycles: {recharge}, failed: {err}`",);
                ic_cdk::println!("{}", message);
                trace.trace(message);
                continue;
            }
            let message = format!("{prefix}`recharged cycles: {recharge}`",);
            ic_cdk::println!("{}", message);
            trace.trace(message);
        }
    }

    ic_cdk::println!("maintain canister done: {}", canisters.len());
    trace.success(canisters.len().to_string());

    Ok(())
}
