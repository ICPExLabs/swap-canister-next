#[allow(unused)]
use super::*;
#[allow(unused)]
pub use ic_canister_kit::identity::self_canister_id;
#[allow(unused)]
pub use ic_canister_kit::types::{CanisterId, PauseReason, UserId};
#[allow(unused)]
pub use std::collections::{HashMap, HashSet};
#[allow(unused)]
pub use std::fmt::Display;

#[allow(unused_variables)]
pub trait Business:
    Pausable<PauseReason>
    + ParsePermission
    + Permissable<Permission>
    + Recordable<Record, RecordTopic, RecordSearch>
    + Schedulable
    + ScheduleTask
    + StableHeap
{
    fn business_maintainer(&self, caller: &UserId) -> Result<(), String> {
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

    fn business_blocks_append(&mut self, blocks: Vec<EncodedBlock>) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_example_query(&self) -> String {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_example_update(&mut self, test: String) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_example_cell_query(&self) -> crate::stable::ExampleCell {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_example_cell_update(&mut self, test: String) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_example_vec_query(&self) -> Vec<crate::stable::ExampleVec> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_example_vec_push(&mut self, test: u64) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_example_vec_pop(&mut self) -> Option<crate::stable::ExampleVec> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_example_map_query(&self) -> HashMap<u64, String> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_example_map_update(&mut self, key: u64, value: Option<String>) -> Option<String> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_example_log_query(&self) -> Vec<String> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_example_log_update(&mut self, item: String) -> u64 {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_example_priority_queue_query(&self) -> Vec<crate::stable::ExampleVec> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_example_priority_queue_push(&mut self, item: u64) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_example_priority_queue_pop(&mut self) -> Option<crate::stable::ExampleVec> {
        ic_cdk::trap("Not supported operation by this version.")
    }
}

// 业务实现
impl Business for State {
    fn business_maintainer(&self, caller: &UserId) -> Result<(), String> {
        self.get().business_maintainer(caller)
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

    fn business_blocks_append(&mut self, blocks: Vec<EncodedBlock>) {
        self.get_mut().business_blocks_append(blocks)
    }

    fn business_example_query(&self) -> String {
        self.get().business_example_query()
    }
    fn business_example_update(&mut self, test: String) {
        self.get_mut().business_example_update(test)
    }

    fn business_example_cell_query(&self) -> ExampleCell {
        self.get().business_example_cell_query()
    }
    fn business_example_cell_update(&mut self, test: String) {
        self.get_mut().business_example_cell_update(test)
    }

    fn business_example_vec_query(&self) -> Vec<ExampleVec> {
        self.get().business_example_vec_query()
    }
    fn business_example_vec_push(&mut self, test: u64) {
        self.get_mut().business_example_vec_push(test)
    }
    fn business_example_vec_pop(&mut self) -> Option<ExampleVec> {
        self.get_mut().business_example_vec_pop()
    }

    fn business_example_map_query(&self) -> HashMap<u64, String> {
        self.get().business_example_map_query()
    }
    fn business_example_map_update(&mut self, key: u64, value: Option<String>) -> Option<String> {
        self.get_mut().business_example_map_update(key, value)
    }

    fn business_example_log_query(&self) -> Vec<String> {
        self.get().business_example_log_query()
    }
    fn business_example_log_update(&mut self, item: String) -> u64 {
        self.get_mut().business_example_log_update(item)
    }

    fn business_example_priority_queue_query(&self) -> Vec<ExampleVec> {
        self.get().business_example_priority_queue_query()
    }
    fn business_example_priority_queue_push(&mut self, item: u64) {
        self.get_mut().business_example_priority_queue_push(item)
    }
    fn business_example_priority_queue_pop(&mut self) -> Option<ExampleVec> {
        self.get_mut().business_example_priority_queue_pop()
    }
}
