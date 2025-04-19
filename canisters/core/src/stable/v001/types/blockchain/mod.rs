use std::sync::RwLock;

use candid::CandidType;
use ic_canister_kit::types::{CanisterId, UserId};
use serde::{Deserialize, Serialize};

use common::types::{BlockIndex, BusinessError, HashOf};

mod token;
pub use token::*;

mod swap;
pub use swap::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockChain<T> {
    // 归档相关
    pub archived: Vec<ArchivedBlocks>,               // 已经存档的块
    pub current_archiving: Option<CurrentArchiving>, // 当前正在存档的块
    pub archive_config: NextArchiveCanisterConfig,   // 下一个存档的配置
    pub archive_locked: RwLock<bool>,                // 标记是否获取锁,只有持有锁才能修改
    // 添加交易相关
    pub locked: RwLock<bool>,         // 标记是否获取锁,只有持有锁才能修改
    pub latest_block_hash: HashOf<T>, // 记录上一个块的 hash
    pub next_block_index: BlockIndex, // 记录下一个块的高度
}

impl<T> Default for BlockChain<T> {
    fn default() -> Self {
        Self {
            archived: Default::default(),
            current_archiving: Default::default(),
            archive_config: Default::default(),
            archive_locked: Default::default(),
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
    }

    fn next_block(&mut self, latest_block_hash: HashOf<T>) {
        self.latest_block_hash = latest_block_hash;
        self.next_block_index += 1;
    }

    fn set_current_archiving_max_length(&mut self, max_length: u64) -> Option<CurrentArchiving> {
        self.archive_config.max_length = max_length;
        if let Some(current_archiving) = &mut self.current_archiving {
            current_archiving.max_length = max_length;
        }
        self.current_archiving
    }
    fn set_archive_config(
        &mut self,
        archive_config: NextArchiveCanisterConfig,
    ) -> NextArchiveCanisterConfig {
        std::mem::replace(&mut self.archive_config, archive_config)
    }
    fn replace_current_archiving(
        &mut self,
        archiving: CurrentArchiving,
    ) -> Option<CurrentArchiving> {
        std::mem::replace(&mut self.current_archiving, Some(archiving))
    }

    fn archive_current_canister(&mut self) -> Result<(), BusinessError> {
        let current_archiving = match self.current_archiving {
            Some(current_archiving) => current_archiving,
            None => return Ok(()),
        };
        // 必须达到最大限度才能归档
        if current_archiving.length < current_archiving.max_length {
            return Err(common::types::system_error(format!(
                "can not archive canister because: current_length: {} < max_length:{}",
                current_archiving.length, current_archiving.max_length
            )));
        }
        let archived = ArchivedBlocks {
            canister_id: current_archiving.canister_id,
            block_height_offset: current_archiving.block_height_offset,
            length: current_archiving.length,
        };
        self.archived.push(archived);
        self.current_archiving = None;
        Ok(())
    }

    fn get_maintain_canisters(&self) -> Vec<CanisterId> {
        let mut canisters = self
            .archived
            .iter()
            .map(|a| a.canister_id)
            .collect::<Vec<_>>();
        if let Some(current_archiving) = &self.current_archiving {
            canisters.push(current_archiving.canister_id);
        }
        canisters
    }

    fn increment(&mut self, block_height: BlockIndex) -> bool {
        match self.current_archiving.as_mut() {
            Some(current_archiving) => current_archiving.increment(block_height),
            None => false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, CandidType)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, CandidType)]
pub struct CurrentArchiving {
    pub canister_id: CanisterId,
    pub block_height_offset: BlockIndex, // 起始偏移，如果有的话，第一个就是
    pub length: u64,                     // 该罐子已保存的个数
    pub max_length: u64,                 // 超过这个长度就不应该再往这个里面保存
}

impl CurrentArchiving {
    pub fn query(&self, block_height: BlockIndex) -> Option<CanisterId> {
        let block_height_start = self.block_height_offset;
        let block_height_end = self.block_height_offset + self.length;
        if block_height_start <= block_height && block_height < block_height_end {
            return Some(self.canister_id);
        }
        None
    }

    pub fn is_full(&self) -> bool {
        self.max_length <= self.length
    }
    pub fn remain(&self) -> u64 {
        self.max_length - self.length
    }
    pub fn increment(&mut self, block_height: BlockIndex) -> bool {
        if self.is_full() {
            return false;
        }
        if self.block_height_offset + self.length != block_height {
            return false;
        }
        self.length += 1;
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct NextArchiveCanisterConfig {
    pub maintainers: Option<Vec<UserId>>,   // 维护人
    pub max_memory_size_bytes: Option<u64>, // 最大内存
    pub max_length: u64,                    // 最大长度
}

impl Default for NextArchiveCanisterConfig {
    fn default() -> Self {
        Self {
            maintainers: None,
            max_memory_size_bytes: None,
            max_length: 1_000_000, // ? 估计 10 GB
        }
    }
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct BlockChainView<T> {
    pub archived: Vec<ArchivedBlocks>,               // 已经存档的块
    pub current_archiving: Option<CurrentArchiving>, // 当前正在存档的块
    pub archive_config: NextArchiveCanisterConfig,   // 下一个存档的配置
    pub latest_block_hash: HashOf<T>,                // 记录上一个块的 hash
    pub next_block_index: BlockIndex,                // 记录下一个块的高度
}

impl<T: Clone> From<&BlockChain<T>> for BlockChainView<T> {
    fn from(value: &BlockChain<T>) -> Self {
        Self {
            archived: value.archived.clone(),
            current_archiving: value.current_archiving,
            archive_config: value.archive_config.clone(),
            latest_block_hash: value.latest_block_hash,
            next_block_index: value.next_block_index,
        }
    }
}
