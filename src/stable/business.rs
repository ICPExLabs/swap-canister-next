use super::*;
#[allow(unused)]
pub use ic_canister_kit::identity::self_canister_id;
#[allow(unused)]
pub use ic_canister_kit::types::{CanisterId, PauseReason, UserId};
#[allow(unused)]
pub use std::collections::{HashMap, HashSet};
#[allow(unused)]
pub use std::fmt::Display;

#[allow(clippy::panic)] // ? SAFETY
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
    // tokens
    fn business_tokens_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        panic!("Not supported operation by this version.")
    }
    fn business_token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        panic!("Not supported operation by this version.")
    }
    fn business_dummy_tokens_query(&self) -> HashMap<CanisterId, TokenInfo> {
        panic!("Not supported operation by this version.")
    }

    // token balance lock
    fn business_token_balance_lock<'a>(
        &mut self,
        token_accounts: &'a [TokenAccount],
    ) -> Result<TokenBalanceLockGuard<'a>, Vec<TokenAccount>> {
        panic!("Not supported operation by this version.")
    }
    fn business_token_balance_unlock(&mut self, token_accounts: &[TokenAccount]) {
        panic!("Not supported operation by this version.")
    }

    // token deposit and withdraw
    fn business_token_deposit(&mut self, token: CanisterId, account: Account, amount: Nat) {
        panic!("Not supported operation by this version.")
    }
    fn business_token_withdraw(&mut self, token: CanisterId, account: Account, amount: Nat) {
        panic!("Not supported operation by this version.")
    }

    // pair
    fn business_token_pair_pools_query(&self) -> Vec<(&TokenPair, &Amm, &MarketMaker)> {
        panic!("Not supported operation by this version.")
    }
    fn business_token_pair_pool_maker_get(&self, pa: &PairAmm) -> Option<&MarketMaker> {
        panic!("Not supported operation by this version.")
    }
    fn business_token_pair_pool_create(&mut self, pa: PairAmm) -> Result<(), BusinessError> {
        panic!("Not supported operation by this version.")
    }

    // pair liquidity
    fn business_token_pair_liquidity_add(
        &mut self,
        self_canister: SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        panic!("Not supported operation by this version.")
    }
    fn business_token_pair_check_liquidity_removable(
        &self,
        pa: &PairAmm,
        from: &Account,
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        panic!("Not supported operation by this version.")
    }
    fn business_token_pair_liquidity_remove(
        &mut self,
        self_canister: SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        panic!("Not supported operation by this version.")
    }

    fn business_example_query(&self) -> String {
        panic!("Not supported operation by this version.")
    }
    fn business_example_update(&mut self, test: String) {
        panic!("Not supported operation by this version.")
    }

    fn business_example_cell_query(&self) -> crate::stable::ExampleCell {
        panic!("Not supported operation by this version.")
    }
    fn business_example_cell_update(&mut self, test: String) {
        panic!("Not supported operation by this version.")
    }

    fn business_example_vec_query(&self) -> Vec<crate::stable::ExampleVec> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_vec_push(&mut self, test: u64) {
        panic!("Not supported operation by this version.")
    }

    fn business_example_vec_pop(&mut self) -> Option<crate::stable::ExampleVec> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_map_query(&self) -> HashMap<u64, String> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_map_update(&mut self, key: u64, value: Option<String>) -> Option<String> {
        panic!("Not supported operation by this version.")
    }

    fn business_example_log_query(&self) -> Vec<String> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_log_update(&mut self, item: String) -> u64 {
        panic!("Not supported operation by this version.")
    }

    fn business_example_priority_queue_query(&self) -> Vec<crate::stable::ExampleVec> {
        panic!("Not supported operation by this version.")
    }
    fn business_example_priority_queue_push(&mut self, item: u64) {
        panic!("Not supported operation by this version.")
    }
    fn business_example_priority_queue_pop(&mut self) -> Option<crate::stable::ExampleVec> {
        panic!("Not supported operation by this version.")
    }
}

// 业务实现
impl Business for State {
    // tokens
    fn business_tokens_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        self.get().business_tokens_query()
    }
    fn business_token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        self.get().business_token_balance_of(token, account)
    }
    fn business_dummy_tokens_query(&self) -> HashMap<CanisterId, TokenInfo> {
        self.get().business_dummy_tokens_query()
    }

    // token balance lock
    fn business_token_balance_lock<'a>(
        &mut self,
        token_accounts: &'a [TokenAccount],
    ) -> Result<TokenBalanceLockGuard<'a>, Vec<TokenAccount>> {
        self.get_mut().business_token_balance_lock(token_accounts)
    }
    fn business_token_balance_unlock(&mut self, token_accounts: &[TokenAccount]) {
        self.get_mut().business_token_balance_unlock(token_accounts)
    }

    // token deposit and withdraw
    fn business_token_deposit(&mut self, token: CanisterId, account: Account, amount: Nat) {
        self.get_mut()
            .business_token_deposit(token, account, amount)
    }
    fn business_token_withdraw(&mut self, token: CanisterId, account: Account, amount: Nat) {
        self.get_mut()
            .business_token_withdraw(token, account, amount)
    }

    // pair
    fn business_token_pair_pools_query(&self) -> Vec<(&TokenPair, &Amm, &MarketMaker)> {
        self.get().business_token_pair_pools_query()
    }
    fn business_token_pair_pool_maker_get(&self, pa: &PairAmm) -> Option<&MarketMaker> {
        self.get().business_token_pair_pool_maker_get(pa)
    }
    fn business_token_pair_pool_create(&mut self, pa: PairAmm) -> Result<(), BusinessError> {
        self.get_mut().business_token_pair_pool_create(pa)
    }

    // pair liquidity
    fn business_token_pair_liquidity_add(
        &mut self,
        self_canister: SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        self.get_mut()
            .business_token_pair_liquidity_add(self_canister, pa, arg)
    }
    fn business_token_pair_check_liquidity_removable(
        &self,
        pa: &PairAmm,
        from: &Account,
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        self.get()
            .business_token_pair_check_liquidity_removable(pa, from, liquidity)
    }
    fn business_token_pair_liquidity_remove(
        &mut self,
        self_canister: SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        self.get_mut()
            .business_token_pair_liquidity_remove(self_canister, pa, arg)
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
