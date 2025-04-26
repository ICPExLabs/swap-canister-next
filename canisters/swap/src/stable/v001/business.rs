use super::super::business::*;
use super::types::*;

impl Business for InnerState {
    fn business_updated(&self) -> u64 {
        self.business_data.updated.into_inner()
    }

    // ======================== config ========================

    // fee to
    fn business_config_fee_to_query(&self) -> FeeTo {
        self.business_data.fee_to
    }
    fn business_config_fee_to_replace(&mut self, fee_to: FeeTo) -> FeeTo {
        self.updated(|s| std::mem::replace(&mut s.business_data.fee_to, fee_to))
    }

    // archive canister
    // token
    fn business_config_token_block_chain_query(&self) -> &BlockChain<TokenBlock> {
        self.token_block_chain.get_token_block_chain()
    }
    fn business_config_token_archive_wasm_module_query(&self) -> &Option<Vec<u8>> {
        self.token_block_chain.query_wasm_module()
    }
    fn business_config_token_archive_wasm_module_replace(
        &mut self,
        wasm_module: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, BusinessError> {
        self.updated(|s| s.token_block_chain.replace_wasm_module(wasm_module))
    }
    fn business_config_token_current_archiving_max_length_replace(
        &mut self,
        max_length: u64,
    ) -> Option<CurrentArchiving> {
        self.updated(|s| s.token_block_chain.set_token_current_archiving_max_length(max_length))
    }
    fn business_config_token_archive_config_replace(
        &mut self,
        archive_config: NextArchiveCanisterConfig,
    ) -> NextArchiveCanisterConfig {
        self.updated(|s| s.token_block_chain.set_token_archive_config(archive_config))
    }
    fn business_config_token_current_archiving_replace(
        &mut self,
        archiving: CurrentArchiving,
    ) -> Option<CurrentArchiving> {
        self.updated(|s| s.token_block_chain.replace_token_current_archiving(archiving))
    }
    fn business_config_token_archive_current_canister(&mut self) -> Result<(), BusinessError> {
        self.updated(|s| s.token_block_chain.archive_current_canister())
    }
    fn business_config_token_parent_hash_get(&self, block_height: BlockIndex) -> Option<HashOf<TokenBlock>> {
        self.token_block_chain.get_parent_hash(block_height)
    }
    fn business_config_token_cached_block_get(&self) -> Option<(BlockIndex, u64)> {
        self.token_block_chain.get_cached_block_index()
    }
    fn business_config_token_block_archived(&mut self, block_height: BlockIndex) -> Result<(), BusinessError> {
        self.updated(|s| s.token_block_chain.archived_block(block_height))
    }

    // swap
    fn business_config_swap_block_chain_query(&self) -> &BlockChain<SwapBlock> {
        self.swap_block_chain.get_swap_block_chain()
    }
    fn business_config_swap_archive_wasm_module_query(&self) -> &Option<Vec<u8>> {
        self.swap_block_chain.query_wasm_module()
    }
    fn business_config_swap_archive_wasm_module_replace(
        &mut self,
        wasm_module: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, BusinessError> {
        self.updated(|s| s.swap_block_chain.replace_wasm_module(wasm_module))
    }
    fn business_config_swap_current_archiving_max_length_replace(
        &mut self,
        max_length: u64,
    ) -> Option<CurrentArchiving> {
        self.updated(|s| s.swap_block_chain.set_swap_current_archiving_max_length(max_length))
    }
    fn business_config_swap_archive_config_replace(
        &mut self,
        archive_config: NextArchiveCanisterConfig,
    ) -> NextArchiveCanisterConfig {
        self.updated(|s| s.swap_block_chain.set_swap_archive_config(archive_config))
    }
    fn business_config_swap_current_archiving_replace(
        &mut self,
        archiving: CurrentArchiving,
    ) -> Option<CurrentArchiving> {
        self.updated(|s| s.swap_block_chain.replace_swap_current_archiving(archiving))
    }
    fn business_config_swap_archive_current_canister(&mut self) -> Result<(), BusinessError> {
        self.updated(|s| s.swap_block_chain.archive_current_canister())
    }
    fn business_config_swap_parent_hash_get(&self, block_height: BlockIndex) -> Option<HashOf<SwapBlock>> {
        self.swap_block_chain.get_parent_hash(block_height)
    }
    fn business_config_swap_cached_block_get(&self) -> Option<(BlockIndex, u64)> {
        self.swap_block_chain.get_cached_block_index()
    }
    fn business_config_swap_block_archived(&mut self, block_height: BlockIndex) -> Result<(), BusinessError> {
        self.updated(|s| s.swap_block_chain.archived_block(block_height))
    }

    // maintain archives
    fn business_config_maintain_archives_query(&self) -> &MaintainArchives {
        &self.business_data.maintain_archives
    }
    fn business_config_maintain_archives_set(&mut self, config: MaintainArchivesConfig) {
        self.updated(|s| s.business_data.maintain_archives.update_config(config));
    }
    fn business_config_maintain_trigger(&mut self, now: TimestampNanos) -> bool {
        self.updated(|s| s.business_data.maintain_archives.is_trigger(now))
    }
    fn business_config_maintain_canisters(&self) -> Vec<CanisterId> {
        let tokens = self.token_block_chain.get_maintain_canisters();
        let swaps = self.swap_block_chain.get_maintain_canisters();
        let mut canisters = Vec::with_capacity(tokens.len() + swaps.len());
        canisters.extend_from_slice(&tokens);
        canisters.extend_from_slice(&swaps);
        canisters
    }
    fn business_config_maintain_archives_cycles_recharged(&mut self, canister_id: CanisterId, cycles: u128) {
        self.updated(|s| s.business_data.maintain_archives.cycles_recharged(canister_id, cycles))
    }

    // token frozen
    fn business_config_token_frozen_query(&self) -> &HashSet<CanisterId> {
        self.tokens.get_frozen_tokens()
    }
    fn business_config_token_frozen(&mut self, token: CanisterId, frozen: bool) {
        self.tokens.frozen_token(token, frozen)
    }

    // token custom
    fn business_config_token_preset_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        self.tokens.get_preset_tokens()
    }
    fn business_config_token_custom_query(&self) -> Vec<TokenInfo> {
        self.tokens
            .get_custom_tokens()
            .iter()
            .map(|(_, info)| info.clone())
            .collect()
    }
    fn business_config_token_custom_put(&mut self, token: TokenInfo) {
        self.tokens.put_custom_token(token)
    }

    // set_certified_data
    fn business_certified_data_refresh(&self) {
        let token_hash = self.token_block_chain.get_latest_hash();
        let swap_hash = self.swap_block_chain.get_latest_hash();
        let mut data = Vec::with_capacity(token_hash.len() + swap_hash.len());
        data.extend_from_slice(token_hash);
        data.extend_from_slice(swap_hash);
        let hash = common::utils::hash::hash_sha256(&data);
        ic_cdk::api::set_certified_data(&hash);
    }

    // ======================== locks ========================

    // token block chain
    fn business_token_block_chain_archive_lock(&mut self) -> Option<TokenBlockChainArchiveLock> {
        self.updated(|s| s.token_block_chain.archive_lock())
    }
    fn business_token_block_chain_archive_unlock(&mut self) {
        self.updated(|s| s.token_block_chain.archive_unlock())
    }
    fn business_token_block_chain_lock(&mut self) -> Option<TokenBlockChainLock> {
        self.updated(|s| s.token_block_chain.lock(s.business_data.fee_to.token_fee_to))
    }
    fn business_token_block_chain_unlock(&mut self) {
        self.updated(|s| s.token_block_chain.unlock())
    }

    // swap block chain
    fn business_swap_block_chain_archive_lock(&mut self) -> Option<SwapBlockChainArchiveLock> {
        self.updated(|s| s.swap_block_chain.archive_lock())
    }
    fn business_swap_block_chain_archive_unlock(&mut self) {
        self.updated(|s| s.swap_block_chain.archive_unlock())
    }
    fn business_swap_block_chain_lock(&mut self) -> Option<SwapBlockChainLock> {
        self.updated(|s| s.swap_block_chain.lock(s.business_data.fee_to.swap_fee_to))
    }
    fn business_swap_block_chain_unlock(&mut self) {
        self.updated(|s| s.swap_block_chain.unlock())
    }

    // token balance
    fn business_token_balance_lock(
        &mut self,
        required: Vec<TokenAccount>,
    ) -> Result<TokenBalancesLock, Vec<TokenAccount>> {
        self.updated(|s| s.token_balances.lock(required))
    }
    fn business_token_balance_unlock(&mut self, locked: &HashSet<TokenAccount>) {
        self.updated(|s| s.token_balances.unlock(locked))
    }

    // ======================== token block chain ========================

    // ======================== query ========================

    // tokens query
    fn business_token_alive(&self, canister_id: &CanisterId) -> Result<(), BusinessError> {
        self.tokens.token_alive(canister_id)
    }
    fn business_tokens_query(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        self.tokens.get_all_tokens()
    }
    fn business_dummy_tokens_query(&self) -> HashMap<CanisterId, TokenInfo> {
        self.token_pairs.query_dummy_tokens(&self.business_tokens_query())
    }
    fn business_all_tokens_with_dummy_query(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        self.business_tokens_query()
            .into_iter()
            .chain(
                self.business_dummy_tokens_query()
                    .into_iter()
                    .map(|(token, info)| (token, Cow::Owned(info))),
            )
            .collect()
    }
    fn business_token_query(&self, token: &CanisterId) -> Option<TokenInfo> {
        self.business_tokens_query()
            .remove(token)
            .map(|t| t.into_owned())
            .or_else(|| self.business_dummy_tokens_query().remove(token))
    }
    fn business_token_query_by_pa(&self, pa: &TokenPairAmm) -> Option<TokenInfo> {
        self.token_pairs
            .query_dummy_token_info(&self.business_tokens_query(), pa)
    }
    fn business_token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        ic_canister_kit::common::trap_debug(self.token_balances.token_balance_of(token, account))
    }
    fn business_token_balance_of_with_fee_to(
        &self,
        token: CanisterId,
        account: Account,
    ) -> (candid::Nat, Option<Account>) {
        (
            ic_canister_kit::common::trap_debug(self.token_balances.token_balance_of(token, account)),
            self.business_data.fee_to.token_fee_to,
        )
    }

