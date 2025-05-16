#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== query ==============================

#[ic_cdk::query]
fn config_token_block_chain_query(args: BlockChainArgs) -> TokenBlockResult {
    inner_config_token_block_chain_query(args).into()
}
fn inner_config_token_block_chain_query(args: BlockChainArgs) -> Result<TokenBlockResponse, BusinessError> {
    let response = match args {
        BlockChainArgs::BlockChainQuery => {
            BlockChainResponse::BlockChain(with_state(|s| s.business_config_token_block_chain_query().into()))
        }
        BlockChainArgs::WasmModuleQuery => BlockChainResponse::WasmModule(with_state(|s| {
            s.business_config_token_archive_wasm_module_query().clone()
        })),
        BlockChainArgs::CachedBlockQuery => {
            BlockChainResponse::CachedBlock(with_state(|s| s.business_config_token_cached_block_get()))
        }
        BlockChainArgs::BlockQuery(_) => unimplemented!(),
        _ => ic_cdk::trap("not query args"),
    };
    Ok(response.into())
}

// ============================== update ==============================

#[ic_cdk::update(guard = "has_business_config_maintaining")]
async fn config_token_block_chain_update(args: BlockChainArgs) -> TokenBlockResult {
    inner_config_token_block_chain_update(args).await.into()
}
async fn inner_config_token_block_chain_update(args: BlockChainArgs) -> Result<TokenBlockResponse, BusinessError> {
    let response = match args {
        BlockChainArgs::WasmModuleUpdate(wasm_module) => BlockChainResponse::WasmModule(with_mut_state(|s| {
            s.business_config_token_archive_wasm_module_replace(wasm_module)
        })?),
        BlockChainArgs::CurrentArchivingMaxLengthUpdate(max_length) => {
            BlockChainResponse::CurrentArchivingMaxLength(with_mut_state(|s| {
                s.business_config_token_current_archiving_max_length_replace(max_length)
            }))
        }
        BlockChainArgs::NextArchiveCanisterConfigUpdate(archive_config) => {
            BlockChainResponse::NextArchiveCanisterConfig(with_mut_state(|s| {
                s.business_config_token_archive_config_replace(archive_config)
            }))
        }
        BlockChainArgs::ArchivedCanisterMaintainersUpdate {
            canister_id,
            maintainers,
        } => {
            let service_archive = crate::services::archive::Service(canister_id);
            service_archive.set_maintainers(maintainers).await?;
            BlockChainResponse::ArchivedCanisterMaintainers
        }
        BlockChainArgs::ArchivedCanisterMaxMemorySizeBytesUpdate {
            canister_id,
            max_memory_size_bytes,
        } => {
            let service_archive = crate::services::archive::Service(canister_id);
            service_archive.set_max_memory_size_bytes(max_memory_size_bytes).await?;
            BlockChainResponse::ArchivedCanisterMaxMemorySizeBytes
        }
        BlockChainArgs::BlocksPush => BlockChainResponse::BlocksPush(inner_config_token_blocks_push().await?),
        _ => ic_cdk::trap("not update args"),
    };
    Ok(response.into())
}

// ============================== push token block ==============================

