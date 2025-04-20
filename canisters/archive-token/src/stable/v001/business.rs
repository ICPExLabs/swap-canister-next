use super::super::business::*;
use super::types::*;

impl Business for InnerState {
    fn business_queryable(&self, caller: &UserId) -> Result<(), String> {
        if self
            .business_data
            .maintainers
            .as_ref()
            .is_none_or(|maintainers| maintainers.contains(caller))
        {
            return Ok(());
        }
        Err("Only Maintainers are allowed to query data".into())
    }
    fn business_blocks_append_authorized(&self, caller: &UserId) -> Result<(), String> {
        if self
            .business_data
            .core_canister_id
            .is_some_and(|core| core == *caller)
        {
            return Ok(());
        }
        Err("'Only Core Canister is allowed to append blocks to an Archive Node'".into())
    }

    fn business_block_query(&self, block_height: BlockIndex) -> Option<EncodedBlock> {
        let adjusted_height = block_height.checked_sub(self.business_data.block_height_offset());
        let adjusted_height = trap(adjusted_height.ok_or("block height too small."));
        self.blocks.get_block(adjusted_height)
    }
    fn business_blocks_iter(&self, index_start: u64, length: u64) -> Vec<EncodedBlock> {
        let length = length.min(MAX_BLOCKS_PER_REQUEST);
        let blocks_len = self.blocks.blocks_len();
        let start = index_start.min(blocks_len);
        let end = std::cmp::min(start + length, blocks_len);
        let mut blocks = Vec::with_capacity((end - start) as usize);
        for index in start..end {
            let block = self.blocks.get_block(index);
            let block = trap(block.ok_or(format!("can not find block: {index}")));
            blocks.push(block);
        }
        blocks
    }
    fn business_blocks_query(
        &self,
        height_start: BlockIndex,
        length: u64,
    ) -> Result<Vec<EncodedBlock>, String> {
        use ::common::utils::range::*;

        let from_offset = self.business_data.block_height_offset();
        let length = length.min(MAX_BLOCKS_PER_REQUEST);
        let local_blocks_range = from_offset..from_offset + self.blocks.blocks_len();
        let requested_range = height_start..height_start + length;
        if !is_sub_range(&requested_range, &local_blocks_range) {
            return Err(format!(
                "Requested blocks outside the range stored in the archive node. Requested [{} .. {}]. Available [{} .. {}].",
                requested_range.start,
                requested_range.end,
                local_blocks_range.start,
                local_blocks_range.end
            ));
        }

        let mut blocks = Vec::with_capacity(length as usize);
        let offset_requested_range =
            requested_range.start - from_offset..requested_range.end - from_offset;
        for index in offset_requested_range {
            let block = self.blocks.get_block(index);
            let block = trap(block.ok_or(format!("can not find block by index: {index}")));
            blocks.push(block);
        }

        Ok(blocks)
    }
    fn business_blocks_get(
        &self,
        height_start: BlockIndex,
        length: u64,
    ) -> Result<Vec<EncodedBlock>, GetBlocksError> {
        use ::common::utils::range::*;

        let block_range = make_range(
            self.business_data.block_height_offset(),
            self.blocks.blocks_len(),
        );

        if height_start < block_range.start {
            return Err(GetBlocksError::BadFirstBlockIndex {
                requested_index: height_start,
                first_valid_index: block_range.start,
            });
        }

        let requested_range = make_range(height_start, length);
        let effective_range = match intersect(
            &block_range,
            &take(&requested_range, MAX_BLOCKS_PER_REQUEST),
        ) {
            Ok(range) => range,
            Err(NoIntersection) => return Ok(vec![]),
        };

        let mut encoded_blocks = Vec::with_capacity(range_len(&effective_range) as usize);
        for height in effective_range {
            let index = height - block_range.start;
            let block = self.blocks.get_block(index);
            let block = trap(block.ok_or(format!("can not find block by index: {index}")));
            encoded_blocks.push(block);
        }

        Ok(encoded_blocks)
    }

