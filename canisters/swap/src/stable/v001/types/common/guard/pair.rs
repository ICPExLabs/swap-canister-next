use candid::Nat;
use common::types::{BurnFee, TimestampNanos};
use ic_canister_kit::{common::option::display_option, types::CanisterId};
use icrc_ledger_types::icrc1::account::Account;

use crate::types::{SelfCanisterArg, TokenPairArg};

use super::super::{
    ArgWithMeta, BusinessError, DepositToken, PairOperation, RequestTraceGuard, SwapBlockChainGuard, SwapOperation,
    SwapTransaction, SwapV2BurnToken, SwapV2MintFeeToken, SwapV2MintToken, SwapV2Operation, SwapV2State,
    TokenBalancesGuard, TokenBlockChainGuard, TokenPairAmm, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess,
    TokenPairLiquidityAddSuccessView, TokenPairLiquidityRemoveArg, TokenPairLiquidityRemoveSuccess,
    TokenPairLiquidityRemoveSuccessView, TokenPairSwapByLoanArg, TokenPairSwapExactTokensForTokensArg,
    TokenPairSwapTokensForExactTokensArg, TokenPairSwapTokensSuccess, TokenPairSwapTokensSuccessView, TokenPairs,
    TransferToken, WithdrawToken, display_account,
};

pub struct TokenPairSwapGuard<'a> {
    trace_guard: RequestTraceGuard<'a>,
    balances_guard: TokenBalancesGuard<'a>,
    token_guard: TokenBlockChainGuard<'a>,
    swap_guard: SwapBlockChainGuard<'a>,
    token_pairs: &'a mut TokenPairs,
}

impl<'a> TokenPairSwapGuard<'a> {
    pub fn new(
        trace_guard: RequestTraceGuard<'a>,
        balances_guard: TokenBalancesGuard<'a>,
        token_guard: TokenBlockChainGuard<'a>,
        swap_guard: SwapBlockChainGuard<'a>,
        token_pairs: &'a mut TokenPairs,
    ) -> Self {
        Self {
            trace_guard,
            balances_guard,
            token_guard,
            swap_guard,
            token_pairs,
        }
    }

    // transfer lp token
    pub fn token_lp_transfer(
        &mut self,
        pa: TokenPairAmm,
        arg: ArgWithMeta<TransferToken>,
    ) -> Result<Nat, BusinessError> {
        self.balances_guard
            .token_lp_transfer(&mut self.swap_guard, pa, &mut self.token_guard, arg)
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
                    arg,
                };
                let data = self.token_pairs.swap_exact_tokens_for_tokens(&mut inner, pas)?;
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
                    arg,
                };
                let data = self.token_pairs.swap_tokens_for_exact_tokens(&mut inner, pas)?;
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
    pub arg: ArgWithMeta<T>,
}

impl<T> InnerTokenPairSwapGuard<'_, '_, '_, T> {
    pub fn handle_guard<R, D, F>(&mut self, arg: ArgWithMeta<D>, handle: F) -> Result<R, BusinessError>
    where
        F: FnOnce(&mut InnerTokenPairSwapGuard<'_, '_, '_, D>) -> Result<R, BusinessError>,
    {
        let mut inner = InnerTokenPairSwapGuard {
            trace_guard: self.trace_guard,
            balances_guard: self.balances_guard,
            token_guard: self.token_guard,
            swap_guard: self.swap_guard,
            arg,
        };
        handle(&mut inner)
    }

    pub fn token_balance_of(&self, token: CanisterId, account: Account) -> Result<candid::Nat, BusinessError> {
        self.balances_guard.token_balance_of(token, account)
    }

    pub fn get_swap_fee_to(&self) -> Option<Account> {
        self.swap_guard.get_fee_to()
    }

    /// Check whether the balance meets the requirements
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
        let changed = self.balances_guard.token_transfer(self.token_guard, arg.clone())?; // do transfer
        self.trace_guard.trace(format!(
            "*TokenTransfer* `token:[{}], from:({}), to:({}), amount:{}, done:{changed}`",
            arg.arg.token.to_text(),
            display_account(&arg.arg.from),
            display_account(&arg.arg.to),
            arg.arg.amount,
        )); // * trace
        Ok(())
    }

    /// Lend tokens
    pub fn token_loan(&mut self, arg: DepositToken) -> Result<(), BusinessError> {
        let arg = ArgWithMeta::simple(self.arg.now, self.arg.caller, arg);
        let trace = format!(
            "*TokenLoan* `token:[{}], from:({}), to:({}), amount:{}`",
            arg.arg.token.to_text(),
            display_account(&arg.arg.from),
            display_account(&arg.arg.to),
            arg.arg.amount,
        );
        self.balances_guard.token_deposit(self.token_guard, arg)?; // do loan
        self.trace_guard.trace(trace); // * trace
        Ok(())
    }
    /// Return tokens
    pub fn token_repay(&mut self, arg: WithdrawToken) -> Result<(), BusinessError> {
        let arg = ArgWithMeta::simple(self.arg.now, self.arg.caller, arg);
        let trace = format!(
            "*TokenRepay* `token:[{}], from:({}), to:({}), amount:{}`",
            arg.arg.token.to_text(),
            display_account(&arg.arg.from),
            display_account(&arg.arg.to),
            arg.arg.amount,
        );
        self.balances_guard.token_withdraw(self.token_guard, arg)?; // do repay
        self.trace_guard.trace(trace); // * trace
        Ok(())
    }
}

