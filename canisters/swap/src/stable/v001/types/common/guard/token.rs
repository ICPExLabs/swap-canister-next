use candid::Nat;
use common::types::BusinessError;

use super::super::{
    ArgWithMeta, DepositToken, RequestTraceGuard, TokenBalancesGuard, TokenBlockChainGuard, TransferToken,
    WithdrawToken, display_account,
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

    pub fn token_deposit(&mut self, arg: ArgWithMeta<DepositToken>, height: Nat) -> Result<Nat, BusinessError> {
        self.trace_guard.handle(
            |trace| {
                trace.trace(format!(
                    "*Deposit* `token:[{}], from:({}), to:({}), amount:{}, height:{height}`",
                    arg.arg.token.to_text(),
                    display_account(&arg.arg.from),
                    display_account(&arg.arg.to),
                    arg.arg.amount,
                )); // * trace
                self.balances_guard.token_deposit(&mut self.token_guard, arg)?; // do deposit
                trace.trace("Deposit Done.".into()); // * trace
                Ok(height)
            },
            |data| data.to_string(),
        )
    }

    pub fn token_withdraw(&mut self, arg: ArgWithMeta<WithdrawToken>, height: Nat) -> Result<Nat, BusinessError> {
        self.trace_guard.handle(
            |trace| {
                trace.trace(format!(
                    "*Withdraw* `token:[{}], from:({}), to:({}), amount:{}, height:{height}`",
                    arg.arg.token.to_text(),
                    display_account(&arg.arg.from),
                    display_account(&arg.arg.to),
                    arg.arg.amount,
                )); // * trace
                self.balances_guard.token_withdraw(&mut self.token_guard, arg)?; // do withdraw
                trace.trace("Withdraw Done.".into()); // * trace
                Ok(height)
            },
            |data| data.to_string(),
        )
    }

    pub fn token_transfer(&mut self, arg: ArgWithMeta<TransferToken>) -> Result<Nat, BusinessError> {
        self.trace_guard.handle(
            |trace| {
                trace.trace(format!(
                    "*Transfer* `token:[{}], from:({}), to:({}), amount:{}`",
                    arg.arg.token.to_text(),
                    display_account(&arg.arg.from),
                    display_account(&arg.arg.to),
                    arg.arg.amount,
                )); // * trace
                let changed = self.balances_guard.token_transfer(&mut self.token_guard, arg.clone())?; // do transfer
                trace.trace(format!("Transfer Done: {changed}.",)); // * trace
                Ok(changed)
            },
            |data| data.to_string(),
        )
    }
}
