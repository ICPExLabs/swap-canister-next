use ic_canister_kit::{common::option::display_option_by, times::now};

use super::super::*;
#[allow(unused)]
use super::types::*;

#[allow(unused)]
#[allow(unused_variables)]
pub async fn schedule_task(record_by: Option<CallerId>) {
    // 如果有定时任务
    ic_cdk::println!(
        "{}: do schedule task... ({})",
        display_option_by(&record_by, |p| p.to_text()),
        now()
    );

    // ! 为了保证记录的完整性，不应当发生 panic
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
    use ::common::types::system_error;

    // 0.检查维护状态
    if let Err(err) = with_state(|s| s.pause_must_be_running()) {
        trace.failed(format!("system paused: {err:?}"));
        return Err(system_error(err));
    }

    // 1. 查询所有需要管理的罐子
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

    // 2. 遍历每一个罐子判断是否需要充值
    for (i, &canister_id) in canisters.iter().enumerate() {
        let prefix = format!("# {} *[{}]* ", i + 1, canister_id.to_text());
        let status = match ic_cdk::api::call::call::<
            (),
            (ic_cdk::api::management_canister::main::CanisterStatusResponse,),
        >(canister_id, "canister_status", ())
        .await
        .map(|(s,)| s)
        {
            Ok(status) => status,
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
            if let Err(err) = ic_cdk::api::management_canister::main::deposit_cycles(
                ic_cdk::api::management_canister::provisional::CanisterIdRecord { canister_id },
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
