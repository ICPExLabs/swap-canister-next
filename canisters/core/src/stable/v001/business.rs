use super::super::business::*;
use super::types::*;

impl Business for InnerState {
    // config
    fn business_config_fee_to_query(&self) -> Option<&Account> {
        self.business_data.fee_to.as_ref()
    }
    fn business_config_fee_to_replace(&mut self, fee_to: Option<Account>) -> Option<Account> {
        std::mem::replace(&mut self.business_data.fee_to, fee_to)
    }

    // tokens query
    fn business_tokens_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        &TOKENS
    }
    fn business_dummy_tokens_query(&self) -> HashMap<CanisterId, TokenInfo> {
        self.business_data
            .token_pairs
            .business_dummy_tokens_query(self.business_tokens_query())
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
    fn business_token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        ic_canister_kit::common::trap_debug(self.token_balances.token_balance_of(token, account))
    }

    // token balance lock
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

    // token block chain lock
    fn business_token_block_chain_lock(&mut self) -> Option<TokenBlockChainLock> {
        self.token_block_chain.lock()
    }
    fn business_token_block_chain_unlock(&mut self) {
        self.token_block_chain.unlock()
    }

    // token deposit and withdraw and transfer
    fn business_token_deposit(
        &mut self,
        locks: &(TokenBalancesLock, TokenBlockChainLock),
        arg: ArgWithMeta<DepositToken>,
    ) -> Result<(), BusinessError> {
        let mut balance_guard = self.token_balances.be_guard(&locks.0);
        let mut token_guard = self.token_block_chain.be_guard(&locks.1);
        // 1. get token block
        let transaction = TokenTransaction {
            operation: TokenOperation::Deposit(arg.arg.clone()),
            memo: arg.memo,
            created: arg.created,
        };
        let (encoded_block, hash) = token_guard.push_token_transaction(arg.now, transaction)?;
        // 2. do deposit
        balance_guard.token_deposit(arg.arg.token, arg.arg.from, arg.arg.amount)?;
        // 3. push block
        token_guard.push_block(encoded_block, hash);
        // 4. set certified data
        ic_cdk::api::set_certified_data(hash.as_slice()); // TODO with swap hash?
        Ok(())
    }
    fn business_token_withdraw(
        &mut self,
        locks: &(TokenBalancesLock, TokenBlockChainLock),
        arg: ArgWithMeta<WithdrawToken>,
    ) -> Result<(), BusinessError> {
        let mut balance_guard = self.token_balances.be_guard(&locks.0);
        let mut token_guard = self.token_block_chain.be_guard(&locks.1);
        // 1. get token block
        let transaction = TokenTransaction {
            operation: TokenOperation::Withdraw(arg.arg.clone()),
            memo: arg.memo,
            created: arg.created,
        };
        let (encoded_block, hash) = token_guard.push_token_transaction(arg.now, transaction)?;
        // 2. do deposit
        balance_guard.token_withdraw(arg.arg.token, arg.arg.from, arg.arg.amount)?;
        // 3. push block
        token_guard.push_block(encoded_block, hash);
        // 4. set certified data
        ic_cdk::api::set_certified_data(hash.as_slice()); // TODO with swap hash?
        Ok(())
    }
    fn business_token_transfer(
        &mut self,
        balance_lock: &TokenBalancesLock,
        token: CanisterId,
        from: Account,
        to: Account,
        amount_without_fee: Nat,
        fee: Nat,
    ) -> Result<Nat, BusinessError> {
        let mut guard = self.token_balances.be_guard(balance_lock);
        match guard
            .fee_to()
            .iter()
            .find(|&fee_to| fee_to.token == token)
            .cloned()
        {
            Some(fee_to) if *crate::utils::math::ZERO < fee => {
                assert_eq!(token, fee_to.token, "wrong fee_to token");

                let amount = amount_without_fee.clone() + fee.clone();

                // withdraw
                guard.token_withdraw(token, from, amount.clone())?;

                // deposit
                guard.token_deposit(token, to, amount_without_fee)?;
                guard.token_deposit(token, fee_to.account, fee)?;

                // return changed amount
                Ok(amount)
            }
            _ => {
                // transfer
                guard.token_transfer(token, from, to, amount_without_fee.clone())?;

                // return changed amount
                Ok(amount_without_fee)
            }
        }
    }

    // pair
    fn business_token_pair_pools_query(&self) -> Vec<(&TokenPair, &Amm, &MarketMaker)> {
        self.business_data.token_pairs.query_token_pair_pools()
    }
    fn business_token_pair_pool_maker_get(&self, pa: &PairAmm) -> Option<&MarketMaker> {
        self.business_data.token_pairs.get_token_pair_pool_maker(pa)
    }
    fn business_token_pair_pool_create(&mut self, pa: PairAmm) -> Result<(), BusinessError> {
        let token0 = TOKENS
            .get(&pa.pair.token0)
            .ok_or(BusinessError::NotSupportedToken(pa.pair.token0))?;
        let token1 = TOKENS
            .get(&pa.pair.token1)
            .ok_or(BusinessError::NotSupportedToken(pa.pair.token1))?;

        let (subaccount, dummy_canister_id) = pa.get_subaccount_and_dummy_canister_id();

        self.business_data.token_pairs.create_token_pair_pool(
            pa,
            subaccount,
            dummy_canister_id,
            token0,
            token1,
        )
    }

    // pair liquidity
    fn business_token_pair_liquidity_add(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        let mut guard = self.token_balances.be_guard(balance_lock);
        self.business_data.token_pairs.add_liquidity(
            self.business_data.fee_to,
            &mut guard,
            self_canister,
            pa,
            arg,
        )
    }
    fn business_token_pair_check_liquidity_removable(
        &self,
        pa: &PairAmm,
        from: &Account,
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        self.business_data.token_pairs.check_liquidity_removable(
            &self.token_balances,
            pa,
            from,
            liquidity,
        )
    }
    fn business_token_pair_liquidity_remove(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        let mut guard = self.token_balances.be_guard(balance_lock);
        self.business_data.token_pairs.remove_liquidity(
            self.business_data.fee_to,
            &mut guard,
            self_canister,
            pa,
            arg,
        )
    }

    // pair swap
    fn business_token_pair_swap_exact_tokens_for_tokens(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        args: TokenPairSwapExactTokensForTokensArgs,
        pas: Vec<PairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        let mut guard = self.token_balances.be_guard(balance_lock);
        self.business_data.token_pairs.swap_exact_tokens_for_tokens(
            &mut guard,
            self_canister,
            args,
            pas,
        )
    }
    fn business_token_pair_swap_tokens_for_exact_tokens(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        args: TokenPairSwapTokensForExactTokensArgs,
        pas: Vec<PairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        let mut guard = self.token_balances.be_guard(balance_lock);
        self.business_data.token_pairs.swap_tokens_for_exact_tokens(
            &mut guard,
            self_canister,
            args,
            pas,
        )
    }
    fn business_token_pair_swap_by_loan(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        args: TokenPairSwapByLoanArgs,
        pas: Vec<PairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        let mut guard = self.token_balances.be_guard(balance_lock);
        self.business_data
            .token_pairs
            .swap_by_loan(&mut guard, self_canister, args, pas)
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
