use candid::Nat;
use common::types::TimestampNanos;
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;

use crate::types::{SelfCanisterArg, TokenPairArg};

use super::super::{
    ArgWithMeta, BusinessError, DepositToken, PairCumulativePrice, PairOperation,
    RequestTraceGuard, SwapBlockChainGuard, SwapOperation, SwapTransaction, SwapV2BurnToken,
    SwapV2MintFeeToken, SwapV2MintToken, SwapV2Operation, TokenBalancesGuard, TokenBlockChainGuard,
    TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess, TokenPairLiquidityAddSuccessView,
    TokenPairLiquidityRemoveArg, TokenPairLiquidityRemoveSuccess,
    TokenPairLiquidityRemoveSuccessView, TokenPairSwapByLoanArg,
    TokenPairSwapExactTokensForTokensArg, TokenPairSwapTokensForExactTokensArg,
    TokenPairSwapTokensSuccess, TokenPairSwapTokensSuccessView, TokenPairs, TransferToken,
    WithdrawToken, display_account,
};

pub struct TokenPairSwapGuard<'a> {
    trace_guard: RequestTraceGuard<'a>,
    balances_guard: TokenBalancesGuard<'a>,
    token_guard: TokenBlockChainGuard<'a>,
    swap_guard: SwapBlockChainGuard<'a>,
    token_pairs: &'a mut TokenPairs,
    fee_to: Option<Account>,
}

impl<'a> TokenPairSwapGuard<'a> {
    pub fn new(
        trace_guard: RequestTraceGuard<'a>,
        balances_guard: TokenBalancesGuard<'a>,
        token_guard: TokenBlockChainGuard<'a>,
        swap_guard: SwapBlockChainGuard<'a>,
        token_pairs: &'a mut TokenPairs,
        fee_to: Option<Account>,
    ) -> Self {
        Self {
            trace_guard,
            balances_guard,
            token_guard,
            swap_guard,
            token_pairs,
            fee_to,
        }
    }

    pub fn add_liquidity(
        &mut self,
        arg: ArgWithMeta<TokenPairLiquidityAddArg>,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        self.trace_guard.handle(
            |trace| {
                let pa = arg.arg.pa;
                let mut inner = InnerTokenPairSwapGuard {
                    trace_guard: trace,
                    balances_guard: &mut self.balances_guard,
                    token_guard: &mut self.token_guard,
                    swap_guard: &mut self.swap_guard,
                    fee_to: self.fee_to,
                    arg,
                };
                let data = self.token_pairs.add_liquidity(&mut inner, pa)?;
                trace.trace("Token Pair Add Liquidity Done.".into());
                Ok(data)
            },
            |data| {
                let view: TokenPairLiquidityAddSuccessView = data.into();
                serde_json::to_string(&view).unwrap_or_default()
            },
        )
    }

    pub fn remove_liquidity(
        &mut self,
        arg: ArgWithMeta<TokenPairLiquidityRemoveArg>,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        self.trace_guard.handle(
            |trace| {
                let pa = arg.arg.pa;
                let mut inner = InnerTokenPairSwapGuard {
                    trace_guard: trace,
                    balances_guard: &mut self.balances_guard,
                    token_guard: &mut self.token_guard,
                    swap_guard: &mut self.swap_guard,
                    fee_to: self.fee_to,
                    arg,
                };
                let data = self.token_pairs.remove_liquidity(&mut inner, pa)?;
                trace.trace("Token Pair Remove Liquidity Done.".into());
                Ok(data)
            },
            |data| {
                let view: TokenPairLiquidityRemoveSuccessView = data.into();
                serde_json::to_string(&view).unwrap_or_default()
            },
        )
    }

    pub fn swap_exact_tokens_for_tokens(
        &mut self,
        arg: ArgWithMeta<TokenPairSwapExactTokensForTokensArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.trace_guard.handle(
            |trace| {
                let pas = arg.arg.pas.clone();
                let mut inner = InnerTokenPairSwapGuard {
                    trace_guard: trace,
                    balances_guard: &mut self.balances_guard,
                    token_guard: &mut self.token_guard,
                    swap_guard: &mut self.swap_guard,
                    fee_to: self.fee_to,
                    arg,
                };
                let data = self
                    .token_pairs
                    .swap_exact_tokens_for_tokens(&mut inner, pas)?;
                trace.trace("Token Pair Swap Exact Tokens for Tokens Done.".into());
                Ok(data)
            },
            |data| {
                let view: TokenPairSwapTokensSuccessView = data.into();
                serde_json::to_string(&view).unwrap_or_default()
            },
        )
    }

