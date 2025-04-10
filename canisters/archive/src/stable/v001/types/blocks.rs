use ic_canister_kit::common::trap;

use super::*;

pub struct Blocks(StableLog<Vec<u8>>);

impl Blocks {
    pub fn new(inner: StableLog<Vec<u8>>) -> Self {
        Self(inner)
    }

    pub fn get_block(&self, index: u64) -> Option<Vec<u8>> {
        self.0.get(index)
    }

    pub fn total_block_size(&self) -> u64 {
        self.0.log_size_bytes()
    }

    pub fn blocks_len(&self) -> u64 {
        self.0.len()
    }

    pub fn append_block(&mut self, block: &Vec<u8>) {
        let result = self.0.append(block);
        trap(result.map_err(|err| format!("Could not append block to stable block log: {err:?}")));
    }
}
