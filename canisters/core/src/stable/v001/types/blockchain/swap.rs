use ic_canister_kit::{
    common::trap,
    types::{StableBTreeMap, StableCell, UserId},
};
use serde::{Deserialize, Serialize};

use crate::types::with_mut_state;

use super::super::{
    Account, BlockIndex, Business, BusinessError, CandidBlock, CanisterId, CurrentArchiving,
    EncodedBlock, HashOf, NextArchiveCanisterConfig, QueryBlockResult, SwapBlock, SwapTransaction,
    TimestampNanos, init_swap_blocks, init_swap_wasm_module, system_error,
};

use super::BlockChain;

const WASM_MODULE: &[u8] = include_bytes!("../../../../../../archive-swap/sources/source_opt.wasm");

#[derive(Serialize, Deserialize)]
pub struct SwapBlockChain {
    #[serde(skip, default = "init_swap_blocks")]
    cached: StableBTreeMap<BlockIndex, EncodedBlock>, // 暂存所有缓存的块
    #[serde(skip, default = "init_swap_wasm_module")]
    wasm_module: StableCell<Option<Vec<u8>>>,
    block_chain: BlockChain<SwapBlock>,
}

impl Default for SwapBlockChain {
    fn default() -> Self {
        Self {
            cached: init_swap_blocks(),
            wasm_module: init_swap_wasm_module(),
            block_chain: BlockChain::default(),
        }
    }
}

impl SwapBlockChain {
    pub fn queryable(&self, caller: &UserId) -> bool {
        self.block_chain.queryable(caller)
    }
    pub fn query(&self, block_height: BlockIndex) -> QueryBlockResult<EncodedBlock> {
        if let Some(canister_id) = self.block_chain.query(block_height) {
            return QueryBlockResult::Archive(canister_id);
        }
        let block = self.cached.get(&block_height).map(QueryBlockResult::Block);
        trap(block.ok_or("invalid block height"))
    }

    pub fn set_archive_maintainers(&mut self, maintainers: Option<Vec<UserId>>) {
        self.block_chain.set_archive_maintainers(maintainers);
    }

    // swap
    pub fn init_wasm_module(&mut self) -> Result<(), BusinessError> {
        if self.wasm_module.get().is_none() {
            self.wasm_module
                .set(Some(WASM_MODULE.to_vec()))
                .map_err(|err| system_error(format!("init wasm module failed: {err:?}")))?;
        }
        Ok(())
    }
    pub fn get_swap_block_chain(&self) -> &BlockChain<SwapBlock> {
        &self.block_chain
    }
    pub fn query_wasm_module(&self) -> &Option<Vec<u8>> {
        self.wasm_module.get()
    }
    pub fn replace_wasm_module(
        &mut self,
        wasm_module: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, BusinessError> {
        let old = self.wasm_module.get().clone();
        self.wasm_module
            .set(Some(wasm_module))
            .map_err(|err| system_error(format!("replace wasm module failed: {err:?}")))?;
        Ok(old)
    }
    pub fn set_swap_current_archiving_max_length(
        &mut self,
        max_length: u64,
    ) -> Option<CurrentArchiving> {
        self.block_chain
            .set_current_archiving_max_length(max_length)
    }
    pub fn set_swap_archive_config(
        &mut self,
        archive_config: NextArchiveCanisterConfig,
    ) -> NextArchiveCanisterConfig {
        self.block_chain.set_archive_config(archive_config)
    }
    pub fn replace_swap_current_archiving(
        &mut self,
        archiving: CurrentArchiving,
    ) -> Option<CurrentArchiving> {
        self.block_chain.replace_current_archiving(archiving)
    }
    pub fn archive_current_canister(&mut self) -> Result<(), BusinessError> {
        self.block_chain.archive_current_canister()
    }
    pub fn get_parent_hash(&self, block_height: BlockIndex) -> Option<HashOf<SwapBlock>> {
        if let Some(block) = self.cached.get(&block_height) {
            let block: SwapBlock = trap(block.try_into());
            return Some(block.get_parent_hash());
        }
        Some(self.block_chain.latest_block_hash)
    }
    pub fn get_cached_block_index(&self) -> Option<(BlockIndex, u64)> {
        let keys = self.cached.keys().collect::<Vec<_>>();
        let length = keys.len();
        keys.into_iter().min().map(|height| (height, length as u64))
    }
    pub fn archived_block(&mut self, block_height: BlockIndex) -> Result<(), BusinessError> {
        use ::common::types::system_error;
        if !self.cached.contains_key(&block_height) {
            return Err(system_error(format!(
                "block: {block_height} is not exist. can not remove"
            )));
        }
        if !self.block_chain.increment(block_height) {
            return Err(system_error(format!(
                "last block height is not: {block_height}"
            )));
        }
        self.cached.remove(&block_height);
        Ok(())
    }

    pub fn get_maintain_canisters(&self) -> Vec<CanisterId> {
        self.block_chain.get_maintain_canisters()
    }

    // locks
    pub fn archive_lock(&mut self) -> Option<SwapBlockChainArchiveLock> {
        let mut locked = trap(self.block_chain.archive_locked.write()); // ! what if failed ?

        if *locked {
            return None;
        }

        *locked = true;
        ic_cdk::println!("🔒 Archive Locked swap block chain.");

        Some(SwapBlockChainArchiveLock)
    }

    pub fn archive_unlock(&mut self) {
        let mut locked = trap(self.block_chain.archive_locked.write()); // ! what if failed ?

        // 1. check first
        if !*locked {
            // if not true, terminator
            let tips = "Archive Unlock swap block chain failed. That is not locked.";
            ic_cdk::trap(tips); // never be here
        }

        // 2. do unlock
        *locked = false;
        ic_cdk::println!("🔐 Archive Unlock swap block chain.");
    }

    // locks
    pub fn lock(&mut self, fee_to: Option<Account>) -> Option<SwapBlockChainLock> {
        let mut locked = trap(self.block_chain.locked.write()); // ! what if failed ?

        if *locked {
            return None;
        }

        *locked = true;
        ic_cdk::println!("🔒 Locked swap block chain.");

        Some(SwapBlockChainLock { fee_to })
    }

    pub fn unlock(&mut self) {
        let mut locked = trap(self.block_chain.locked.write()); // ! what if failed ?

        // 1. check first
        if !*locked {
            // if not true, terminator
            let tips = "Unlock swap block chain failed. That is not locked.";
            ic_cdk::trap(tips); // never be here
        }

        // 2. do unlock
        *locked = false;
        ic_cdk::println!("🔐 Unlock swap block chain.");
    }

    pub fn be_guard<'a>(&'a mut self, lock: &'a SwapBlockChainLock) -> SwapBlockChainGuard<'a> {
        SwapBlockChainGuard {
            swap_block_chain: self,
            _lock: lock,
        }
    }

    pub fn get_latest_hash(&self) -> &[u8] {
        self.block_chain.latest_block_hash.as_slice()
    }
}

// ============================ lock ============================

pub struct SwapBlockChainArchiveLock;

impl Drop for SwapBlockChainArchiveLock {
    fn drop(&mut self) {
        with_mut_state(|s| s.business_swap_block_chain_archive_unlock())
    }
}

pub struct SwapBlockChainLock {
    pub fee_to: Option<Account>,
}

impl Drop for SwapBlockChainLock {
    fn drop(&mut self) {
        with_mut_state(|s| s.business_swap_block_chain_unlock())
    }
}

// ============================ guard ============================

pub struct SwapBlockChainGuard<'a> {
    swap_block_chain: &'a mut SwapBlockChain,
    _lock: &'a SwapBlockChainLock,
}

