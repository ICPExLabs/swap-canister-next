use ic_canister_kit::common::trap;

use super::super::business::*;
use super::types::*;

impl Business for InnerState {
    fn business_block_query(&self, block_height: BlockIndex) -> Option<Vec<u8>> {
        let adjusted_height = block_height.checked_sub(self.business_data.block_height_offset);
        let adjusted_height = trap(adjusted_height.ok_or("block height too small."));
        self.blocks.get_block(adjusted_height)
    }

    fn business_remaining_capacity(&self) -> u64 {
        let remaining_capacity = self
            .business_data
            .max_memory_size_bytes
            .checked_sub(self.blocks.total_block_size());
        trap(remaining_capacity.ok_or("exceed max memory size"))
    }

    fn business_blocks_append_authorized(&self, caller: &UserId) -> Result<(), String> {
        if self
            .business_data
            .core_canister_id
            .is_some_and(|core| core == *caller)
        {
            return Ok(());
        }
        Err("Only Core canister is allowed to append blocks to an Archive Node".into())
    }
    fn business_blocks_append(&mut self, blocks: Vec<Vec<u8>>) {
        self.business_remaining_capacity(); // would be failed if exceed max memory size
        ic_cdk::println!(
            "[archive node] append_blocks(): archive size: {} blocks, appending {} blocks",
            self.blocks.blocks_len(),
            blocks.len()
        );
        for block in &blocks {
            self.blocks.append_block(block);
        }
        if self.blocks.total_block_size() > self.business_data.max_memory_size_bytes {
            ic_cdk::trap("No space left");
        }
        ic_cdk::println!(
            "[archive node] append_blocks(): done. archive size: {} blocks",
            self.blocks.blocks_len()
        );
    }

    fn business_example_query(&self) -> String {
        self.example_data.clone()
    }

    fn business_example_update(&mut self, test: String) {
        self.example_data = test
    }

    fn business_example_cell_query(&self) -> ExampleCell {
        self.example_cell.get().clone()
    }

    fn business_example_cell_update(&mut self, test: String) {
        use ic_canister_kit::common::trap_debug;
        let mut cell = self.example_cell.get().to_owned();
        cell.cell_data = test;
        trap_debug(self.example_cell.set(cell));
    }

    fn business_example_vec_query(&self) -> Vec<ExampleVec> {
        self.example_vec.iter().collect()
    }

    fn business_example_vec_push(&mut self, test: u64) {
        use ic_canister_kit::common::trap;
        trap(self.example_vec.push(&ExampleVec { vec_data: test }))
    }
    fn business_example_vec_pop(&mut self) -> Option<ExampleVec> {
        self.example_vec.pop()
    }

    fn business_example_map_query(&self) -> HashMap<u64, String> {
        self.example_map.iter().collect()
    }
    fn business_example_map_update(&mut self, key: u64, value: Option<String>) -> Option<String> {
        if let Some(value) = value {
            self.example_map.insert(key, value)
        } else {
            self.example_map.remove(&key)
        }
    }

    fn business_example_log_query(&self) -> Vec<String> {
        self.example_log.iter().collect()
    }

    fn business_example_log_update(&mut self, item: String) -> u64 {
        use ic_canister_kit::common::trap_debug;
        trap_debug(self.example_log.append(&item))
    }

    fn business_example_priority_queue_query(&self) -> Vec<ExampleVec> {
        self.example_priority_queue.iter().collect()
    }

    fn business_example_priority_queue_push(&mut self, item: u64) {
        use ic_canister_kit::common::trap;
        let result = self
            .example_priority_queue
            .push(&ExampleVec { vec_data: item });
        trap(result);
    }
    fn business_example_priority_queue_pop(&mut self) -> Option<ExampleVec> {
        self.example_priority_queue.pop()
    }
}