    fn business_remaining_capacity(&self) -> u64 {
        let remaining_capacity = self
            .business_data
            .max_memory_size_bytes
            .checked_sub(self.blocks.total_block_size());
        trap(remaining_capacity.ok_or("exceed max memory size"))
    }
    fn business_metrics(&self, w: &mut MetricsEncoder<Vec<u8>>) -> IoResult<()> {
        w.encode_gauge(
            "archive_node_block_height_offset",
            self.business_data.block_height_offset() as f64,
            "Block height offset assigned to this instance of the archive canister.",
        )?;
        w.encode_gauge(
            "archive_node_max_memory_size_bytes",
            self.business_data.max_memory_size_bytes as f64,
            "Maximum amount of memory this canister is allowed to use for blocks.",
        )?;
        // This value can increase/decrease in the current implementation.
        w.encode_gauge(
            "archive_node_blocks",
            self.blocks.blocks_len() as f64,
            "Number of blocks stored by this canister.",
        )?;
        w.encode_gauge(
            "archive_node_blocks_bytes",
            self.blocks.total_block_size() as f64,
            "Total amount of memory consumed by the blocks stored by this canister.",
        )?;
        w.encode_gauge(
            "archive_node_stable_memory_pages",
            ic_cdk::api::stable::stable_size() as f64,
            "Size of the stable memory allocated by this canister measured in 64K Wasm pages.",
        )?;
        w.encode_gauge(
            "stable_memory_bytes",
            (ic_cdk::api::stable::stable_size() * 64 * 1024) as f64,
            "Size of the stable memory allocated by this canister measured in bytes.",
        )?;
        w.encode_gauge(
            "heap_memory_bytes",
            common::utils::runtime::heap_memory_size_bytes() as f64,
            "Size of the heap memory allocated by this canister measured in bytes.",
        )?;
        w.encode_gauge(
            "archive_node_last_upgrade_time_seconds",
            self.business_data.last_upgrade_timestamp_ns as f64 / 1_000_000_000.0,
            "IC timestamp of the last upgrade performed on this canister.",
        )?;
        Ok(())
    }

    fn business_blocks_append(&mut self, blocks: Vec<EncodedBlock>) {
        self.business_remaining_capacity(); // would be failed if exceed max memory size
        ic_cdk::println!(
            "[archive node] append_blocks(): archive size: {} blocks, appending {} blocks",
            self.blocks.blocks_len(),
            blocks.len()
        );
        for block in &blocks {
            // 1. try to parse block bytes before append
            let token_block: proto::TokenBlock = trap(from_proto_bytes(&block.0));
            let token_block: TokenBlock = trap(token_block.try_into());
            // 2. check block hash before append
            if token_block.0.parent_hash != self.business_data.latest_block_hash {
                ic_cdk::trap(&format!(
                    "Parent hash mismatch. Expected: {}, got: {}",
                    self.business_data.latest_block_hash.hex(),
                    token_block.0.parent_hash.hex()
                ));
            }
            // 3. get current block hash
            let block_hash = trap(token_block.do_hash());
            // 4. push
            self.blocks.append_block(&block.0);
            // 5. update latest block hash
            self.business_data.latest_block_hash = block_hash;
        }
        if self.blocks.total_block_size() > self.business_data.max_memory_size_bytes {
            ic_cdk::trap("No space left");
        }
        ic_cdk::println!(
            "[archive node] append_blocks(): done. archive size: {} blocks",
            self.blocks.blocks_len()
        );
    }

    fn business_config_maintainers_set(&mut self, maintainers: Option<Vec<UserId>>) {
        self.business_data.maintainers =
            maintainers.map(|maintainers| maintainers.into_iter().collect());
    }
    fn business_config_max_memory_size_bytes_set(&mut self, max_memory_size_bytes: u64) {
        self.update_max_memory_size_bytes(max_memory_size_bytes)
    }

    fn business_latest_block_index_query(&self) -> Option<BlockIndex> {
        let length = self.blocks.blocks_len();
        if length == 0 {
            return None;
        }
        Some(self.business_data.block_height_offset() + length - 1)
    }
    fn business_metrics_query(&self) -> CustomMetrics {
        CustomMetrics {
            block_height_offset: self.business_data.block_height_offset(),
            max_memory_size_bytes: self.business_data.max_memory_size_bytes,
            blocks: self.blocks.blocks_len(),
            blocks_bytes: self.blocks.total_block_size(),
            stable_memory_pages: ic_cdk::api::stable::stable_size(),
            stable_memory_bytes: (ic_cdk::api::stable::stable_size() * 64 * 1024),
            heap_memory_bytes: common::utils::runtime::heap_memory_size_bytes() as u64,
            last_upgrade_time_seconds: self.business_data.last_upgrade_timestamp_ns
                / 1_000_000_000_u64,
        }
    }
}
