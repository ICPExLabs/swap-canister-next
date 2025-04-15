use candid::Nat;
use common::types::BusinessError;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;

use crate::types::{SwapBlockChainGuard, TokenBalancesGuard, TokenBlockChainGuard};

pub struct TokenPairGuard<'a> {
    balances_guard: TokenBalancesGuard<'a>,
    token_guard: TokenBlockChainGuard<'a>,
    swap_guard: SwapBlockChainGuard<'a>,
}

impl<'a> TokenPairGuard<'a> {
    pub fn new(
        balances_guard: TokenBalancesGuard<'a>,
        token_guard: TokenBlockChainGuard<'a>,
        swap_guard: SwapBlockChainGuard<'a>,
    ) -> Self {
        Self {
            balances_guard,
            token_guard,
            swap_guard,
        }
    }

    pub fn token_balance_of(
        &self,
        token: CanisterId,
        account: Account,
    ) -> Result<candid::Nat, BusinessError> {
        self.balances_guard.token_balance_of(token, account)
    }

    /// 检查余额是否满足要求
    pub fn assert_token_balance(
        &self,
        token: CanisterId,
        account: Account,
        desired: &Nat,
    ) -> Result<(), BusinessError> {
        let balance = self.balances_guard.token_balance_of(token, account)?;
        if balance < *desired {
            return Err(BusinessError::InsufficientBalance((token, balance)));
        }
        Ok(())
    }
}