impl<T> InnerTokenPairSwapGuard<'_, '_, '_, T> {
    pub fn mint_swap_block<R, F>(
        &mut self,
        now: TimestampNanos,
        transaction: SwapTransaction,
        handle: F,
        trace: String,
    ) -> Result<R, BusinessError>
    where
        F: FnOnce(&mut InnerTokenPairSwapGuard<'_, '_, '_, &T>) -> Result<R, BusinessError>,
    {
        let result = self.swap_guard.mint_block(now, transaction, |swap_guard| {
            let mut inner = InnerTokenPairSwapGuard {
                trace_guard: self.trace_guard,
                balances_guard: self.balances_guard,
                token_guard: self.token_guard,
                swap_guard,
                arg: ArgWithMeta {
                    now: self.arg.now,
                    caller: self.arg.caller,
                    arg: &self.arg.arg,
                    memo: self.arg.memo.clone(),
                    created: self.arg.created,
                },
            };
            handle(&mut inner)
        })?;
        self.trace(trace);
        Ok(result)
    }
}

impl<T: TokenPairArg> InnerTokenPairSwapGuard<'_, '_, '_, T> {
    pub fn push_state(
        &mut self,
        reserve0: Nat,
        reserve1: Nat,
        supply: Nat,
        price_cumulative_exponent: u8,
        price0_cumulative: Nat,
        price1_cumulative: Nat,
    ) -> Result<(), BusinessError> {
        let message = format!(
            "*SwapV2State* `pa:({}), timestamp:{}, supply:{supply}, reserve0:{reserve0}, reserve1:{reserve1}, exponent:{price_cumulative_exponent}, price0_cumulative:{price0_cumulative}, price1_cumulative:{price1_cumulative}`",
            self.arg.arg.get_pa(),
            self.arg.now.into_inner(),
        );
        // reserve and cumulative price
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::SwapV2(SwapV2Operation::State(SwapV2State {
                pa: self.arg.arg.get_pa().to_owned(),
                block_timestamp: self.arg.now,
                supply,
                reserve0,
                reserve1,
                price_cumulative_exponent,
                price0_cumulative,
                price1_cumulative,
            }))),
            memo: None,
            created: None,
        };
        self.swap_guard.mint_block(self.arg.now, transaction, |_| {
            // do nothing
            Ok(())
        })?;
        self.trace(message); // * trace
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
            operation: SwapOperation::Pair(PairOperation::SwapV2(SwapV2Operation::MintFee(SwapV2MintFeeToken {
                pa: self.arg.arg.get_pa().to_owned(),
                to,
                token,
                amount: amount.clone(),
            }))),
            memo: None,
            created: None,
        };
        // Mint coins for handling fee accounts, generate a DepositToken event
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
            let trace = format!(
                "*PairFeeMint(Deposit)* `token:[{}], from:({}), to:({}), amount:{}`",
                arg.arg.token.to_text(),
                display_account(&arg.arg.from),
                display_account(&arg.arg.to),
                arg.arg.amount,
            );
            self.balances_guard.token_deposit(self.token_guard, arg)?;
            self.trace_guard.trace(trace); // * trace
            Ok(())
        })?;
        self.trace(format!(
            "*PairFeeMint*. `token:[{}], to:({}), amount:{amount}`",
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
        pool_account: Account,
        to: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        // mint
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::SwapV2(SwapV2Operation::Mint({
                let ((token0, token1), (amount0, amount1)) = if self.arg.arg.pa.pair.token0 == self.arg.arg.token_a {
                    (
                        (self.arg.arg.token_a, self.arg.arg.token_b),
                        (amount_a.clone(), amount_b.clone()),
                    )
                } else {
                    (
                        (self.arg.arg.token_b, self.arg.arg.token_a),
                        (amount_b.clone(), amount_a.clone()),
                    )
                };
                SwapV2MintToken {
                    pa: self.arg.arg.pa,
                    from: self.arg.arg.from,
                    token0,
                    token1,
                    amount0,
                    amount1,
                    token,
                    amount: amount.clone(),
                    to,
                }
            }))),
            memo: self.arg.memo.clone(),
            created: self.arg.created,
        };
        // Mint coins for users and generate a DepositToken event
        let arg = ArgWithMeta::simple(
            self.arg.now,
            self.arg.caller,
            DepositToken {
                token,
                from: pool_account,
                amount: amount.clone(),
                to,
            },
        );
        self.swap_guard.mint_block(self.arg.now, transaction, |_| {
            let trace = format!(
                "*PairLiquidityMint(Deposit)* `token:[{}], from[transferred 2 tokens]:({}), to[minted liquidity]:({}), amount:{}`",
                arg.arg.token.to_text(),
                display_account(&pool_account),
                display_account(&arg.arg.to),
                arg.arg.amount,
            );
            self.balances_guard.token_deposit(self.token_guard, arg)?;
            self.trace_guard.trace(trace); // * trace
            Ok(())
        })?;
        self.trace(format!(
            "*PairLiquidityMint* `token:[{}], to:({}), amount:{amount}`",
            token.to_text(),
            display_account(&to),
        )); // * trace
        Ok(())
    }
}

