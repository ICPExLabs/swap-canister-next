#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== query ==============================

#[ic_cdk::update(guard = "has_pause_replace")]
fn config_token_block_chain() -> BlockChainView<TokenBlock> {
    with_state(|s| s.business_config_token_block_chain().into())
}

// ============================== replace ==============================

#[ic_cdk::update(guard = "has_pause_replace")]
fn config_token_archive_max_length_replace(max_length: u64) -> Option<CurrentArchiving> {
    with_mut_state_without_record(|s| {
        s.business_config_token_archive_max_length_replace(max_length)
    })
}

#[ic_cdk::update(guard = "has_pause_replace")]
fn config_token_archive_config_replace(
    archive_config: NextArchiveCanisterConfig,
) -> NextArchiveCanisterConfig {
    with_mut_state_without_record(|s| {
        s.business_config_token_archive_config_replace(archive_config)
    })
}

// ============================== set archived canisters ==============================

#[ic_cdk::update(guard = "has_pause_replace")]
async fn config_token_archived_canister_maintainers_set(
    canister_id: CanisterId,
    maintainers: Option<Vec<UserId>>,
) -> Result<(), BusinessError> {
    let service_archive = crate::services::archive::Service(canister_id);
    return service_archive.set_maintainers(maintainers).await;
}

#[ic_cdk::update(guard = "has_pause_replace")]
async fn config_token_archived_canister_max_memory_size_bytes_set(
    canister_id: CanisterId,
    max_memory_size_bytes: u64,
) -> Result<(), BusinessError> {
    let service_archive = crate::services::archive::Service(canister_id);
    return service_archive
        .set_max_memory_size_bytes(max_memory_size_bytes)
        .await;
}

// ============================== push token block ==============================

/// 推送罐子
#[ic_cdk::update(guard = "has_pause_replace")]
pub async fn config_token_blocks_push() -> Result<(), BusinessError> {
    // 0. 获取锁
    let _lock = with_mut_state_without_record(|s| s.business_token_block_chain_archive_lock())
        .ok_or(BusinessError::SystemError(
            "token block chain archive locked".into(),
        ))?;

    // 1. 若当前已满，需要进行存档
    with_mut_state_without_record(|s| s.business_config_token_archive_current_canister())?;

    // 2. 查询罐子是否已满
    let mut view: BlockChainView<TokenBlock> =
        with_state(|s| s.business_config_token_block_chain().into());
    if let Some(archiving) = view.current_archiving {
        if archiving.max_length <= archiving.length {
            return Err(BusinessError::SystemError(
                "token block chain current archive canister is full".into(),
            ));
        }
    }

    // 3. 若不存在或已满，需要创建新的罐子
    if view.current_archiving.is_none() {
        const INITIAL_CYCLES: u128 = 3_000_000_000_000;
        let cycles_balance = ic_canister_kit::canister::cycles::wallet_balance();
        let required = Nat::from(INITIAL_CYCLES * 2);
        // 判断自身 cycles 是否足够
        if cycles_balance < required {
            return Err(BusinessError::SystemError(format!(
                "self canister insufficient cycles: {cycles_balance} < {required}"
            )));
        }
        // 创建新的罐子
        let _wasm = view.archive_config.wasm.ok_or(BusinessError::SystemError(
            "token block chain wasm is none, can not deploy next archive canister".into(),
        ))?;
        // let init_args = ::common::archive::token::InitArgV1 {
        //     maintainers: todo!(),
        //     schedule: todo!(),
        //     max_memory_size_bytes: todo!(),
        //     core_canister_id: todo!(),
        //     block_offset: todo!(),
        // };
        // let init_args = candid::Encode!(Some(init_args),);
        // ic_canister_kit::canister::deploy::deploy_canister(None, INITIAL_CYCLES, wasm, init_args);
        // 再次获取
        view = with_state(|s| s.business_config_token_block_chain().into());
    }
    let _current_archiving = match view.current_archiving {
        Some(current_archiving) => current_archiving,
        None => {
            return Err(BusinessError::SystemError(
                "token block chain current archive canister is none".into(),
            ));
        }
    };

    // 2. 查询当前能够存储到当前罐子的块信息
    // 3. 查询目标罐子是否缓存这些块
    // 4. 移除相同的块
    // 5. 推送新的块并在成功后移除
    // todo!()
    Ok(())
}
