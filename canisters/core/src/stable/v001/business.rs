use super::super::business::*;
use super::types::*;

impl Business for InnerState {
    // ======================== config ========================

    // // fee to
    // fn business_config_fee_to_query(&self) -> Option<&Account> {
    //     self.business_data.fee_to.as_ref()
    // }
    // fn business_config_fee_to_replace(&mut self, fee_to: Option<Account>) -> Option<Account> {
    //     std::mem::replace(&mut self.business_data.fee_to, fee_to)
    // }

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

    // token balance
    fn business_token_balance_lock(
        &mut self,
        fee_to: Vec<CanisterId>,
        required: Vec<TokenAccount>,
    ) -> Result<TokenBalancesLock, Vec<TokenAccount>> {
        self.token_balances.lock(
            self.business_data
                .fee_to
                .as_ref()
                .map(|account| {
                    fee_to
                        .into_iter()
                        .map(|token| TokenAccount::new(token, *account))
                        .collect()
                })
                .unwrap_or_default(),
            required,
        )
    }
    fn business_token_balance_unlock(&mut self, locked: &HashSet<TokenAccount>) {
        self.token_balances.unlock(locked)
    }

    // token block chain
    fn business_token_block_chain_lock(&mut self) -> Option<TokenBlockChainLock> {
        self.token_block_chain.lock()
    }
    fn business_token_block_chain_unlock(&mut self) {
        self.token_block_chain.unlock()
    }

    // swap block chain
    fn business_swap_block_chain_lock(&mut self) -> Option<SwapBlockChainLock> {
        self.swap_block_chain.lock()
    }
    fn business_swap_block_chain_unlock(&mut self) {
        self.swap_block_chain.unlock()
    }

    // ======================== token block chain ========================

    // ======================== query ========================