    // ======================== update ========================

    fn business_token_deposit(
        &mut self,
        locks: &(TokenBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<DepositToken>,
        height: Nat,
    ) -> Result<Nat, BusinessError> {
        self.updated(|s| {
            let mut guard = s.get_token_guard(locks, arg.clone(), None)?;
            let height = guard.token_deposit(arg, height)?; // do deposit
            s.business_certified_data_refresh(); // set certified data
            Ok(height)
        })
    }
    fn business_token_withdraw(
        &mut self,
        locks: &(TokenBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<WithdrawToken>,
        height: Nat,
    ) -> Result<Nat, BusinessError> {
        self.updated(|s| {
            let mut guard = s.get_token_guard(locks, arg.clone(), None)?;
            let height = guard.token_withdraw(arg, height)?; // do withdraw
            s.business_certified_data_refresh(); // set certified data
            Ok(height)
        })
    }
    fn business_token_transfer(
        &mut self,
        locks: &(TokenBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        self.updated(|s| {
            let mut guard = s.get_token_guard(locks, arg.clone(), None)?;
            let changed = guard.token_transfer(arg)?; // do transfer
            s.business_certified_data_refresh(); // set certified data
            Ok(changed)
        })
    }
    fn business_token_transfer_lp(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        let pa = self
            .token_pairs
            .query_all_token_pair_pools()
            .into_iter()
            .find(|(_, maker)| maker.dummy_canisters().contains(&arg.arg.token))
            .map(|(pa, _)| pa)
            .ok_or(BusinessError::SystemError("must be lp token".to_string()))?;
        self.updated(|s| {
            let mut guard = s.get_pair_swap_guard(locks, arg.clone(), None)?;
            let changed = guard.token_lp_transfer(pa, arg)?; // do transfer
            s.business_certified_data_refresh(); // set certified data
            Ok(changed)
        })
    }

    // ======================== swap block chain ========================

    // ======================== token pair swap ========================

    // query
    fn business_token_pair_pools_query(&self) -> Vec<(TokenPairAmm, MarketMaker)> {
        self.token_pairs.query_all_token_pair_pools()
    }
    fn business_token_pair_pool_get(&self, pa: &TokenPairAmm) -> Option<MarketMaker> {
        self.token_pairs.get_token_pair_pool(pa)
    }
    // create
    fn business_token_pair_pool_create(
        &mut self,
        lock: &SwapBlockChainLock,
        arg: ArgWithMeta<TokenPairAmm>,
    ) -> Result<MarketMaker, BusinessError> {
        self.updated(|s| {
            let tokens = s.business_tokens_query();
            let token0 = tokens
                .get(&arg.arg.pair.token0)
                .ok_or(BusinessError::NotSupportedToken(arg.arg.pair.token0))?
                .clone()
                .into_owned();
            let token1 = tokens
                .get(&arg.arg.pair.token1)
                .ok_or(BusinessError::NotSupportedToken(arg.arg.pair.token1))?
                .clone()
                .into_owned();

            let mut swap_guard = s.swap_block_chain.be_guard(lock);
            let mut trace_guard = s
                .request_traces
                .be_guard(arg.clone().into(), None, Some(&swap_guard), None, None)?;
            let maker =
                s.token_pairs
                    .create_token_pair_pool(&mut swap_guard, &mut trace_guard, arg, &token0, &token1)?;
            s.business_certified_data_refresh(); // set certified data
            Ok(maker)
        })
    }
    // liquidity
    fn business_token_pair_liquidity_add(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairLiquidityAddArg>,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        self.updated(|s| {
            let mut guard = s.get_pair_swap_guard(locks, arg.clone(), None)?;
            let success = guard.add_liquidity(arg)?;
            s.business_certified_data_refresh(); // set certified data
            Ok(success)
        })
    }
    fn business_token_pair_check_liquidity_removable(
        &self,
        pa: &TokenPairAmm,
        from: &Account,
        liquidity_without_fee: &Nat,
    ) -> Result<(), BusinessError> {
        self.token_pairs.check_liquidity_removable(
            &self.token_balances,
            pa,
            from,
            liquidity_without_fee,
            self.business_data.fee_to.token_fee_to,
        )
    }
    fn business_token_pair_liquidity_remove(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairLiquidityRemoveArg>,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        self.updated(|s| {
            let mut guard = s.get_pair_swap_guard(locks, arg.clone(), None)?;
            let success = guard.remove_liquidity(arg)?;
            s.business_certified_data_refresh(); // set certified data
            Ok(success)
        })
    }

    // pair swap
    fn business_token_pair_swap_fixed_in_checking(
        &self,
        arg: &TokenPairSwapExactTokensForTokensArg,
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        self.token_pairs.get_amounts_out(
            &arg.self_canister,
            &arg.amount_in,
            &arg.amount_out_min,
            &arg.path,
            &arg.pas,
        ) // ? check again
    }
    fn business_token_pair_swap_fixed_out_checking(
        &self,
        arg: &TokenPairSwapTokensForExactTokensArg,
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        self.token_pairs.get_amounts_in(
            &arg.self_canister,
            &arg.amount_out,
            &arg.amount_in_max,
            &arg.path,
            &arg.pas,
        ) // ? check again
    }
    fn business_token_pair_swap_exact_tokens_for_tokens(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairSwapExactTokensForTokensArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.updated(|s| {
            let mut guard = s.get_pair_swap_guard(locks, arg.clone(), None)?;
            let success = guard.swap_exact_tokens_for_tokens(arg)?;
            s.business_certified_data_refresh(); // set certified data
            Ok(success)
        })
    }
    fn business_token_pair_swap_tokens_for_exact_tokens(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairSwapTokensForExactTokensArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.updated(|s| {
            let mut guard = s.get_pair_swap_guard(locks, arg.clone(), None)?;
            let success = guard.swap_tokens_for_exact_tokens(arg)?;
            s.business_certified_data_refresh(); // set certified data
            Ok(success)
        })
    }
    fn business_token_pair_swap_by_loan(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairSwapByLoanArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.updated(|s| {
            let mut guard = s.get_pair_swap_guard(locks, arg.clone(), None)?;
            let success = guard.swap_by_loan(arg)?;
            s.business_certified_data_refresh(); // set certified data
            Ok(success)
        })
    }

    // ======================== blocks query ========================

    fn business_token_queryable(&self, caller: &UserId) -> Result<(), String> {
        if self.token_block_chain.queryable(caller) {
            return Ok(());
        }
        Err("Only Maintainers are allowed to query data".into())
    }
    fn business_swap_queryable(&self, caller: &UserId) -> Result<(), String> {
        if self.swap_block_chain.queryable(caller) {
            return Ok(());
        }
        Err("Only Maintainers are allowed to query data".into())
    }

    fn business_token_block_get(&self, block_height: BlockIndex) -> QueryBlockResult<EncodedBlock> {
        self.token_block_chain.query(block_height)
    }
    fn business_swap_block_get(&self, block_height: BlockIndex) -> QueryBlockResult<EncodedBlock> {
        self.swap_block_chain.query(block_height)
    }

    // ======================== request ========================

    fn business_request_index_get(&self) -> (RequestIndex, u64) {
        self.request_traces.get_request_index()
    }
    fn business_request_trace_get(&self, index: &RequestIndex) -> Option<RequestTrace> {
        self.request_traces.get_request_trace(index)
    }
    fn business_request_trace_remove(&mut self, index: &RequestIndex) -> Option<RequestTrace> {
        self.updated(|s| s.request_traces.remove_request_trace(index))
    }
    fn business_request_trace_insert(&mut self, trace: RequestTrace) {
        self.updated(|s| s.request_traces.insert_request_trace(trace))
    }
}
