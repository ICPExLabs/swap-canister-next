use common::{
    archive::token::{TokenBlock, TokenTransaction},
    types::{BlockIndex, CandidBlock, EncodedBlock, HashOf, QueryBlockResult, TimestampNanos},
};
use ic_canister_kit::{
    common::trap,
    types::{StableBTreeMap, UserId},
};
use serde::{Deserialize, Serialize};

use crate::types::{Business, BusinessError, with_mut_state_without_record};

use super::super::init_token_blocks;

use super::BlockChain;

#[derive(Serialize, Deserialize)]
pub struct TokenBlockChain {
    #[serde(skip, default = "init_token_blocks")]
    cached: StableBTreeMap<BlockIndex, EncodedBlock>, // 暂存所有缓存的块
    block_chain: BlockChain<TokenBlock>,
}

impl Default for TokenBlockChain {
    fn default() -> Self {
        Self {
            cached: init_token_blocks(),
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

    pub fn set_archive_maintainers(&mut self, maintainers: Option<Vec<UserId>>) {
        self.block_chain.set_archive_maintainers(maintainers);
    }

    // locks
    pub fn lock(&mut self) -> Option<TokenBlockChainLock> {
        let mut locked = trap(self.block_chain.locked.write()); // ! what if failed ?

        if *locked {
            return None;
        }

        *locked = true;
        ic_cdk::println!("🔒 Locked token block chain.");

        Some(TokenBlockChainLock)
    }

    pub fn unlock(&mut self) {
        let mut locked = trap(self.block_chain.locked.write()); // ! what if failed ?

        // 1. check first
        if !*locked {
            // if not true, terminator
            let tips = "Unlock a token block chain failed. That is not locked.";
            ic_cdk::trap(tips); // never be here
        }

        // 2. do unlock
        *locked = false;
        ic_cdk::println!("🔐 Unlock token block chain.");
    }

    pub fn be_guard<'a>(&'a mut self, lock: &'a TokenBlockChainLock) -> TokenBlockChainGuard<'a> {
        TokenBlockChainGuard {
            token_block_chain: self,
            _lock: lock,
        }
    }

    pub fn get_latest_hash(&self) -> &[u8] {
        self.block_chain.latest_block_hash.as_slice()
    }
}

// ============================ lock ============================

pub struct TokenBlockChainLock;

impl Drop for TokenBlockChainLock {
    fn drop(&mut self) {
        with_mut_state_without_record(|s| s.business_token_block_chain_unlock())
    }
}

// ============================ guard ============================

pub struct TokenBlockChainGuard<'a> {
    token_block_chain: &'a mut TokenBlockChain,
    _lock: &'a TokenBlockChainLock,
}

impl TokenBlockChainGuard<'_> {
    pub fn push_token_transaction(
        &self,
        now: TimestampNanos,
        transaction: TokenTransaction,
    ) -> Result<(EncodedBlock, HashOf<TokenBlock>), BusinessError> {
        use ::common::utils::pb::to_proto_bytes;
        use ::common::{archive::token::TokenBlock, proto, types::DoHash};

        if self
            .token_block_chain
            .cached
            .contains_key(&self.token_block_chain.block_chain.next_block_index)
        {
            return Err(BusinessError::TokenBlockChainError(
                "The next block index is already in the cache.".to_string(),
            ));
        }

        let parent_hash = self.token_block_chain.block_chain.latest_block_hash;
        let block = TokenBlock(CandidBlock {
            parent_hash,
            timestamp: now,
            transaction,
        });
        let hash = block
            .do_hash()
            .map_err(BusinessError::TokenBlockChainError)?;
        let block: proto::TokenBlock = block
            .try_into()
            .map_err(|err| BusinessError::TokenBlockChainError(format!("{err:?}")))?;
        let encoded_block = to_proto_bytes(&block).map_err(BusinessError::TokenBlockChainError)?;
        let encoded_block = EncodedBlock(encoded_block);
        Ok((encoded_block, hash))
    }

    pub fn push_block(&mut self, encoded_block: EncodedBlock, block_hash: HashOf<TokenBlock>) {
        let block_height = self.token_block_chain.block_chain.next_block_index;
        self.token_block_chain
            .cached
            .insert(block_height, encoded_block);
        self.token_block_chain.block_chain.next_block(block_hash);
    }
}