/// Push blocks
pub async fn inner_config_token_blocks_push() -> Result<Option<PushBlocks>, BusinessError> {
    use super::deploy_canister;

    // 0. Must be non-pause state, obtain lock
    with_state(|s| s.pause_must_be_running()).map_err(BusinessError::system_error)?;
    let _lock = with_mut_state(|s| s.business_token_block_chain_archive_lock())
        .ok_or(BusinessError::system_error("token block chain archive locked"))?;

    // 1. If it is currently full, archive is required
    with_mut_state(|s| s.business_config_token_archive_current_canister())?;

    // 2. Check if the canister is full
    let mut view: BlockChainView<TokenBlock> = with_state(|s| s.business_config_token_block_chain_query().into());
    if let Some(current_archiving) = view.current_archiving {
        if current_archiving.is_full() {
            return Err(BusinessError::system_error(
                "token block chain current archive canister is full",
            ));
        }
    }

    // 3. If it does not exist or is full, a new canister needs to be created
    if view.current_archiving.is_none() {
        const INITIAL_CYCLES: u128 = 3_000_000_000_000; // initial 3 TCycles
        let cycles_balance = ic_canister_kit::canister::cycles::wallet_balance();
        let required = Nat::from(INITIAL_CYCLES * 2);
        // whether self cycles are sufficient
        if cycles_balance < required {
            return Err(BusinessError::system_error(format!(
                "self canister insufficient cycles: {cycles_balance} < {required}"
            )));
        }
        // Create a new canister
        let wasm = with_state(|s| s.business_config_token_archive_wasm_module_query().clone()).ok_or(
            BusinessError::system_error("token block chain wasm is none, can not deploy next archive canister"),
        )?;
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
                let hash = with_state(|s| s.business_config_token_parent_hash_get(block_offset)).ok_or(
                    BusinessError::system_error(format!("can not find parent hash by height: {block_offset}")),
                )?;
                Some((block_offset, hash))
            }
        };
        let init_args = ::common::archive::token::InitArgV1 {
            maintainers: view.archive_config.maintainers,
            max_memory_size_bytes: view.archive_config.max_memory_size_bytes,
            host_canister_id: Some(self_canister_id()),
            block_offset,
        };
        let init_args = candid::encode_args((Some(init_args.clone()),))
            .map_err(|err| BusinessError::system_error(format!("can not encode args: {init_args:?} {err:?}")))?;
        let mut trace = RequestTrace::from_args(RequestArgs::TokenBlockPush);
        let deploy_result = deploy_canister(&mut trace, INITIAL_CYCLES, wasm, init_args).await;
        with_mut_state(|s| s.business_request_trace_insert(trace));
        let canister_id = deploy_result.map_err(|err| {
            BusinessError::system_error(format!("create and deploy new token canister failed: {err:?}"))
        })?;
        // Set up a new archive canister
        with_mut_state(|s| {
            s.business_config_token_current_archiving_replace(CurrentArchiving {
                canister_id,
                block_height_offset: block_offset.map(|(h, _)| h).unwrap_or_default(),
                length: 0,
                max_length: view.archive_config.max_length,
            })
        });
        // Get it again
        view = with_state(|s| s.business_config_token_block_chain_query().into());
    }
    let current_archiving = match view.current_archiving {
        Some(current_archiving) => {
            if current_archiving.is_full() {
                return Err(BusinessError::system_error(
                    "token block chain current archive canister is full",
                ));
            }
            current_archiving
        }
        None => {
            return Err(BusinessError::system_error(
                "token block chain current archive canister is none",
            ));
        }
    };

    // 4. Query the block information that can be stored in the current canister
    let (height_start, length) = match with_state(|s| s.business_config_token_cached_block_get()) {
        Some(v) => v,
        None => return Ok(None), // nothing
    };
    let num = current_archiving.remain().min(length).min(MAX_BLOCKS_PER_REQUEST);
    if num == 0 {
        return Ok(None);
    }
    // 5. Push new block and remove after success
    let service = crate::services::archive::Service(current_archiving.canister_id);
    // push_block(num, height_start, service).await?;
    push_blocks(num, height_start, service).await?;
    Ok(Some(PushBlocks {
        block_height_start: height_start,
        length: num,
    }))
}

#[allow(unused)]
async fn push_block(
    num: u64,
    height_start: BlockIndex,
    service: crate::services::archive::Service,
) -> Result<(), BusinessError> {
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
        let s = ic_cdk::api::time();
        service.append_blocks(vec![block]).await?;
        let e = ic_cdk::api::time();
        ic_cdk::println!("🕓 append token block({block_height}) spend: {} ns", e - s);
        // After successful insertion, remove the cache and update the sequence number
        with_mut_state(|s| s.business_config_token_block_archived(block_height))?;
    }
    Ok(())
}

#[allow(unused)]
async fn push_blocks(
    num: u64,
    height_start: BlockIndex,
    service: crate::services::archive::Service,
) -> Result<(), BusinessError> {
    let mut block_height_list = Vec::with_capacity(num as usize);
    let mut blocks = Vec::with_capacity(num as usize);
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
        block_height_list.push(block_height);
        blocks.push(block);
    }
    let s = ic_cdk::api::time();
    service.append_blocks(blocks).await?;
    let e = ic_cdk::api::time();
    ic_cdk::println!("🕓 append token blocks[{height_start}, #{num}) spend: {} ns", e - s);
    for block_height in block_height_list {
        // After successful insertion, remove the cache and update the sequence number
        with_mut_state(|s| s.business_config_token_block_archived(block_height))?;
    }
    Ok(())
}