    pub fn swap_tokens_for_exact_tokens(
        &mut self,
        arg: ArgWithMeta<TokenPairSwapTokensForExactTokensArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.trace_guard.handle(
            |trace| {
                let pas = arg.arg.pas.clone();
                let mut inner = InnerTokenPairSwapGuard {
                    trace_guard: trace,
                    balances_guard: &mut self.balances_guard,
                    token_guard: &mut self.token_guard,
                    swap_guard: &mut self.swap_guard,
                    fee_to: self.fee_to,
                    arg,
                };
                let data = self
                    .token_pairs
                    .swap_tokens_for_exact_tokens(&mut inner, pas)?;
                trace.trace("Token Pair Swap Tokens for Exact Tokens Done.".into());
                Ok(data)
            },
            |data| {
                let view: TokenPairSwapTokensSuccessView = data.into();
                serde_json::to_string(&view).unwrap_or_default()
            },
        )
    }

    pub fn swap_by_loan(
        &mut self,
        arg: ArgWithMeta<TokenPairSwapByLoanArg>,
    ) -> Result<TokenPairSwapTokensSuccess, BusinessError> {
        self.trace_guard.handle(
            |trace| {
                let pas = arg.arg.pas.clone();
                let mut inner = InnerTokenPairSwapGuard {
                    trace_guard: trace,
                    balances_guard: &mut self.balances_guard,
                    token_guard: &mut self.token_guard,
                    swap_guard: &mut self.swap_guard,
                    fee_to: self.fee_to,
                    arg,
                };
                let data = self.token_pairs.swap_by_loan(&mut inner, pas)?;
                trace.trace("Token Pair Swap by Loan Done.".into());
                Ok(data)
            },
            |data| {
                let view: TokenPairSwapTokensSuccessView = data.into();
                serde_json::to_string(&view).unwrap_or_default()
            },
        )
    }
}

// ============================== inner guard ==============================

pub struct InnerTokenPairSwapGuard<'a, 'b, 'c, T> {
    trace_guard: &'a mut RequestTraceGuard<'c>,
    balances_guard: &'a mut TokenBalancesGuard<'b>,
    token_guard: &'a mut TokenBlockChainGuard<'b>,
    swap_guard: &'a mut SwapBlockChainGuard<'b>,
    pub fee_to: Option<Account>,
    pub arg: ArgWithMeta<T>,
}

impl<T> InnerTokenPairSwapGuard<'_, '_, '_, T> {
    pub fn handle_guard<R, D, F>(
        &mut self,
        arg: ArgWithMeta<D>,
        handle: F,
    ) -> Result<R, BusinessError>
    where
        F: FnOnce(&mut InnerTokenPairSwapGuard<'_, '_, '_, D>) -> Result<R, BusinessError>,
    {
        let mut inner = InnerTokenPairSwapGuard {
            trace_guard: self.trace_guard,
            balances_guard: self.balances_guard,
            token_guard: self.token_guard,
            swap_guard: self.swap_guard,
            fee_to: self.fee_to,
            arg,
        };
        handle(&mut inner)
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
            return Err(BusinessError::insufficient_balance(token, balance));
        }
        Ok(())
    }

    pub fn trace(&mut self, trace: String) {
        self.trace_guard.trace(trace);
    }

    pub fn token_transfer(&mut self, arg: TransferToken) -> Result<(), BusinessError> {
        let arg = ArgWithMeta::simple(self.arg.now, self.arg.caller, arg);
        let changed = self
            .balances_guard
            .token_transfer(self.token_guard, arg.clone())?; // do transfer
        self.trace_guard.trace(format!(
            "*Transfer Token* `token:[{}], from:({}), to:({}), amount:{}, done:{changed}`",
            arg.arg.token.to_text(),
            display_account(&arg.arg.from),
            display_account(&arg.arg.to),
            arg.arg.amount,
        )); // * trace
        Ok(())
    }

    /// 借出代币
    pub fn token_loan(&mut self, arg: DepositToken) -> Result<(), BusinessError> {
        let arg = ArgWithMeta::simple(self.arg.now, self.arg.caller, arg);
        self.balances_guard
            .token_deposit(self.token_guard, arg.clone())?; // do loan
        self.trace_guard.trace(format!(
            "*Loan Token* `token:[{}], from:({}), to:({}), amount:{}`",
            arg.arg.token.to_text(),
            display_account(&arg.arg.from),
            display_account(&arg.arg.to),
            arg.arg.amount,
        )); // * trace
        Ok(())
    }
    /// 归还代币
    pub fn token_repay(&mut self, arg: WithdrawToken) -> Result<(), BusinessError> {
        let arg = ArgWithMeta::simple(self.arg.now, self.arg.caller, arg);
        self.balances_guard
            .token_withdraw(self.token_guard, arg.clone())?; // do repay
        self.trace_guard.trace(format!(
            "*Repay Token* `token:[{}], from:({}), to:({}), amount:{}`",
            arg.arg.token.to_text(),
            display_account(&arg.arg.from),
            display_account(&arg.arg.to),
            arg.arg.amount,
        )); // * trace
        Ok(())
    }
}

