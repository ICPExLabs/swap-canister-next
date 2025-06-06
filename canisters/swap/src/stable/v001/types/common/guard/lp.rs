use candid::Nat;

use super::super::{
    ArgWithMeta, BusinessError, RequestTraceGuard, SwapBlockChainGuard, TokenBalancesGuard, TokenBlockChainGuard,
    TokenPairAmm, TransferToken,
};

pub struct LpTokenTransferGuard<'a> {
    trace_guard: RequestTraceGuard<'a>,
    balances_guard: TokenBalancesGuard<'a>,
    token_guard: TokenBlockChainGuard<'a>,
    swap_guard: SwapBlockChainGuard<'a>,
}

impl<'a> LpTokenTransferGuard<'a> {
    pub fn new(
        trace_guard: RequestTraceGuard<'a>,
        balances_guard: TokenBalancesGuard<'a>,
        token_guard: TokenBlockChainGuard<'a>,
        swap_guard: SwapBlockChainGuard<'a>,
    ) -> Self {
        Self {
            trace_guard,
            balances_guard,
            token_guard,
            swap_guard,
        }
    }

    pub fn dump(self) {
        self.balances_guard.dump();
        self.token_guard.dump();
        self.swap_guard.dump();
    }

    // transfer lp token
    pub fn token_lp_transfer(
        &mut self,
        pa: TokenPairAmm,
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        self.trace_guard.handle(
            |trace| {
                self.balances_guard
                    .token_lp_transfer(trace, &mut self.swap_guard, pa, &mut self.token_guard, arg)
            },
            |data| data.to_string(),
        )
    }
}
