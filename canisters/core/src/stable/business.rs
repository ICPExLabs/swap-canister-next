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
    // config
    fn business_config_fee_to_query(&self) -> Option<&Account> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_fee_to_replace(&mut self, fee_to: Option<Account>) -> Option<Account> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // tokens query
    fn business_tokens_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_dummy_tokens_query(&self) -> HashMap<CanisterId, TokenInfo> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_all_tokens_query(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // token balance lock
    fn business_token_balance_lock(
        &mut self,
        fee_to: Vec<CanisterId>,
        required: Vec<TokenAccount>,
    ) -> Result<TokenBalancesLock, Vec<TokenAccount>> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_balance_unlock(&mut self, locked: &HashSet<TokenAccount>) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // token block chain lock
    fn business_token_block_chain_lock(&mut self) -> Option<TokenBlockChainLock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_block_chain_unlock(&mut self) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // token deposit and withdraw and transfer
    fn business_token_deposit(
        &mut self,
        locks: &(TokenBalancesLock, TokenBlockChainLock),
        arg: ArgWithMeta<DepositToken>,
    ) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_withdraw(
        &mut self,
        locks: &(TokenBalancesLock, TokenBlockChainLock),
        arg: ArgWithMeta<WithdrawToken>,
    ) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
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
        ic_cdk::trap("Not supported operation by this version.")
    }

    // pair
    fn business_token_pair_pools_query(&self) -> Vec<(&TokenPair, &Amm, &MarketMaker)> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_pool_maker_get(&self, pa: &PairAmm) -> Option<&MarketMaker> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_pool_create(&mut self, pa: PairAmm) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // pair liquidity
    fn business_token_pair_liquidity_add(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_check_liquidity_removable(
        &self,
        pa: &PairAmm,
        from: &Account,
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_liquidity_remove(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // pair swap
    fn business_token_pair_swap_exact_tokens_for_tokens(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        args: TokenPairSwapExactTokensForTokensArgs,
        pas: Vec<PairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_swap_tokens_for_exact_tokens(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        args: TokenPairSwapTokensForExactTokensArgs,
        pas: Vec<PairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_swap_by_loan(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        args: TokenPairSwapByLoanArgs,
        pas: Vec<PairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
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
    // config
    fn business_config_fee_to_query(&self) -> Option<&Account> {
        self.get().business_config_fee_to_query()
    }
    fn business_config_fee_to_replace(&mut self, fee_to: Option<Account>) -> Option<Account> {
        self.get_mut().business_config_fee_to_replace(fee_to)
    }

    // tokens query
    fn business_tokens_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        self.get().business_tokens_query()
    }
    fn business_dummy_tokens_query(&self) -> HashMap<CanisterId, TokenInfo> {
        self.get().business_dummy_tokens_query()
    }
    fn business_all_tokens_query(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        self.get().business_all_tokens_query()
    }
    fn business_token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        self.get().business_token_balance_of(token, account)
    }

    // token balance lock
    fn business_token_balance_lock(
        &mut self,
        fee_to: Vec<CanisterId>,
        required: Vec<TokenAccount>,
    ) -> Result<TokenBalancesLock, Vec<TokenAccount>> {
        self.get_mut().business_token_balance_lock(fee_to, required)
    }
    fn business_token_balance_unlock(&mut self, locked: &HashSet<TokenAccount>) {
        self.get_mut().business_token_balance_unlock(locked)
    }

    // token block chain lock
    fn business_token_block_chain_lock(&mut self) -> Option<TokenBlockChainLock> {
        self.get_mut().business_token_block_chain_lock()
    }
    fn business_token_block_chain_unlock(&mut self) {
        self.get_mut().business_token_block_chain_unlock()
    }

    // token deposit and withdraw and transfer
    fn business_token_deposit(
        &mut self,
        locks: &(TokenBalancesLock, TokenBlockChainLock),
        arg: ArgWithMeta<DepositToken>,
    ) -> Result<(), BusinessError> {
        self.get_mut().business_token_deposit(locks, arg)
    }
    fn business_token_withdraw(
        &mut self,
        locks: &(TokenBalancesLock, TokenBlockChainLock),
        arg: ArgWithMeta<WithdrawToken>,
    ) -> Result<(), BusinessError> {
        self.get_mut().business_token_withdraw(locks, arg)
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
        self.get_mut().business_token_transfer(
            balance_lock,
            token,
            from,
            to,
            amount_without_fee,
            fee,
        )
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
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        self.get_mut()
            .business_token_pair_liquidity_add(balance_lock, self_canister, pa, arg)
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
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        self.get_mut()
            .business_token_pair_liquidity_remove(balance_lock, self_canister, pa, arg)
    }

    // pair swap
    fn business_token_pair_swap_exact_tokens_for_tokens(
        &mut self,
        balance_lock: &TokenBalancesLock,
        self_canister: &SelfCanister,
        args: TokenPairSwapExactTokensForTokensArgs,
        pas: Vec<PairAmm>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.get_mut()
            .business_token_pair_swap_exact_tokens_for_tokens(
                balance_lock,
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
        self.get_mut()
            .business_token_pair_swap_tokens_for_exact_tokens(
                balance_lock,
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
        self.get_mut()
            .business_token_pair_swap_by_loan(balance_lock, self_canister, args, pas)
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