impl<T: Clone> InnerTokenPairSwapGuard<'_, '_, '_, T> {
    pub fn mint_swap_block<R, F>(
        &mut self,
        now: TimestampNanos,
        transaction: SwapTransaction,
        handle: F,
        trace: String,
    ) -> Result<R, BusinessError>
    where
        F: FnOnce(&mut InnerTokenPairSwapGuard<'_, '_, '_, T>) -> Result<R, BusinessError>,
    {
        let result = self.swap_guard.mint_block(now, transaction, |swap_guard| {
            let mut inner = InnerTokenPairSwapGuard {
                trace_guard: self.trace_guard,
                balances_guard: self.balances_guard,
                token_guard: self.token_guard,
                swap_guard,
                fee_to: self.fee_to,
                arg: self.arg.clone(),
            };
            handle(&mut inner)
        })?;
        self.trace(trace);
        Ok(result)
    }
}

impl<T: TokenPairArg> InnerTokenPairSwapGuard<'_, '_, '_, T> {
    pub fn mint_cumulative_price(
        &mut self,
        price_cumulative_exponent: u8,
        price0_cumulative: Nat,
        price1_cumulative: Nat,
    ) -> Result<(), BusinessError> {
        // cumulative price
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::SwapV2(
                SwapV2Operation::CumulativePrice(PairCumulativePrice {
                    pa: self.arg.arg.get_pa().to_owned(),
                    block_timestamp: self.arg.now,
                    price_cumulative_exponent,
                    price0_cumulative: price0_cumulative.clone(),
                    price1_cumulative: price1_cumulative.clone(),
                }),
            )),
            memo: None,
            created: None,
        };
        self.swap_guard.mint_block(self.arg.now, transaction, |_| {
            // do nothing
            Ok(())
        })?;
        self.trace(format!(
            "*Pair Cumulative Price* `pa:({}), timestamp:{}, exponent:{price_cumulative_exponent}, price0_cumulative:{price0_cumulative}, price1_cumulative:{price1_cumulative}`",
            self.arg.arg.get_pa(),
            self.arg.now.into_inner(),
        )); // * trace
        Ok(())
    }
}

// ============================== pair liquidity ==============================

impl<T: SelfCanisterArg + TokenPairArg> InnerTokenPairSwapGuard<'_, '_, '_, T> {
    pub fn token_liquidity_mint_fee(
        &mut self,
        token: CanisterId,
        to: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        // mint fee
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::SwapV2(SwapV2Operation::MintFee(
                SwapV2MintFeeToken {
                    pa: self.arg.arg.get_pa().to_owned(),
                    to,
                    token,
                    amount: amount.clone(),
                },
            ))),
            memo: None,
            created: None,
        };
        // 为手续费账户铸币，产生 DepositToken 事件
        let arg = ArgWithMeta::simple(
            self.arg.now,
            self.arg.caller,
            DepositToken {
                token,
                from: Account {
                    owner: self.arg.arg.get_self_canister().id(),
                    subaccount: None,
                },
                amount: amount.clone(),
                to,
            },
        );
        self.swap_guard.mint_block(self.arg.now, transaction, |_| {
            self.balances_guard
                .token_deposit(self.token_guard, arg.clone())?;
            self.trace_guard.trace(format!(
                "*Mint Fee (Deposit)* `token:[{}], from:({}), to:({}), amount:{}`",
                arg.arg.token.to_text(),
                display_account(&arg.arg.from),
                display_account(&arg.arg.to),
                arg.arg.amount,
            )); // * trace
            Ok(())
        })?;
        self.trace(format!(
            "*Mint Fee*. `token:[{}], to:({}), amount:{amount}`",
            token.to_text(),
            display_account(&to),
        )); // * trace
        Ok(())
    }
}

