use ic_canister_kit::{
    common::trap,
    types::{StableBTreeMap, StableCell, UserId},
};
use serde::{Deserialize, Serialize};

use crate::types::with_mut_state;

use super::super::{
    Account, BlockIndex, Business, BusinessError, CandidBlock, CanisterId, CurrentArchiving, EncodedBlock, HashOf,
    NextArchiveCanisterConfig, QueryBlockResult, TimestampNanos, TokenBlock, TokenTransaction, init_token_blocks,
    init_token_wasm_module,
};

use super::BlockChain;

const WASM_MODULE: &[u8] = include_bytes!("../../../../../../archive-token/sources/source_opt.wasm.gz");

#[derive(Serialize, Deserialize)]
pub struct TokenBlockChain {
    #[serde(skip, default = "init_token_blocks")]
    cached: StableBTreeMap<BlockIndex, EncodedBlock>, // Staging all cached blocks
    #[serde(skip, default = "init_token_wasm_module")]
    wasm_module: StableCell<Option<Vec<u8>>>,
    block_chain: BlockChain<TokenBlock>,
}

impl Default for TokenBlockChain {
    fn default() -> Self {
        Self {
            cached: init_token_blocks(),
            wasm_module: init_token_wasm_module(),
            block_chain: BlockChain::default(),
        }
    }
}

impl TokenBlockChain {
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
    pub fn query_blocks(&self, block_height: BlockIndex) -> Vec<(BlockIndex, QueryBlockResult<EncodedBlock>)> {
        self.block_chain
            .query_blocks(block_height, |block_height| self.cached.get(block_height))
    }

    pub fn set_archive_maintainers(&mut self, maintainers: Option<Vec<UserId>>) {
        self.block_chain.set_archive_maintainers(maintainers);
    }

