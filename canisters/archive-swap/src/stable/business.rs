#[allow(unused)]
use super::*;
#[allow(unused)]
pub use ic_canister_kit::types::UserId;

#[allow(unused_variables)]
pub trait Business: StableHeap {
    fn business_queryable(&self, caller: &UserId) -> Result<(), String> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_blocks_append_authorized(&self, caller: &UserId) -> Result<(), String> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_block_query(&self, block_height: BlockIndex) -> Option<EncodedBlock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_blocks_iter(&self, index_start: u64, length: u64) -> Vec<EncodedBlock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_blocks_query(
        &self,
        height_start: BlockIndex,
        length: u64,
    ) -> Result<Vec<EncodedBlock>, String> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_blocks_get(
        &self,
        height_start: BlockIndex,
        length: u64,
    ) -> Result<Vec<EncodedBlock>, GetBlocksError> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_remaining_capacity(&self) -> u64 {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_metrics(&self, w: &mut MetricsEncoder<Vec<u8>>) -> IoResult<()> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_blocks_append(&mut self, blocks: Vec<EncodedBlock>) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_config_maintainers_set(&mut self, maintainers: Option<Vec<UserId>>) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_max_memory_size_bytes_set(&mut self, max_memory_size_bytes: u64) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_latest_block_index_query(&self) -> Option<BlockIndex> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_metrics_query(&self) -> CustomMetrics {
        ic_cdk::trap("Not supported operation by this version.")
    }
}

// 业务实现
impl Business for State {
    fn business_queryable(&self, caller: &UserId) -> Result<(), String> {
        self.get().business_queryable(caller)
    }
    fn business_blocks_append_authorized(&self, caller: &UserId) -> Result<(), String> {
        self.get().business_blocks_append_authorized(caller)
    }

    fn business_block_query(&self, block_height: BlockIndex) -> Option<EncodedBlock> {
        self.get().business_block_query(block_height)
    }
    fn business_blocks_iter(&self, index_start: u64, length: u64) -> Vec<EncodedBlock> {
        self.get().business_blocks_iter(index_start, length)
    }
    fn business_blocks_query(
        &self,
        height_start: BlockIndex,
        length: u64,
    ) -> Result<Vec<EncodedBlock>, String> {
        self.get().business_blocks_query(height_start, length)
    }
    fn business_blocks_get(
        &self,
        height_start: BlockIndex,
        length: u64,
    ) -> Result<Vec<EncodedBlock>, GetBlocksError> {
        self.get().business_blocks_get(height_start, length)
    }

    fn business_remaining_capacity(&self) -> u64 {
        self.get().business_remaining_capacity()
    }
    fn business_metrics(&self, w: &mut MetricsEncoder<Vec<u8>>) -> IoResult<()> {
        self.get().business_metrics(w)
    }

    fn business_blocks_append(&mut self, blocks: Vec<EncodedBlock>) {
        self.get_mut().business_blocks_append(blocks)
    }

    fn business_config_maintainers_set(&mut self, maintainers: Option<Vec<UserId>>) {
        self.get_mut().business_config_maintainers_set(maintainers)
    }
    fn business_config_max_memory_size_bytes_set(&mut self, max_memory_size_bytes: u64) {
        self.get_mut()
            .business_config_max_memory_size_bytes_set(max_memory_size_bytes)
    }

    fn business_latest_block_index_query(&self) -> Option<BlockIndex> {
        self.get().business_latest_block_index_query()
    }
    fn business_metrics_query(&self) -> CustomMetrics {
        self.get().business_metrics_query()
    }
}
