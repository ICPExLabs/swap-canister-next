#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

pub mod token;

pub mod swap;

async fn deploy_canister(
    trace: &mut RequestTrace,
    initial_cycles: u128,
    wasm: Vec<u8>,
    arg: Vec<u8>,
) -> Result<CanisterId, BusinessError> {
    use ic_cdk::api::management_canister::main::{
        CanisterIdRecord, CanisterInstallMode, CreateCanisterArgument, InstallCodeArgument,
        create_canister, install_code, start_canister,
    };

    // 1. 创建一个新的罐子
    let canister_id =
        match create_canister(CreateCanisterArgument { settings: None }, initial_cycles)
            .await
            .map(|(CanisterIdRecord { canister_id },)| canister_id)
            .map_err(|err| {
                let err: BusinessError = err.into();
                err
            }) {
            Ok(canister_id) => canister_id,
            Err(err) => {
                trace.failed(format!("create token archive canister failed: {err}"));
                return Err(err);
            }
        };
    trace.trace(format!(
        "create token archive canister success: [{}]",
        canister_id.to_text()
    ));
    with_mut_state_without_record(|s| {
        s.business_config_maintain_archives_cycles_recharged(canister_id, initial_cycles)
    });

    // 2. 安装代码
    if let Err(err) = install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: wasm,
        arg,
    })
    .await
    .map_err(|err| {
        let err: BusinessError = err.into();
        err
    }) {
        trace.failed(format!("token archive canister install code failed: {err}"));
        return Err(err);
    }
    trace.trace(format!(
        "token archive canister install code success: [{}]",
        canister_id.to_text()
    ));

    // 3. 启动
    if let Err(err) = start_canister(CanisterIdRecord { canister_id })
        .await
        .map_err(|err| {
            let err: BusinessError = err.into();
            err
        })
    {
        trace.failed(format!("start token archive canister failed: {err}"));
        return Err(err);
    }

    trace.trace(format!(
        "start token archive canister success: [{}]",
        canister_id.to_text()
    ));

    ic_cdk::println!("new canister id: {:?}", canister_id.to_text());

    trace.success(canister_id.to_text());

    Ok(canister_id)
}
