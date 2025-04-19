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
async fn config_token_blocks_push() -> PushBlocksResult {
    inner_config_token_blocks_push().await.into()
}

/// 推送罐子
pub async fn inner_config_token_blocks_push() -> Result<Option<PushBlocks>, BusinessError> {
    use super::deploy_canister;
    use ::common::types::system_error;

    // 0. 获取锁
    let _lock = with_mut_state_without_record(|s| s.business_token_block_chain_archive_lock())
        .ok_or(system_error("token block chain archive locked"))?;

    // 1. 若当前已满，需要进行存档
    with_mut_state_without_record(|s| s.business_config_token_archive_current_canister())?;

    // 2. 查询罐子是否已满
    let mut view: BlockChainView<TokenBlock> =
        with_state(|s| s.business_config_token_block_chain().into());
    if let Some(current_archiving) = view.current_archiving {
        if current_archiving.is_full() {
            return Err(system_error(
                "token block chain current archive canister is full",
            ));
        }
    }

    // 3. 若不存在或已满，需要创建新的罐子
    if view.current_archiving.is_none() {
        const INITIAL_CYCLES: u128 = 3_000_000_000_000; // initial 3 TCycles
        let cycles_balance = ic_canister_kit::canister::cycles::wallet_balance();
        let required = Nat::from(INITIAL_CYCLES * 2);
        // 判断自身 cycles 是否足够
        if cycles_balance < required {
            return Err(system_error(format!(
                "self canister insufficient cycles: {cycles_balance} < {required}"
            )));
        }
        // 创建新的罐子
        let wasm = view.archive_config.wasm.ok_or(system_error(
            "token block chain wasm is none, can not deploy next archive canister",
        ))?;
        let block_offset = {
            let mut block_offset = 0;
            for a in &view.archived {
                let last = a.block_height_offset + a.length;
                if block_offset < last {
                    block_offset = last;
                }
            }
            if block_offset == 0 {
                None
            } else {
                let hash = with_state(|s| s.business_config_token_parent_hash_get(block_offset))
                    .ok_or(system_error(format!(
                        "can not find parent hash by height: {block_offset}"
                    )))?;
                Some((block_offset, hash))
            }
        };
        let init_args = ::common::archive::token::InitArgV1 {
            maintainers: view.archive_config.maintainers,
            schedule: None,
            max_memory_size_bytes: view.archive_config.max_memory_size_bytes,
            core_canister_id: Some(self_canister_id()),
            block_offset,
        };
        let init_args = candid::encode_args((Some(init_args.clone()),))
            .map_err(|err| system_error(format!("can not encode args: {init_args:?} {err:?}")))?;
        let mut trace = RequestTrace::from_args(RequestArgs::TokenBlockPush);
        let deploy_result = deploy_canister(&mut trace, INITIAL_CYCLES, wasm, init_args).await;
        with_mut_state_without_record(|s| s.business_request_trace_insert(trace));
        let canister_id = deploy_result.map_err(|err| {
            system_error(format!(
                "create and deploy new token canister failed: {err:?}"
            ))
        })?;
        // 设置新的存档罐子
        with_mut_state_without_record(|s| {
            s.business_config_token_current_archiving_replace(CurrentArchiving {
                canister_id,
                block_height_offset: block_offset.map(|(h, _)| h).unwrap_or_default(),
                length: 0,
                max_length: view.archive_config.max_length,
            })
        });
        // 再次获取
        view = with_state(|s| s.business_config_token_block_chain().into());
    }
    let current_archiving = match view.current_archiving {
        Some(current_archiving) => {
            if current_archiving.is_full() {
                return Err(system_error(
                    "token block chain current archive canister is full",
                ));
            }
            current_archiving
        }
        None => {
            return Err(system_error(
                "token block chain current archive canister is none",
            ));
        }
    };

    // 4. 查询当前能够存储到当前罐子的块信息
    let (height_start, length) = match with_state(|s| s.business_config_token_cached_block_get()) {
        Some(v) => v,
        None => return Ok(None), // nothing
    };
    let num = current_archiving.remain().min(length);
    if num == 0 {
        return Ok(None);
    }
    // 7. 推送新的块并在成功后移除
    let service = crate::services::archive::Service(current_archiving.canister_id);
    for i in 0..num {
        let block_height = height_start + i;
        let block = match with_state(|s| s.business_token_block_get(block_height)) {
            QueryBlockResult::Block(block) => block,
            QueryBlockResult::Archive(principal) => {
                return Err(BusinessError::SystemError(format!(
                    "token block: {block_height} already archived in [{}]",
                    principal.to_text()
                )));
            }
        };
        service.append_blocks(vec![block]).await?;
        // 成功插入后要移除缓存并更新序号
        with_mut_state_without_record(|s| s.business_config_token_block_archived(block_height))?;
    }
    Ok(Some(PushBlocks {
        block_height_start: height_start,
        length: num,
    }))
}
