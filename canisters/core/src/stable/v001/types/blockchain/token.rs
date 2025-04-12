use common::{
    archive::token::{TokenBlock, TokenTransaction},
    common::{BlockIndex, CandidBlock, EncodedBlock, HashOf},
};
use ic_canister_kit::{common::trap, types::StableBTreeMap};
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
        ic_cdk::println!("🔒 Locked token block chain.");
    }

    pub fn be_guard<'a>(&'a mut self, lock: &'a TokenBlockChainLock) -> TokenBlockChainGuard<'a> {
        TokenBlockChainGuard {
            token_block_chain: self,
            _lock: lock,
        }
    }
}

// ============================ lock ============================

pub enum LockTokenBlockChainResult {
    Lock(TokenBlockChainLock),
    Retry(u8),
}
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
        transaction: TokenTransaction,
    ) -> Result<(EncodedBlock, HashOf<TokenBlock>), BusinessError> {
        use ::common::utils::pb::to_proto_bytes;
        use ::common::{archive::token::TokenBlock, common::TimestampNanos, proto};

        if self
            .token_block_chain
            .cached
            .contains_key(&self.token_block_chain.block_chain.next_block_index)
        {
            return Err(BusinessError::TokenBlockChainError(
                "The next block index is already in the cache.".to_string(),
            ));
        }

        let parent_hash = self.token_block_chain.block_chain.parent_hash;
        let timestamp = TimestampNanos::now();
        let block = TokenBlock(CandidBlock {
            parent_hash,
            timestamp,
            transaction,
        });
        let hash =
            common::common::DoHash::do_hash(&block).map_err(BusinessError::TokenBlockChainError)?;
        let block: proto::TokenBlock = block.try_into()?;
        let encoded_block = to_proto_bytes(&block).map_err(BusinessError::TokenBlockChainError)?;
        let encoded_block = EncodedBlock(encoded_block);
        Ok((encoded_block, hash))
    }

    pub fn push_block(&mut self, encoded_block: EncodedBlock, hash: HashOf<TokenBlock>) {
        let block_height = self.token_block_chain.block_chain.next_block_index;
        self.token_block_chain
            .cached
            .insert(block_height, encoded_block);
        self.token_block_chain.block_chain.next_block(hash);
    }
}
