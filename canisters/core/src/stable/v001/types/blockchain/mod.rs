use std::sync::RwLock;

use ic_canister_kit::types::{CanisterId, UserId};
use serde::{Deserialize, Serialize};

use common::types::{BlockIndex, HashOf};

mod token;
pub use token::*;

mod swap;
pub use swap::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockChain<T> {
    pub archived: Vec<ArchivedBlocks>,             // 已经存档的块
    pub current_archiving: Option<ArchivedBlocks>, // 当前正在存档的块
    pub archive_config: NextArchiveCanisterConfig, // 下一个存档的配置
    pub locked: RwLock<bool>,                      // 标记是否获取锁,只有持有锁才能修改
    pub latest_block_hash: HashOf<T>,              // 记录上一个块的 hash
    pub next_block_index: BlockIndex,              // 记录下一个块的高度
}

impl<T> Default for BlockChain<T> {
    fn default() -> Self {
        Self {
            archived: Default::default(),
            current_archiving: Default::default(),
            archive_config: Default::default(),
            locked: Default::default(),
            latest_block_hash: HashOf::default(),
            next_block_index: Default::default(),
        }
    }
}

impl<T> BlockChain<T> {
    pub fn queryable(&self, caller: &UserId) -> bool {
        self.archive_config
            .maintainers
            .as_ref()
            .is_none_or(|maintainers| maintainers.contains(caller))
    }
    pub fn query(&self, block_height: BlockIndex) -> Option<CanisterId> {
        for archived in &self.archived {
            if let Some(canister_id) = archived.query(block_height) {
                return Some(canister_id);
            }
        }
        if let Some(current_archiving) = &self.current_archiving {
            if let Some(canister_id) = current_archiving.query(block_height) {
                return Some(canister_id);
            }
        }
        None
    }

    pub fn set_archive_maintainers(&mut self, maintainers: Option<Vec<UserId>>) {
        self.archive_config.maintainers = maintainers;
        // TODO 异步设置下辖的归档罐子的 maintainers
    }

    fn next_block(&mut self, latest_block_hash: HashOf<T>) {
        self.latest_block_hash = latest_block_hash;
        self.next_block_index += 1;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivedBlocks {
    pub canister_id: CanisterId,
    pub block_height_offset: BlockIndex, // 起始偏移，如果有的话，第一个就是
    pub length: u64,                     // 该罐子已保存的个数
}

impl ArchivedBlocks {
    pub fn query(&self, block_height: BlockIndex) -> Option<CanisterId> {
        let block_height_start = self.block_height_offset;
        let block_height_end = self.block_height_offset + self.length;
        if block_height_start <= block_height && block_height < block_height_end {
            return Some(self.canister_id);
        }
        None
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NextArchiveCanisterConfig {
    pub maintainers: Option<Vec<UserId>>,   // 维护人
    pub max_memory_size_bytes: Option<u64>, // 最大内存
    pub wasm: Option<Vec<u8>>,              // wasm
}
