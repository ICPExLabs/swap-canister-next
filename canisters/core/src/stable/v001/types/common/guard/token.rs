use candid::Nat;
use common::types::BusinessError;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;

use super::super::{
    ArgWithMeta, DepositToken, RequestTraceGuard, SwapBlockChainGuard, TokenBalancesGuard,
    TokenBlockChainGuard,
};

pub struct TokenGuard<'a> {
    trace_guard: RequestTraceGuard<'a>,
    balances_guard: TokenBalancesGuard<'a>,
    token_guard: TokenBlockChainGuard<'a>,
}

impl<'a> TokenGuard<'a> {
    pub fn new(
        trace_guard: RequestTraceGuard<'a>,
        balances_guard: TokenBalancesGuard<'a>,
        token_guard: TokenBlockChainGuard<'a>,
    ) -> Self {
        Self {
            trace_guard,
            balances_guard,
            token_guard,
        }
    }

    pub fn token_deposit(
        &mut self,
        arg: ArgWithMeta<DepositToken>,
        height: Nat,
    ) -> Result<Nat, BusinessError> {
        self.trace_guard.handle(
            |_trace| {
                self.balances_guard
                    .token_deposit(&mut self.token_guard, arg)?; // do deposit
                Ok(height)
            },
            |data| data.to_string(),
        )
    }
}
