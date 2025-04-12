use std::sync::RwLock;

use common::common::{BlockIndex, HashOf};
use ic_canister_kit::types::{CanisterId, UserId};

mod token;
use serde::{Deserialize, Serialize};
pub use token::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockChain<T> {
    pub archived: Vec<ArchivedBlocks>,             // 已经存档的块
    pub current_archiving: Option<ArchivedBlocks>, // 当前正在存档的块
    pub archive_config: NextArchiveCanisterConfig, // 下一个存档的配置
    pub locked: RwLock<bool>,                      // 标记是否获取锁,只有持有锁才能修改
    pub parent_hash: HashOf<T>,                    // 记录上一个块的 hash
    pub next_block_index: BlockIndex,              // 记录下一个块的高度
}

impl<T> Default for BlockChain<T> {
    fn default() -> Self {
        Self {
            archived: Default::default(),
            current_archiving: Default::default(),
            archive_config: Default::default(),
            locked: Default::default(),
            parent_hash: HashOf::new([0; 32]),
            next_block_index: Default::default(),
        }
    }
}

impl<T> BlockChain<T> {
    fn next_block(&mut self, parent_hash: HashOf<T>) {
        self.parent_hash = parent_hash;
        self.next_block_index += 1;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivedBlocks {
    pub canister_id: CanisterId,
    pub block_height_offset: BlockIndex, // 起始偏移，如果有的话，第一个就是
    pub length: u64,                     // 该罐子已保存的个数
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NextArchiveCanisterConfig {
    pub maintainers: Option<Vec<UserId>>,   // 维护人
    pub max_memory_size_bytes: Option<u64>, // 最大内存
    pub wasm: Option<Vec<u8>>,              // wasm
}