impl InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityRemoveArg> {
    #[allow(clippy::too_many_arguments)]
    pub fn token_liquidity_burn(
        &mut self,
        amount_a: &Nat,
        amount_b: &Nat,
        token: CanisterId,
        from: Account,
        pool_account: Account,
        amount: Nat,
        fee: Option<BurnFee>,
    ) -> Result<(), BusinessError> {
        // burn
        let transaction = SwapTransaction {
            operation: SwapOperation::Pair(PairOperation::SwapV2(SwapV2Operation::Burn({
                let ((token0, token1), (amount0, amount1)) = if self.arg.arg.pa.pair.token0 == self.arg.arg.token_a {
                    (
                        (self.arg.arg.token_a, self.arg.arg.token_b),
                        (amount_a.clone(), amount_b.clone()),
                    )
                } else {
                    (
                        (self.arg.arg.token_b, self.arg.arg.token_a),
                        (amount_b.clone(), amount_a.clone()),
                    )
                };
                SwapV2BurnToken {
                    pa: self.arg.arg.pa,
                    from,
                    token0,
                    token1,
                    amount0,
                    amount1,
                    token,
                    amount: amount.clone(),
                    to: self.arg.arg.to,
                    fee: fee.clone(),
                }
            }))),
            memo: self.arg.memo.clone(),
            created: self.arg.created,
        };
        // Destroy for users, generates a WithdrawToken event
        let arg = ArgWithMeta::simple(
            self.arg.now,
            self.arg.caller,
            WithdrawToken {
                token,
                from,
                amount: amount.clone() + fee.as_ref().map(|f| f.fee.clone()).unwrap_or_default(), // withdraw sum
                to: pool_account,
            },
        );
        let deposit_fee = fee.map(|BurnFee { fee, fee_to }| {
            ArgWithMeta::simple(
                self.arg.now,
                self.arg.caller,
                DepositToken {
                    token,
                    from,
                    amount: fee, // deposit fee
                    to: fee_to,
                },
            )
        });
        self.swap_guard.mint_block(self.arg.now, transaction, |_| {
            let trace = format!(
                "*PairLiquidityBurn(Withdraw)*. `token:[{}], from[burned liquidity]:({}), to[withdrawn 2 tokens]:({}), amount:{}, fee:{}`",
                arg.arg.token.to_text(),
                display_account(&arg.arg.from),
                display_account(&pool_account),
                arg.arg.amount,
                display_option(&deposit_fee.as_ref().map(|d|d.arg.amount.to_string()))
            );
            self.balances_guard
                .token_withdraw(self.token_guard, arg)?;
            self.trace_guard.trace(trace); // * trace
            if let Some(deposit_fee) = deposit_fee {
                let trace = format!(
                    "*PairBurnFeeMint(Deposit)*. `token:[{}], to:({}), amount:{}`",
                    deposit_fee.arg.token.to_text(),
                    display_account(&deposit_fee.arg.to),
                    deposit_fee.arg.amount,
                );
                self.balances_guard.token_deposit(self.token_guard, deposit_fee.clone())?;
                self.trace_guard.trace(trace); // * trace
            }
            Ok(())
        })?;
        self.trace(format!(
            "*PairLiquidityBurn* `token:[{}], from:({}), amount:{amount}`",
            token.to_text(),
            display_account(&from),
        )); // * trace
        Ok(())
    }
}
