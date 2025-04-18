use ic_canister_kit::{
    common::trap,
    types::{StableBTreeMap, UserId},
};
use serde::{Deserialize, Serialize};

use crate::types::{
    Account, BlockIndex, Business, BusinessError, CandidBlock, EncodedBlock, HashOf,
    QueryBlockResult, SwapBlock, SwapTransaction, TimestampNanos, with_mut_state_without_record,
};

use super::super::init_swap_blocks;

use super::BlockChain;

#[derive(Serialize, Deserialize)]
pub struct SwapBlockChain {
    #[serde(skip, default = "init_swap_blocks")]
    cached: StableBTreeMap<BlockIndex, EncodedBlock>, // 暂存所有缓存的块
    block_chain: BlockChain<SwapBlock>,
}

impl Default for SwapBlockChain {
    fn default() -> Self {
        Self {
            cached: init_swap_blocks(),
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
        with_mut_state_without_record(|s| s.business_swap_block_chain_archive_unlock())
    }
}

pub struct SwapBlockChainLock {
    pub fee_to: Option<Account>,
}

impl Drop for SwapBlockChainLock {
    fn drop(&mut self) {
        with_mut_state_without_record(|s| s.business_swap_block_chain_unlock())
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
