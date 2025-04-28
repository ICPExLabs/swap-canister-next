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
    Pausable<PauseReason> + ParsePermission + Permissable<Permission> + Schedulable + ScheduleTask + StableHeap
{
    fn business_updated(&self) -> u64 {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // ======================== config ========================

    // fee to
    fn business_config_fee_to_query(&self) -> FeeTo {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_fee_to_replace(&mut self, fee_to: FeeTo) -> FeeTo {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // archive canister
    // token
    fn business_config_token_block_chain_query(&self) -> &BlockChain<TokenBlock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_archive_wasm_module_query(&self) -> &Option<Vec<u8>> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_archive_wasm_module_replace(
        &mut self,
        wasm_module: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_current_archiving_max_length_replace(
        &mut self,
        max_length: u64,
    ) -> Option<CurrentArchiving> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_archive_config_replace(
        &mut self,
        archive_config: NextArchiveCanisterConfig,
    ) -> NextArchiveCanisterConfig {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_current_archiving_replace(
        &mut self,
        archiving: CurrentArchiving,
    ) -> Option<CurrentArchiving> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_archive_current_canister(&mut self) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_parent_hash_get(&self, block_height: BlockIndex) -> Option<HashOf<TokenBlock>> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_cached_block_get(&self) -> Option<(BlockIndex, u64)> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_block_archived(&mut self, block_height: BlockIndex) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // swap
    fn business_config_swap_block_chain_query(&self) -> &BlockChain<SwapBlock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_swap_archive_wasm_module_query(&self) -> &Option<Vec<u8>> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_swap_archive_wasm_module_replace(
        &mut self,
        wasm_module: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_swap_current_archiving_max_length_replace(
        &mut self,
        max_length: u64,
    ) -> Option<CurrentArchiving> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_swap_archive_config_replace(
        &mut self,
        archive_config: NextArchiveCanisterConfig,
    ) -> NextArchiveCanisterConfig {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_swap_current_archiving_replace(
        &mut self,
        archiving: CurrentArchiving,
    ) -> Option<CurrentArchiving> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_swap_archive_current_canister(&mut self) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_swap_parent_hash_get(&self, block_height: BlockIndex) -> Option<HashOf<SwapBlock>> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_swap_cached_block_get(&self) -> Option<(BlockIndex, u64)> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_swap_block_archived(&mut self, block_height: BlockIndex) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // maintain archives
    fn business_config_maintain_archives_query(&self) -> &MaintainArchives {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_maintain_archives_set(&mut self, config: MaintainArchivesConfig) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_maintain_trigger(&mut self, now: TimestampNanos) -> bool {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_maintain_canisters(&self) -> Vec<CanisterId> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_maintain_archives_cycles_recharged(&mut self, canister_id: CanisterId, cycles: u128) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // token frozen
    fn business_config_token_frozen_query(&self) -> &HashSet<CanisterId> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_frozen(&mut self, arg: ArgWithMeta<TokenFrozenArg>) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // token custom
    fn business_config_token_preset_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_custom_query(&self) -> Vec<TokenInfo> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_custom_put(&mut self, arg: ArgWithMeta<TokenInfo>) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_config_token_custom_remove(&mut self, arg: ArgWithMeta<CanisterId>) -> Option<TokenInfo> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // set_certified_data
    fn business_certified_data_refresh(&self) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // ======================== locks ========================

    // token block chain
    fn business_token_block_chain_archive_lock(&mut self) -> Option<TokenBlockChainArchiveLock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_block_chain_archive_unlock(&mut self) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_block_chain_lock(&mut self) -> Option<TokenBlockChainLock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_block_chain_unlock(&mut self) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // swap block chain
    fn business_swap_block_chain_archive_lock(&mut self) -> Option<SwapBlockChainArchiveLock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_swap_block_chain_archive_unlock(&mut self) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_swap_block_chain_lock(&mut self) -> Option<SwapBlockChainLock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_swap_block_chain_unlock(&mut self) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // token balances
    fn business_token_balance_lock(
        &mut self,
        required: Vec<TokenAccount>,
    ) -> Result<TokenBalancesLock, Vec<TokenAccount>> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_balance_unlock(&mut self, locked: &HashSet<TokenAccount>) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // ======================== token block chain ========================

    // ======================== query ========================

    fn business_token_alive(&self, canister_id: &CanisterId) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_tokens_query(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_dummy_tokens_query(&self) -> HashMap<CanisterId, TokenInfo> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_all_tokens_with_dummy_query(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_query(&self, token: &CanisterId) -> Option<TokenInfo> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_query_by_pa(&self, pa: &TokenPairAmm) -> Option<TokenInfo> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_balance_of_with_fee_to(
        &self,
        token: CanisterId,
        account: Account,
    ) -> (candid::Nat, Option<Account>) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // ======================== update ========================

    fn business_token_deposit(
        &mut self,
        locks: &(TokenBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<DepositToken>,
        height: Nat,
    ) -> Result<Nat, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_withdraw(
        &mut self,
        locks: &(TokenBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<WithdrawToken>,
        height: Nat,
    ) -> Result<Nat, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_transfer(
        &mut self,
        locks: &(TokenBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_transfer_lp(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // ======================== swap block chain ========================

    // ======================== token pair swap ========================

    // query
    fn business_token_pair_pools_query(&self) -> Vec<(TokenPairAmm, MarketMaker)> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_pool_get(&self, pa: &TokenPairAmm) -> Option<MarketMaker> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    // create and remove
    fn business_token_pair_pool_create(
        &mut self,
        lock: &SwapBlockChainLock,
        arg: ArgWithMeta<TokenPairAmm>,
    ) -> Result<MarketMaker, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_pool_remove(
        &mut self,
        lock: &SwapBlockChainLock,
        arg: ArgWithMeta<TokenPairAmm>,
    ) -> Result<MarketMaker, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    // liquidity
    fn business_token_pair_liquidity_add(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairLiquidityAddArg>,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_check_liquidity_removable(
        &self,
        pa: &TokenPairAmm,
        from: &Account,
        liquidity_without_fee: &Nat,
    ) -> Result<(), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_liquidity_remove(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairLiquidityRemoveArg>,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // pair swap
    fn business_token_pair_swap_fixed_in_checking(
        &self,
        arg: &TokenPairSwapExactTokensForTokensArg,
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_swap_fixed_out_checking(
        &self,
        arg: &TokenPairSwapTokensForExactTokensArg,
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_swap_exact_tokens_for_tokens(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairSwapExactTokensForTokensArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_swap_tokens_for_exact_tokens(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairSwapTokensForExactTokensArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_token_pair_swap_by_loan(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairSwapByLoanArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // ======================== blocks query ========================

    fn business_token_queryable(&self, caller: &UserId) -> Result<(), String> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_swap_queryable(&self, caller: &UserId) -> Result<(), String> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_token_block_get(&self, block_height: BlockIndex) -> QueryBlockResult<EncodedBlock> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_swap_block_get(&self, block_height: BlockIndex) -> QueryBlockResult<EncodedBlock> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    fn business_token_blocks_get(&self, block_height: BlockIndex) -> Vec<(BlockIndex, QueryBlockResult<EncodedBlock>)> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_swap_blocks_get(&self, block_height: BlockIndex) -> Vec<(BlockIndex, QueryBlockResult<EncodedBlock>)> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // ======================== request ========================

    fn business_request_index_get(&self) -> (RequestIndex, u64) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_request_trace_get(&self, index: &RequestIndex) -> Option<RequestTrace> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_request_trace_remove(&mut self, index: &RequestIndex) -> Option<RequestTrace> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_request_trace_insert(&mut self, trace: RequestTrace) {
        ic_cdk::trap("Not supported operation by this version.")
    }
}

// Business
impl Business for State {
    fn business_updated(&self) -> u64 {
        self.get().business_updated()
    }

    // ======================== config ========================

    // fee to
    fn business_config_fee_to_query(&self) -> FeeTo {
        self.get().business_config_fee_to_query()
    }
    fn business_config_fee_to_replace(&mut self, fee_to: FeeTo) -> FeeTo {
        self.get_mut().business_config_fee_to_replace(fee_to)
    }

    // archive canister
    // token
    fn business_config_token_block_chain_query(&self) -> &BlockChain<TokenBlock> {
        self.get().business_config_token_block_chain_query()
    }
    fn business_config_token_archive_wasm_module_query(&self) -> &Option<Vec<u8>> {
        self.get().business_config_token_archive_wasm_module_query()
    }
    fn business_config_token_archive_wasm_module_replace(
        &mut self,
        wasm_module: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, BusinessError> {
        self.get_mut()
            .business_config_token_archive_wasm_module_replace(wasm_module)
    }
    fn business_config_token_current_archiving_max_length_replace(
        &mut self,
        max_length: u64,
    ) -> Option<CurrentArchiving> {
        self.get_mut()
            .business_config_token_current_archiving_max_length_replace(max_length)
    }
    fn business_config_token_archive_config_replace(
        &mut self,
        archive_config: NextArchiveCanisterConfig,
    ) -> NextArchiveCanisterConfig {
        self.get_mut()
            .business_config_token_archive_config_replace(archive_config)
    }
    fn business_config_token_current_archiving_replace(
        &mut self,
        archiving: CurrentArchiving,
    ) -> Option<CurrentArchiving> {
        self.get_mut()
            .business_config_token_current_archiving_replace(archiving)
    }
    fn business_config_token_archive_current_canister(&mut self) -> Result<(), BusinessError> {
        self.get_mut().business_config_token_archive_current_canister()
    }
    fn business_config_token_parent_hash_get(&self, block_height: BlockIndex) -> Option<HashOf<TokenBlock>> {
        self.get().business_config_token_parent_hash_get(block_height)
    }
    fn business_config_token_cached_block_get(&self) -> Option<(BlockIndex, u64)> {
        self.get().business_config_token_cached_block_get()
    }
    fn business_config_token_block_archived(&mut self, block_height: BlockIndex) -> Result<(), BusinessError> {
        self.get_mut().business_config_token_block_archived(block_height)
    }

    // swap
    fn business_config_swap_block_chain_query(&self) -> &BlockChain<SwapBlock> {
        self.get().business_config_swap_block_chain_query()
    }
    fn business_config_swap_archive_wasm_module_query(&self) -> &Option<Vec<u8>> {
        self.get().business_config_swap_archive_wasm_module_query()
    }
    fn business_config_swap_archive_wasm_module_replace(
        &mut self,
        wasm_module: Vec<u8>,
    ) -> Result<Option<Vec<u8>>, BusinessError> {
        self.get_mut()
            .business_config_swap_archive_wasm_module_replace(wasm_module)
    }
    fn business_config_swap_current_archiving_max_length_replace(
        &mut self,
        max_length: u64,
    ) -> Option<CurrentArchiving> {
        self.get_mut()
            .business_config_swap_current_archiving_max_length_replace(max_length)
    }
    fn business_config_swap_archive_config_replace(
        &mut self,
        archive_config: NextArchiveCanisterConfig,
    ) -> NextArchiveCanisterConfig {
        self.get_mut()
            .business_config_swap_archive_config_replace(archive_config)
    }
    fn business_config_swap_current_archiving_replace(
        &mut self,
        archiving: CurrentArchiving,
    ) -> Option<CurrentArchiving> {
        self.get_mut().business_config_swap_current_archiving_replace(archiving)
    }
    fn business_config_swap_archive_current_canister(&mut self) -> Result<(), BusinessError> {
        self.get_mut().business_config_swap_archive_current_canister()
    }
    fn business_config_swap_parent_hash_get(&self, block_height: BlockIndex) -> Option<HashOf<SwapBlock>> {
        self.get().business_config_swap_parent_hash_get(block_height)
    }
    fn business_config_swap_cached_block_get(&self) -> Option<(BlockIndex, u64)> {
        self.get().business_config_swap_cached_block_get()
    }
    fn business_config_swap_block_archived(&mut self, block_height: BlockIndex) -> Result<(), BusinessError> {
        self.get_mut().business_config_swap_block_archived(block_height)
    }

    // maintain archives
    fn business_config_maintain_archives_query(&self) -> &MaintainArchives {
        self.get().business_config_maintain_archives_query()
    }
    fn business_config_maintain_archives_set(&mut self, config: MaintainArchivesConfig) {
        self.get_mut().business_config_maintain_archives_set(config)
    }
    fn business_config_maintain_trigger(&mut self, now: TimestampNanos) -> bool {
        self.get_mut().business_config_maintain_trigger(now)
    }
    fn business_config_maintain_canisters(&self) -> Vec<CanisterId> {
        self.get().business_config_maintain_canisters()
    }
    fn business_config_maintain_archives_cycles_recharged(&mut self, canister_id: CanisterId, cycles: u128) {
        self.get_mut()
            .business_config_maintain_archives_cycles_recharged(canister_id, cycles)
    }

    // token frozen
    fn business_config_token_frozen_query(&self) -> &HashSet<CanisterId> {
        self.get().business_config_token_frozen_query()
    }
    fn business_config_token_frozen(&mut self, arg: ArgWithMeta<TokenFrozenArg>) {
        self.get_mut().business_config_token_frozen(arg)
    }

    // token custom
    fn business_config_token_preset_query(&self) -> &HashMap<CanisterId, TokenInfo> {
        self.get().business_config_token_preset_query()
    }
    fn business_config_token_custom_query(&self) -> Vec<TokenInfo> {
        self.get().business_config_token_custom_query()
    }
    fn business_config_token_custom_put(&mut self, arg: ArgWithMeta<TokenInfo>) {
        self.get_mut().business_config_token_custom_put(arg)
    }
    fn business_config_token_custom_remove(&mut self, arg: ArgWithMeta<CanisterId>) -> Option<TokenInfo> {
        self.get_mut().business_config_token_custom_remove(arg)
    }

    // set_certified_data
    fn business_certified_data_refresh(&self) {
        self.get().business_certified_data_refresh()
    }

    // ======================== locks ========================

    // token block chain
    fn business_token_block_chain_archive_lock(&mut self) -> Option<TokenBlockChainArchiveLock> {
        self.get_mut().business_token_block_chain_archive_lock()
    }
    fn business_token_block_chain_archive_unlock(&mut self) {
        self.get_mut().business_token_block_chain_archive_unlock()
    }
    fn business_token_block_chain_lock(&mut self) -> Option<TokenBlockChainLock> {
        self.get_mut().business_token_block_chain_lock()
    }
    fn business_token_block_chain_unlock(&mut self) {
        self.get_mut().business_token_block_chain_unlock()
    }

    // swap block chain
    fn business_swap_block_chain_archive_lock(&mut self) -> Option<SwapBlockChainArchiveLock> {
        self.get_mut().business_swap_block_chain_archive_lock()
    }
    fn business_swap_block_chain_archive_unlock(&mut self) {
        self.get_mut().business_swap_block_chain_archive_unlock()
    }
    fn business_swap_block_chain_lock(&mut self) -> Option<SwapBlockChainLock> {
        self.get_mut().business_swap_block_chain_lock()
    }
    fn business_swap_block_chain_unlock(&mut self) {
        self.get_mut().business_swap_block_chain_unlock()
    }

    // token balance
    fn business_token_balance_lock(
        &mut self,
        required: Vec<TokenAccount>,
    ) -> Result<TokenBalancesLock, Vec<TokenAccount>> {
        self.get_mut().business_token_balance_lock(required)
    }
    fn business_token_balance_unlock(&mut self, locked: &HashSet<TokenAccount>) {
        self.get_mut().business_token_balance_unlock(locked)
    }

    // ======================== token block chain ========================

    // ======================== query ========================

    fn business_token_alive(&self, canister_id: &CanisterId) -> Result<(), BusinessError> {
        self.get().business_token_alive(canister_id)
    }
    fn business_tokens_query(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        self.get().business_tokens_query()
    }
    fn business_dummy_tokens_query(&self) -> HashMap<CanisterId, TokenInfo> {
        self.get().business_dummy_tokens_query()
    }
    fn business_all_tokens_with_dummy_query(&self) -> HashMap<CanisterId, Cow<'_, TokenInfo>> {
        self.get().business_all_tokens_with_dummy_query()
    }
    fn business_token_query(&self, token: &CanisterId) -> Option<TokenInfo> {
        self.get().business_token_query(token)
    }
    fn business_token_query_by_pa(&self, pa: &TokenPairAmm) -> Option<TokenInfo> {
        self.get().business_token_query_by_pa(pa)
    }
    fn business_token_balance_of(&self, token: CanisterId, account: Account) -> candid::Nat {
        self.get().business_token_balance_of(token, account)
    }
    fn business_token_balance_of_with_fee_to(
        &self,
        token: CanisterId,
        account: Account,
    ) -> (candid::Nat, Option<Account>) {
        self.get().business_token_balance_of_with_fee_to(token, account)
    }

    // ======================== update ========================

    fn business_token_deposit(
        &mut self,
        locks: &(TokenBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<DepositToken>,
        height: Nat,
    ) -> Result<Nat, BusinessError> {
        self.get_mut().business_token_deposit(locks, arg, height)
    }
    fn business_token_withdraw(
        &mut self,
        locks: &(TokenBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<WithdrawToken>,
        height: Nat,
    ) -> Result<Nat, BusinessError> {
        self.get_mut().business_token_withdraw(locks, arg, height)
    }
    fn business_token_transfer(
        &mut self,
        locks: &(TokenBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        self.get_mut().business_token_transfer(locks, arg)
    }
    fn business_token_transfer_lp(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        self.get_mut().business_token_transfer_lp(locks, arg)
    }

    // ======================== swap block chain ========================

    // ======================== token pair swap ========================

    // query
    fn business_token_pair_pools_query(&self) -> Vec<(TokenPairAmm, MarketMaker)> {
        self.get().business_token_pair_pools_query()
    }
    fn business_token_pair_pool_get(&self, pa: &TokenPairAmm) -> Option<MarketMaker> {
        self.get().business_token_pair_pool_get(pa)
    }
    // create and remove
    fn business_token_pair_pool_create(
        &mut self,
        lock: &SwapBlockChainLock,
        arg: ArgWithMeta<TokenPairAmm>,
    ) -> Result<MarketMaker, BusinessError> {
        self.get_mut().business_token_pair_pool_create(lock, arg)
    }
    fn business_token_pair_pool_remove(
        &mut self,
        lock: &SwapBlockChainLock,
        arg: ArgWithMeta<TokenPairAmm>,
    ) -> Result<MarketMaker, BusinessError> {
        self.get_mut().business_token_pair_pool_remove(lock, arg)
    }
    // liquidity
    fn business_token_pair_liquidity_add(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairLiquidityAddArg>,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        self.get_mut().business_token_pair_liquidity_add(locks, arg)
    }
    fn business_token_pair_check_liquidity_removable(
        &self,
        pa: &TokenPairAmm,
        from: &Account,
        liquidity_without_fee: &Nat,
    ) -> Result<(), BusinessError> {
        self.get()
            .business_token_pair_check_liquidity_removable(pa, from, liquidity_without_fee)
    }
    fn business_token_pair_liquidity_remove(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairLiquidityRemoveArg>,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        self.get_mut().business_token_pair_liquidity_remove(locks, arg)
    }

    // pair swap
    fn business_token_pair_swap_fixed_in_checking(
        &self,
        arg: &TokenPairSwapExactTokensForTokensArg,
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        self.get().business_token_pair_swap_fixed_in_checking(arg)
    }
    fn business_token_pair_swap_fixed_out_checking(
        &self,
        arg: &TokenPairSwapTokensForExactTokensArg,
    ) -> Result<(Vec<Nat>, Vec<Account>), BusinessError> {
        self.get().business_token_pair_swap_fixed_out_checking(arg)
    }
    fn business_token_pair_swap_exact_tokens_for_tokens(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairSwapExactTokensForTokensArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.get_mut()
            .business_token_pair_swap_exact_tokens_for_tokens(locks, arg)
    }
    fn business_token_pair_swap_tokens_for_exact_tokens(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairSwapTokensForExactTokensArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.get_mut()
            .business_token_pair_swap_tokens_for_exact_tokens(locks, arg)
    }
    fn business_token_pair_swap_by_loan(
        &mut self,
        locks: &(TokenBlockChainLock, SwapBlockChainLock, TokenBalancesLock),
        arg: ArgWithMeta<TokenPairSwapByLoanArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.get_mut().business_token_pair_swap_by_loan(locks, arg)
    }

    // ======================== blocks query ========================

    fn business_token_queryable(&self, caller: &UserId) -> Result<(), String> {
        self.get().business_token_queryable(caller)
    }
    fn business_swap_queryable(&self, caller: &UserId) -> Result<(), String> {
        self.get().business_swap_queryable(caller)
    }

    fn business_token_block_get(&self, block_height: BlockIndex) -> QueryBlockResult<EncodedBlock> {
        self.get().business_token_block_get(block_height)
    }
    fn business_swap_block_get(&self, block_height: BlockIndex) -> QueryBlockResult<EncodedBlock> {
        self.get().business_swap_block_get(block_height)
    }

    fn business_token_blocks_get(&self, block_height: BlockIndex) -> Vec<(BlockIndex, QueryBlockResult<EncodedBlock>)> {
        self.get().business_token_blocks_get(block_height)
    }
    fn business_swap_blocks_get(&self, block_height: BlockIndex) -> Vec<(BlockIndex, QueryBlockResult<EncodedBlock>)> {
        self.get().business_swap_blocks_get(block_height)
    }

    // ======================== request ========================

    fn business_request_index_get(&self) -> (RequestIndex, u64) {
        self.get().business_request_index_get()
    }
    fn business_request_trace_get(&self, index: &RequestIndex) -> Option<RequestTrace> {
        self.get().business_request_trace_get(index)
    }
    fn business_request_trace_remove(&mut self, index: &RequestIndex) -> Option<RequestTrace> {
        self.get_mut().business_request_trace_remove(index)
    }
    fn business_request_trace_insert(&mut self, trace: RequestTrace) {
        self.get_mut().business_request_trace_insert(trace)
    }
}