    // token
    pub fn init_wasm_module(&mut self) -> Result<(), BusinessError> {
        if self.wasm_module.get().is_none() {
            self.wasm_module
                .set(Some(WASM_MODULE.to_vec()))
                .map_err(|err| BusinessError::system_error(format!("init wasm module failed: {err:?}")))?;
        }
        Ok(())
    }
    pub fn get_token_block_chain(&self) -> &BlockChain<TokenBlock> {
        &self.block_chain
    }
    pub fn query_wasm_module(&self) -> &Option<Vec<u8>> {
        self.wasm_module.get()
    }
    pub fn replace_wasm_module(&mut self, wasm_module: Vec<u8>) -> Result<Option<Vec<u8>>, BusinessError> {
        let old = self.wasm_module.get().clone();
        self.wasm_module
            .set(Some(wasm_module))
            .map_err(|err| BusinessError::system_error(format!("replace wasm module failed: {err:?}")))?;
        Ok(old)
    }
    pub fn set_token_current_archiving_max_length(&mut self, max_length: u64) -> Option<CurrentArchiving> {
        self.block_chain.set_current_archiving_max_length(max_length)
    }
    pub fn set_token_archive_config(&mut self, archive_config: NextArchiveCanisterConfig) -> NextArchiveCanisterConfig {
        self.block_chain.set_archive_config(archive_config)
    }
    pub fn replace_token_current_archiving(&mut self, archiving: CurrentArchiving) -> Option<CurrentArchiving> {
        self.block_chain.replace_current_archiving(archiving)
    }
    pub fn archive_current_canister(&mut self) -> Result<(), BusinessError> {
        self.block_chain.archive_current_canister()
    }
    pub fn get_parent_hash(&self, block_height: BlockIndex) -> Option<HashOf<TokenBlock>> {
        if let Some(block) = self.cached.get(&block_height) {
            let block: TokenBlock = trap(block.try_into());
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
        if !self.cached.contains_key(&block_height) {
            return Err(BusinessError::system_error(format!(
                "block: {block_height} is not exist. can not remove"
            )));
        }
        if !self.block_chain.increment(block_height) {
            return Err(BusinessError::system_error(format!(
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
    pub fn archive_lock(&mut self) -> Option<TokenBlockChainArchiveLock> {
        let mut locked = trap(self.block_chain.archive_locked.write()); // ! what if failed ?

        if *locked {
            return None;
        }

        *locked = true;
        ic_cdk::println!("🔒 Archive Locked token block chain.");

        Some(TokenBlockChainArchiveLock)
    }

    pub fn archive_unlock(&mut self) {
        let mut locked = trap(self.block_chain.archive_locked.write()); // ! what if failed ?

        // 1. check first
        if !*locked {
            // if not true, terminator
            let tips = "Archive Unlock token block chain failed. That is not locked.";
            ic_cdk::trap(tips); // never be here
        }

        // 2. do unlock
        *locked = false;
        ic_cdk::println!("🔐 Archive Unlock token block chain.");
    }

    // locks
    pub fn lock(&mut self, fee_to: Option<Account>) -> Option<TokenBlockChainLock> {
        let mut locked = trap(self.block_chain.locked.write()); // ! what if failed ?

        if *locked {
            return None;
        }

        *locked = true;
        ic_cdk::println!("🔒 Locked token block chain.");

        Some(TokenBlockChainLock { fee_to })
    }

    pub fn unlock(&mut self) {
        let mut locked = trap(self.block_chain.locked.write()); // ! what if failed ?

        // 1. check first
        if !*locked {
            // if not true, terminator
            let tips = "Unlock token block chain failed. That is not locked.";
            ic_cdk::trap(tips); // never be here
        }

        // 2. do unlock
        *locked = false;
        ic_cdk::println!("🔐 Unlock token block chain.");
    }

    pub fn be_guard<'a>(&'a mut self, lock: &'a TokenBlockChainLock) -> TokenBlockChainGuard<'a> {
        TokenBlockChainGuard::new(self, lock)
    }

    pub fn get_latest_hash(&self) -> &[u8] {
        self.block_chain.latest_block_hash.as_slice()
    }
}

// ============================ lock ============================

pub struct TokenBlockChainArchiveLock;

impl Drop for TokenBlockChainArchiveLock {
    fn drop(&mut self) {
        with_mut_state(|s| s.business_token_block_chain_archive_unlock())
    }
}

pub struct TokenBlockChainLock {
    pub fee_to: Option<Account>,
}

impl Drop for TokenBlockChainLock {
    fn drop(&mut self) {
        with_mut_state(|s| s.business_token_block_chain_unlock())
    }
}

// ============================ guard ============================

pub use guard::TokenBlockChainGuard;
mod guard {
    use super::*;
    pub struct TokenBlockChainGuard<'a> {
        stable_token_block_chain: &'a mut TokenBlockChain,
        lock: &'a TokenBlockChainLock,
        // stack data
        blocks: Vec<(BlockIndex, EncodedBlock, HashOf<TokenBlock>)>,
    }
    impl Drop for TokenBlockChainGuard<'_> {
        fn drop(&mut self) {
            // must drop by manual
        }
    }

    impl<'a> TokenBlockChainGuard<'a> {
        pub(super) fn new(stable_token_block_chain: &'a mut TokenBlockChain, lock: &'a TokenBlockChainLock) -> Self {
            Self {
                stable_token_block_chain,
                lock,
                blocks: Default::default(),
            }
        }

        pub(super) fn get_next_block_index(&self) -> BlockIndex {
            self.blocks
                .last()
                .map(|(height, _, _)| height + 1)
                .unwrap_or_else(|| self.stable_token_block_chain.block_chain.next_block_index)
        }

        pub(super) fn contains_next_block_index(&self, next_block_index: BlockIndex) -> bool {
            self.blocks.iter().any(|(height, _, _)| *height == next_block_index)
                || self.stable_token_block_chain.cached.contains_key(&next_block_index)
        }

        pub(super) fn get_latest_block_hash(&self) -> HashOf<TokenBlock> {
            self.blocks
                .last()
                .map(|(_, _, hash)| *hash)
                .unwrap_or_else(|| self.stable_token_block_chain.block_chain.latest_block_hash)
        }

        pub(super) fn push_block(&mut self, encoded_block: EncodedBlock, block_hash: HashOf<TokenBlock>) {
            let block_height = self.get_next_block_index();
            self.blocks.push((block_height, encoded_block, block_hash));
        }

        pub fn get_fee_to(&self) -> Option<Account> {
            self.lock.fee_to
        }

        pub fn dump(self) {
            for (block_height, encoded_block, block_hash) in self.blocks.iter() {
                self.stable_token_block_chain
                    .cached
                    .insert(*block_height, encoded_block.clone());
                self.stable_token_block_chain.block_chain.next_block(*block_hash);
            }
        }
    }
}

impl TokenBlockChainGuard<'_> {
    fn get_next_token_block(
        &self,
        now: TimestampNanos,
        transaction: TokenTransaction,
    ) -> Result<(EncodedBlock, HashOf<TokenBlock>), BusinessError> {
        use ::common::utils::pb::to_proto_bytes;
        use ::common::{archive::token::TokenBlock, proto, types::DoHash};

        if self.contains_next_block_index(self.get_next_block_index()) {
            return Err(BusinessError::TokenBlockChainError(
                "The next block index is already in the cache.".to_string(),
            ));
        }

        let parent_hash = self.get_latest_block_hash();
        let block = TokenBlock(CandidBlock {
            parent_hash,
            timestamp: now,
            transaction,
        });
        let hash = block.do_hash().map_err(BusinessError::TokenBlockChainError)?;
        let block: proto::TokenBlock = block
            .try_into()
            .map_err(|err| BusinessError::TokenBlockChainError(format!("{err:?}")))?;
        let encoded_block = to_proto_bytes(&block).map_err(BusinessError::TokenBlockChainError)?;
        let encoded_block = EncodedBlock(encoded_block);
        Ok((encoded_block, hash))
    }

    pub fn mint_block<T, F>(
        &mut self,
        now: TimestampNanos,
        transaction: TokenTransaction,
        handle: F,
    ) -> Result<T, BusinessError>
    where
        F: FnOnce(&mut Self) -> Result<T, BusinessError>,
    {
        let (encoded_block, hash) = self.get_next_token_block(now, transaction)?;
        let data = handle(self)?;
        self.push_block(encoded_block, hash);
        Ok(data)
    }
}
