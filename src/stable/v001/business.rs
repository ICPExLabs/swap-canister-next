use super::super::business::*;
use super::types::*;

impl Business for InnerState {
    // tokens
    fn business_tokens_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        &TOKENS
    }
    fn business_token_balance_of(&self, canister_id: CanisterId, account: Account) -> candid::Nat {
        self.token_balances.token_balance_of(canister_id, account)
    }

    // token balance lock
    fn business_token_balance_lock<'a>(
        &mut self,
        token_accounts: &'a [TokenAccount],
    ) -> Result<TokenBalanceLockGuard<'a>, Vec<TokenAccount>> {
        self.business_data.token_balance_locks.lock(token_accounts)
    }
    fn business_token_balance_unlock(&mut self, token_accounts: &[TokenAccount]) {
        self.business_data
            .token_balance_locks
            .unlock(token_accounts)
    }

    // token deposit and withdraw
    fn business_token_deposit(&mut self, canister_id: CanisterId, account: Account, amount: Nat) {
        self.token_balances
            .token_deposit(canister_id, account, amount)
    }
    fn business_token_withdraw(&mut self, canister_id: CanisterId, account: Account, amount: Nat) {
        self.token_balances
            .token_withdraw(canister_id, account, amount)
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
        self_canister: SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        self.business_data.token_pairs.add_liquidity(
            self.business_data.fee_to,
            &mut self.token_balances,
            self_canister,
            pa,
            arg,
        )
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
    #[allow(clippy::unwrap_used)] // ? SAFETY
    fn business_example_cell_update(&mut self, test: String) {
        let mut cell = self.example_cell.get().to_owned();
        cell.cell_data = test;
        self.example_cell.set(cell).unwrap();
    }

    fn business_example_vec_query(&self) -> Vec<ExampleVec> {
        self.example_vec.iter().collect()
    }
    #[allow(clippy::unwrap_used)] // ? SAFETY
    fn business_example_vec_push(&mut self, test: u64) {
        self.example_vec
            .push(&ExampleVec { vec_data: test })
            .unwrap()
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
    #[allow(clippy::unwrap_used)] // ? SAFETY
    fn business_example_log_update(&mut self, item: String) -> u64 {
        self.example_log.append(&item).unwrap()
    }

    fn business_example_priority_queue_query(&self) -> Vec<ExampleVec> {
        self.example_priority_queue.iter().collect()
    }
    #[allow(clippy::unwrap_used)] // ? SAFETY
    fn business_example_priority_queue_push(&mut self, item: u64) {
        self.example_priority_queue
            .push(&ExampleVec { vec_data: item })
            .unwrap();
    }
    fn business_example_priority_queue_pop(&mut self) -> Option<ExampleVec> {
        self.example_priority_queue.pop()
    }
}