    // tokens query
    fn business_tokens_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        &TOKENS
    }
    fn business_dummy_tokens_query(&self) -> HashMap<CanisterId, TokenInfo> {
        self.token_pairs
            .query_dummy_tokens(self.business_tokens_query())
    }
    fn business_all_tokens_query(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        self.business_tokens_query()
            .iter()
            .map(|(token, info)| (*token, Cow::Borrowed(info)))
            .chain(
                self.business_dummy_tokens_query()
                    .into_iter()
                    .map(|(token, info)| (token, Cow::Owned(info))),
            )
            .collect()
    }
    fn business_token_query(&self, token: &CanisterId) -> Option<TokenInfo> {
        self.business_tokens_query()
            .get(token)
            .cloned()
            .or_else(|| self.business_dummy_tokens_query().remove(token))
    }
    fn business_token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        ic_canister_kit::common::trap_debug(self.token_balances.token_balance_of(token, account))
    }

    // ======================== update ========================

    fn business_token_deposit(
        &mut self,
        locks: &(TokenBalancesLock, TokenBlockChainLock),
        arg: ArgWithMeta<DepositToken>,
        height: Nat,
    ) -> Result<Nat, BusinessError> {
        let mut guard = self.get_token_guard(
            locks,
            arg.clone(),
            Some(format!(
                "Deposit {} Token: [{}] From: ({}) With Height: {height}.",
                arg.arg.amount,
                arg.arg.token.to_text(),
                display_account(&arg.arg.from)
            )),
        )?;
        let height = guard.token_deposit(arg, height)?; // do deposit
        self.business_certified_data_refresh(); // set certified data
        Ok(height)
    }
    fn business_token_withdraw(
        &mut self,
        locks: &(TokenBalancesLock, TokenBlockChainLock),
        arg: ArgWithMeta<WithdrawToken>,
        height: Nat,
    ) -> Result<Nat, BusinessError> {
        let mut guard = self.get_token_guard(
            locks,
            arg.clone(),
            Some(format!(
                "Withdraw {} Token: [{}] To: ({}) With Height: {height}.",
                arg.arg.amount,
                arg.arg.token.to_text(),
                display_account(&arg.arg.to)
            )),
        )?;
        let height = guard.token_withdraw(arg, height)?; // do withdraw
        self.business_certified_data_refresh(); // set certified data
        Ok(height)
    }
    fn business_token_transfer(
        &mut self,
        locks: &(TokenBalancesLock, TokenBlockChainLock),
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        let mut guard = self.get_token_guard(locks, arg.clone(), None)?;
        let changed = guard.token_transfer(arg)?; // do transfer
        self.business_certified_data_refresh(); // set certified data
        Ok(changed)
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
        let token0 = TOKENS
            .get(&arg.arg.pair.token0)
            .ok_or(BusinessError::NotSupportedToken(arg.arg.pair.token0))?;
        let token1 = TOKENS
            .get(&arg.arg.pair.token1)
            .ok_or(BusinessError::NotSupportedToken(arg.arg.pair.token1))?;

        let mut swap_guard = self.swap_block_chain.be_guard(lock);
        let mut trace_guard = self.request_traces.be_guard(
            arg.clone().into(),
            None,
            None,
            Some(&swap_guard),
            None,
        )?;
        let maker = self.token_pairs.create_token_pair_pool(
            &mut swap_guard,
            &mut trace_guard,
            arg,
            token0,
            token1,
        )?;
        self.business_certified_data_refresh(); // set certified data
        Ok(maker)
    }
    // // liquidity
    // fn business_token_pair_liquidity_add(
    //     &mut self,
    //     locks: &(TokenBalancesLock, TokenBlockChainLock, SwapBlockChainLock),
    //     arg: ArgWithMeta<TokenPairLiquidityAddArg>,
    // ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
    //     let balances_guard = self.token_balances.be_guard(&locks.0);
    //     let token_guard = self.token_block_chain.be_guard(&locks.1);
    //     let swap_guard = self.swap_block_chain.be_guard(&locks.2);
    //     let mut guard = TokenPairGuard::new(balances_guard, token_guard, swap_guard);
    //     let success = self.business_data.token_pairs.add_liquidity(
    //         &mut guard,
    //         self.business_data.fee_to,
    //         arg,
    //     )?;
    //     self.business_certified_data_refresh(); // set certified data
    //     Ok(success)
    // }
    // fn business_token_pair_check_liquidity_removable(
    //     &self,
    //     pa: &TokenPairAmm,
    //     from: &Account,
    //     liquidity: &Nat,
    // ) -> Result<(), BusinessError> {
    //     self.business_data.token_pairs.check_liquidity_removable(
    //         &self.token_balances,
    //         pa,
    //         from,
    //         liquidity,
    //     )
    // }
    // fn business_token_pair_liquidity_remove(
    //     &mut self,
    //     balance_lock: &TokenBalancesLock,
    //     self_canister: &SelfCanister,
    //     pa: TokenPairAmm,
    //     arg: TokenPairLiquidityRemoveArg,
    // ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
    //     let mut guard = self.token_balances.be_guard(balance_lock);
    //     self.business_data.token_pairs.remove_liquidity(
    //         self.business_data.fee_to,
    //         &mut guard,
    //         self_canister,
    //         pa,
    //         arg,
    //     )
    // }

    // // pair swap
    // fn business_token_pair_swap_exact_tokens_for_tokens(
    //     &mut self,
    //     balance_lock: &TokenBalancesLock,
    //     self_canister: &SelfCanister,
    //     args: TokenPairSwapExactTokensForTokensArgs,
    //     pas: Vec<TokenPairAmm>,
    // ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    //     let mut guard = self.token_balances.be_guard(balance_lock);
    //     self.business_data.token_pairs.swap_exact_tokens_for_tokens(
    //         &mut guard,
    //         self_canister,
    //         args,
    //         pas,
    //     )
    // }
    // fn business_token_pair_swap_tokens_for_exact_tokens(
    //     &mut self,
    //     balance_lock: &TokenBalancesLock,
    //     self_canister: &SelfCanister,
    //     args: TokenPairSwapTokensForExactTokensArgs,
    //     pas: Vec<TokenPairAmm>,
    // ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    //     let mut guard = self.token_balances.be_guard(balance_lock);
    //     self.business_data.token_pairs.swap_tokens_for_exact_tokens(
    //         &mut guard,
    //         self_canister,
    //         args,
    //         pas,
    //     )
    // }
    // fn business_token_pair_swap_by_loan(
    //     &mut self,
    //     balance_lock: &TokenBalancesLock,
    //     self_canister: &SelfCanister,
    //     args: TokenPairSwapByLoanArgs,
    //     pas: Vec<TokenPairAmm>,
    // ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
    //     let mut guard = self.token_balances.be_guard(balance_lock);
    //     self.business_data
    //         .token_pairs
    //         .swap_by_loan(&mut guard, self_canister, args, pas)
    // }

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
        self.request_traces.remove_request_trace(index)
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