impl SwapBlockChainGuard<'_> {
    fn get_next_swap_block(
        &self,
        now: TimestampNanos,
        transaction: SwapTransaction,
    ) -> Result<(EncodedBlock, HashOf<SwapBlock>), BusinessError> {
        use ::common::utils::pb::to_proto_bytes;
        use ::common::{archive::swap::SwapBlock, proto, types::DoHash};

        if self
            .swap_block_chain
            .cached
            .contains_key(&self.swap_block_chain.block_chain.next_block_index)
        {
            return Err(BusinessError::SwapBlockChainError(
                "The next block index is already in the cache.".to_string(),
            ));
        }

        let parent_hash = self.swap_block_chain.block_chain.latest_block_hash;
        let block = SwapBlock(CandidBlock {
            parent_hash,
            timestamp: now,
            transaction,
        });
        let hash = block
            .do_hash()
            .map_err(BusinessError::SwapBlockChainError)?;
        let block: proto::SwapBlock = block
            .try_into()
            .map_err(|err| BusinessError::SwapBlockChainError(format!("{err:?}")))?;
        let encoded_block = to_proto_bytes(&block).map_err(BusinessError::SwapBlockChainError)?;
        let encoded_block = EncodedBlock(encoded_block);
        Ok((encoded_block, hash))
    }

    fn push_block(&mut self, encoded_block: EncodedBlock, block_hash: HashOf<SwapBlock>) {
        let block_height = self.swap_block_chain.block_chain.next_block_index;
        self.swap_block_chain
            .cached
            .insert(block_height, encoded_block);
        self.swap_block_chain.block_chain.next_block(block_hash);
    }

    pub fn mint_block<T, F>(
        &mut self,
        now: TimestampNanos,
        transaction: SwapTransaction,
        handle: F,
    ) -> Result<T, BusinessError>
    where
        F: FnOnce(&mut Self) -> Result<T, BusinessError>,
    {
        let (encoded_block, hash) = self.get_next_swap_block(now, transaction)?;
        let data = handle(self)?;
        self.push_block(encoded_block, hash);
        Ok(data)
    }
}