impl InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg> {
    pub fn token_liquidity_mint(
        &mut self,
        amount_a: &Nat,
        amount_b: &Nat,
        token: CanisterId,
        to: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        // mint
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::SwapV2(SwapV2Operation::Mint(
                if self.arg.arg.pa.pair.token0 == self.arg.arg.token_a {
                    SwapV2MintToken {
                        pa: self.arg.arg.pa,
                        from: self.arg.arg.from,
                        token0: self.arg.arg.token_a,
                        token1: self.arg.arg.token_b,
                        amount0: amount_a.clone(),
                        amount1: amount_b.clone(),
                        token,
                        amount: amount.clone(),
                        to,
                    }
                } else {
                    SwapV2MintToken {
                        pa: self.arg.arg.pa,
                        from: self.arg.arg.from,
                        token0: self.arg.arg.token_b,
                        token1: self.arg.arg.token_a,
                        amount0: amount_b.clone(),
                        amount1: amount_a.clone(),
                        token,
                        amount: amount.clone(),
                        to,
                    }
                },
            ))),
            memo: self.arg.memo.clone(),
            created: self.arg.created,
        };
        // 为用户铸币，产生 DepositToken 事件
        let arg = ArgWithMeta::simple(
            self.arg.now,
            self.arg.caller,
            DepositToken {
                token,
                from: self.arg.arg.from,
                amount: amount.clone(),
                to,
            },
        );
        self.swap_guard.mint_block(self.arg.now, transaction, |_| {
            self.balances_guard
                .token_deposit(self.token_guard, arg.clone())?;
            self.trace_guard.trace(format!(
                "*Mint Liquidity (Deposit)* `token:[{}], from[transferred 2 tokens]:({}), to[minted liquidity]:({}), amount:{}`",
                arg.arg.token.to_text(),
                display_account(&arg.arg.from),
                display_account(&arg.arg.to),
                arg.arg.amount,
            )); // * trace
            Ok(())
        })?;
        self.trace(format!(
            "*Mint Liquidity* `token:[{}], to:({}), amount:{amount}`",
            token.to_text(),
            display_account(&to),
        )); // * trace
        Ok(())
    }
}

impl InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityRemoveArg> {
    pub fn token_liquidity_burn(
        &mut self,
        amount_a: &Nat,
        amount_b: &Nat,
        token: CanisterId,
        from: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        // burn
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::SwapV2(SwapV2Operation::Burn(
                if self.arg.arg.pa.pair.token0 == self.arg.arg.token_a {
                    SwapV2BurnToken {
                        pa: self.arg.arg.pa,
                        from,
                        token0: self.arg.arg.token_a,
                        token1: self.arg.arg.token_b,
                        amount0: amount_a.clone(),
                        amount1: amount_b.clone(),
                        token,
                        amount: amount.clone(),
                        to: self.arg.arg.to,
                    }
                } else {
                    SwapV2BurnToken {
                        pa: self.arg.arg.pa,
                        from,
                        token0: self.arg.arg.token_b,
                        token1: self.arg.arg.token_a,
                        amount0: amount_b.clone(),
                        amount1: amount_a.clone(),
                        token,
                        amount: amount.clone(),
                        to: self.arg.arg.to,
                    }
                },
            ))),
            memo: self.arg.memo.clone(),
            created: self.arg.created,
        };
        // 为用户销毁，产生 WithdrawToken 事件
        let arg = ArgWithMeta::simple(
            self.arg.now,
            self.arg.caller,
            WithdrawToken {
                token,
                from,
                amount: amount.clone(),
                to: self.arg.arg.to,
            },
        );
        self.swap_guard.mint_block(self.arg.now, transaction, |_| {
            self.balances_guard
                .token_withdraw(self.token_guard, arg.clone())?;
            self.trace_guard.trace(format!(
                "*Burn Liquidity (Withdraw)*. `token:[{}], from[burned liquidity]:({}), to[withdrawn 2 tokens]:({}) amount:{}`",
                arg.arg.token.to_text(),
                display_account(&arg.arg.from),
                display_account(&arg.arg.to),
                arg.arg.amount,
            )); // * trace
            Ok(())
        })?;
        self.trace(format!(
            "*Burn Liquidity* `token:[{}], from:({}), amount:{amount}`",
            token.to_text(),
            display_account(&from),
        )); // * trace
        Ok(())
    }
}
