/// Constant product market maker（Constant Product AMM）
/// - Formula：x * y = k（x and y are the number of two assets, and k is the constant）
/// - Representative Project：Uniswap V2/V3、SushiSwap、PancakeSwap
/// - Features
///   - Simple and efficient, suitable for most trading scenarios.
///   - There is significant slippage in large-scale transactions (significant price changes).
///   - Uniswap V3 introduces "concentrated liquidity", allowing LPs to set price ranges and improve capital efficiency.
use ::common::{
    types::SwapV2MarketMaker,
    utils::math::{ZERO, zero},
};

use super::*;

use crate::types::{
    BusinessError, SelfCanister, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess, TokenPairLiquidityRemoveArg,
    TokenPairLiquidityRemoveSuccess,
};

struct InnerSwapResult {
    balance: (Nat, Nat),
    _reserve: (Nat, Nat),
}

/// Calculate the number of tokens required to add liquidity
fn inner_add_liquidity(_self: &SwapV2MarketMaker, arg: &TokenPairLiquidityAddArg) -> Result<(Nat, Nat), BusinessError> {
    let (reserve_a, reserve_b) = _self.get_reserves(arg.token_a, arg.token_b);

    if reserve_a == *ZERO && reserve_b == *ZERO {
        Ok((arg.amount_a_desired.clone(), arg.amount_b_desired.clone())) // No calculation required for the first time
    } else {
        let amount_b_optimal = SwapV2MarketMaker::quote(&arg.amount_a_desired, &reserve_a, &reserve_b); // Calculate the number of b with a
        if amount_b_optimal <= arg.amount_b_desired {
            if amount_b_optimal < arg.amount_b_min {
                // Too few b required in a quantity of a, less than the minimum b
                return Err(BusinessError::Liquidity("INSUFFICIENT_B_AMOUNT".into()));
            }
            Ok((arg.amount_a_desired.clone(), amount_b_optimal))
        } else {
            // Too much b is required in a quantity of a, greater than the maximum b

            // Switch to another token calculation
            let amount_a_optimal = SwapV2MarketMaker::quote(&arg.amount_b_desired, &reserve_b, &reserve_a); // Calculate the number of a with b
            if amount_a_optimal > arg.amount_a_desired || amount_a_optimal < arg.amount_a_min {
                // Too much a is required in b quantity, greater than the maximum a
                // Too little a is needed in b quantity, less than the minimum a
                return Err(BusinessError::Liquidity("INSUFFICIENT_A_AMOUNT".into()));
            }
            Ok((amount_a_optimal, arg.amount_b_desired.clone()))
        }
    }
}

fn mint_fee<T: SelfCanisterArg + TokenPairArg>(
    _self: &mut SwapV2MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, T>,
    pool_account: &Account,
    _reserve0: &Nat,
    _reserve1: &Nat,
) -> Result<bool, BusinessError> {
    let fee_to = guard.get_swap_fee_to();
    let fee_on = fee_to.is_some() && _self.protocol_fee.as_ref().is_some_and(|fee| !fee.is_zero());

    let _k_last = _self.k_last.clone();
    if fee_on {
        if _k_last != *ZERO {
            let root_k = Nat::from((_reserve0.to_owned() * _reserve1.to_owned()).0.sqrt());
            let root_k_last = Nat::from(_k_last.0.sqrt());
            if root_k > root_k_last {
                if let (Some(fee_to), Some(protocol_fee)) = (fee_to, &_self.protocol_fee) {
                    // https://learnblockchain.cn/article/8893

                    //        L2 - L1
                    // --------------------- * S1
                    //  (1/r - 1) * L2 + L1

                    //        L2 - L1
                    // --------------------- * S1
                    //  (d/n - 1) * L2 + L1

                    //         L2 - L1
                    // ----------------------- * S1 * n
                    //  (d - n) * L2 + n * L1

                    let n = Nat::from(protocol_fee.numerator);
                    let d = Nat::from(protocol_fee.denominator);
                    let total_supply = _self.lp.get_total_supply();
                    let numerator = (root_k.clone() - root_k_last.clone()) * total_supply * n.clone();
                    let denominator = (d - n.clone()) * root_k + n * root_k_last;
                    let liquidity = numerator / denominator;
                    if liquidity > *ZERO {
                        _self.lp.mint_fee(
                            |token, to, amount| guard.token_liquidity_mint_fee(token, *pool_account, to, amount),
                            fee_to,
                            liquidity,
                        )?;
                    }
                }
            }
        }
    } else if _k_last != *ZERO {
        _self.k_last = zero()
    }

    Ok(fee_on)
}

fn update<T: TokenPairArg>(
    _self: &mut SwapV2MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, T>,
    balance0: Nat,
    balance1: Nat,
    _reserve0: Nat,
    _reserve1: Nat,
) -> Result<(), BusinessError> {
    let block_timestamp = guard.arg.now.into_inner();
    let time_elapsed = block_timestamp - _self.block_timestamp_last;
    if time_elapsed > 0 && _reserve0 > *ZERO && _reserve1 > *ZERO {
        let e = Nat::from(time_elapsed);
        let price_cumulative_unit = _self.price_cumulative_unit();
        _self.price0_cumulative_last +=
            e.clone() * _reserve1.clone() * price_cumulative_unit.clone() / _reserve0.clone();
        _self.price1_cumulative_last +=
            e.clone() * _reserve0.clone() * price_cumulative_unit.clone() / _reserve1.clone();
    }
    _self.reserve0 = balance0;
    _self.reserve1 = balance1;
    _self.block_timestamp_last = block_timestamp;
    guard.push_state(
        _self.reserve0.clone(),
        _self.reserve1.clone(),
        _self.lp.get_total_supply(),
        _self.price_cumulative_exponent,
        _self.price0_cumulative_last.clone(),
        _self.price1_cumulative_last.clone(),
    )?;
    Ok(())
}

/// mint liquidity
fn mint(
    _self: &mut SwapV2MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg>,
    amount_a: &Nat,
    amount_b: &Nat,
    pool_account: &Account,
) -> Result<Nat, BusinessError> {
    let (token0, token1) = (_self.token0, _self.token1);

    // Get the current hold
    let (_reserve0, _reserve1) = _self.get_reserves(token0, token1);
    // Get the current balance
    let balance0 = guard.token_balance_of(token0, *pool_account)?;
    let balance1 = guard.token_balance_of(token1, *pool_account)?;
    // Calculate the increase amount
    let amount0 = balance0.clone() - _reserve0.clone();
    let amount1 = balance1.clone() - _reserve1.clone();

    // Charge a handling fee to mint LP tokens before the additional liquidity can be calculated
    let fee_on = mint_fee(_self, guard, pool_account, &_reserve0, &_reserve1)?;
    // Calculate increased liquidity
    let _total_supply = _self.lp.get_total_supply();
    let liquidity = if _total_supply == *ZERO {
        Nat::from((amount0 * amount1).0.sqrt())
    } else {
        let liquidity0 = amount0 * _total_supply.clone() / _reserve0.clone();
        let liquidity1 = amount1 * _total_supply.clone() / _reserve1.clone();
        liquidity0.min(liquidity1)
    };

    // do mint，Mint LP tokens for users
    let arg_to = guard.arg.arg.to;
    _self.lp.mint(
        |token, to, amount| guard.token_liquidity_mint(amount_a, amount_b, token, *pool_account, to, amount),
        arg_to,
        liquidity.clone(),
    )?;

    // Update the current balance
    update(_self, guard, balance0, balance1, _reserve0, _reserve1)?;
    if fee_on {
        _self.k_last = _self.reserve0.clone() * _self.reserve1.clone(); // Record the current k
    }

    Ok(liquidity)
}

pub fn add_liquidity(
    _self: &mut SwapV2MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg>,
) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
    // ! check balance
    {
        let arg = &guard.arg.arg;
        guard.assert_token_balance(arg.token_a, arg.from, &arg.amount_a_desired)?;
        guard.assert_token_balance(arg.token_b, arg.from, &arg.amount_b_desired)?;
        guard.trace(format!(
            "*PairLiquidityAdd* `tokenA:[{}], tokenB:[{}], amm:{}, required: {} <= amount_a <= {} && {} <= amount_b <= {}`",
            arg.token_a.to_text(),
            arg.token_b.to_text(),
            arg.pa.amm.into_text().as_ref(),
            arg.amount_a_min,
            arg.amount_a_desired,
            arg.amount_b_min,
            arg.amount_b_desired,
        )); // * trace
    }

    // calculate amount
    let arg = &guard.arg.arg;
    let (amount_a, amount_b) = inner_add_liquidity(_self, arg)?;
    // Pool token account
    let message = format!("*PairLiquidityAdd* `amount_a:{amount_a}, amount_b:{amount_b}`");
    let arg = &guard.arg.arg;
    let pool_account = Account {
        owner: arg.self_canister.id(),
        subaccount: Some(_self.subaccount),
    };
    guard.token_transfer(TransferToken {
        token: arg.token_a,
        from: arg.from,
        amount: amount_a.clone(),
        to: pool_account,
        fee: None,
    })?; // * transfer and trace
    let arg = &guard.arg.arg;
    guard.token_transfer(TransferToken {
        token: arg.token_b,
        from: arg.from,
        amount: amount_b.clone(),
        to: pool_account,
        fee: None,
    })?; // * transfer and trace
    guard.trace(message); // * trace

    let liquidity = mint(_self, guard, &amount_a, &amount_b, &pool_account)?;
    Ok(TokenPairLiquidityAddSuccess {
        amount: (amount_a, amount_b),
        liquidity,
    })
}

fn burn(
    _self: &mut SwapV2MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityRemoveArg>,
    pool_account: &Account,
) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
    let arg = &guard.arg.arg;
    let (token0, token1) = (_self.token0, _self.token1);

    let (_reserve0, _reserve1) = _self.get_reserves(token0, token1);
    let _token0 = token0;
    let _token1 = token1;
    let balance0 = guard.token_balance_of(_token0, *pool_account)?;
    let balance1 = guard.token_balance_of(_token1, *pool_account)?;
    let liquidity_without_fee = arg.liquidity_without_fee.clone();

    let fee_on = mint_fee(_self, guard, pool_account, &_reserve0, &_reserve1)?;
    let _total_supply = _self.lp.get_total_supply();
    let amount0 = liquidity_without_fee.clone() * balance0 / _total_supply.clone();
    let amount1 = liquidity_without_fee.clone() * balance1 / _total_supply.clone();

    // ! check amount before change data
    let arg = &guard.arg.arg;
    if amount0 == *ZERO || amount1 == *ZERO {
        return Err(BusinessError::Liquidity("INSUFFICIENT_LIQUIDITY_BURNED".into()));
    }
    let (amount_a, amount_b) = if arg.token_a == token0 {
        (amount0.clone(), amount1.clone())
    } else {
        (amount1.clone(), amount0.clone())
    };
    if amount_a < arg.amount_a_min {
        return Err(BusinessError::Liquidity("INSUFFICIENT_A_AMOUNT".into()));
    }
    if amount_b < arg.amount_b_min {
        return Err(BusinessError::Liquidity("INSUFFICIENT_B_AMOUNT".into()));
    }

    // do burn，Destroy LP tokens for users
    let arg_from = guard.arg.arg.from;
    let arg_fee = guard.arg.arg.fee.clone();
    _self.lp.burn(
        |token, from, amount_without_fee, fee| {
            guard.token_liquidity_burn(
                &amount_a,
                &amount_b,
                token,
                from,
                *pool_account,
                amount_without_fee, // will burn amount_without_fee + fee and mint fee to fee_to
                fee,
            )
        },
        arg_from,
        liquidity_without_fee.clone(),
        arg_fee, // burn fee to use token fee to
    )?;

    // return token
    let message = format!("*PairLiquidityRemove* `amount0:{amount0}, amount1:{amount1}`");
    let arg = &guard.arg.arg;
    guard.token_transfer(TransferToken {
        token: _token0,
        from: *pool_account,
        amount: amount0,
        to: arg.to,
        fee: None,
    })?; // * transfer and trace
    let arg = &guard.arg.arg;
    guard.token_transfer(TransferToken {
        token: _token1,
        from: *pool_account,
        amount: amount1,
        to: arg.to,
        fee: None,
    })?; // * transfer and trace
    guard.trace(message); // * trace

    let balance0 = guard.token_balance_of(_token0, *pool_account)?;
    let balance1 = guard.token_balance_of(_token1, *pool_account)?;

    // Update the current balance
    update(_self, guard, balance0, balance1, _reserve0, _reserve1)?;
    if fee_on {
        _self.k_last = _self.reserve0.clone() * _self.reserve1.clone();
    }

    Ok(TokenPairLiquidityRemoveSuccess {
        amount: (amount_a, amount_b),
    })
}

pub fn remove_liquidity(
    _self: &mut SwapV2MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityRemoveArg>,
) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
    // ! check balance
    {
        let arg = &guard.arg.arg;
        _self.lp.check_liquidity_removable(
            |token| guard.token_balance_of(token, arg.from),
            &arg.liquidity_without_fee,
            arg.fee.as_ref().map(|fee| fee.fee_to),
        )?;
        guard.trace(format!(
            "*PairLiquidityRemove* `tokenA:[{}], tokenB:[{}], amm:{}, liquidity_without_fee:{}, required: {} <= amount_a && {} <= amount_b`",
            arg.token_a.to_text(),
            arg.token_b.to_text(),
            arg.pa.amm.into_text().as_ref(),
            arg.liquidity_without_fee,
            arg.amount_a_min,
            arg.amount_b_min,
        )); // * trace
    }

    // Transfer token account from the pool
    let arg = &guard.arg.arg;
    let pool_account = Account {
        owner: arg.self_canister.id(),
        subaccount: Some(_self.subaccount),
    };

    burn(_self, guard, &pool_account)
}

fn inner_swap<T: TokenPairArg>(
    _self: &mut SwapV2MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, &T>,
    self_canister: &SelfCanister,
    amount0_out: Nat,
    amount1_out: Nat,
    to: Account,
) -> Result<InnerSwapResult, BusinessError> {
    // Pool's account
    let pool_account = Account {
        owner: self_canister.id(),
        subaccount: Some(_self.subaccount),
    };

    // The output of both tokens cannot be 0
    if amount0_out == *ZERO && amount1_out == *ZERO {
        return Err(BusinessError::Swap("INSUFFICIENT_OUTPUT_AMOUNT".into()));
    }

    // The output of each token cannot be greater than the pool holder
    let (_reserve0, _reserve1) = (_self.reserve0.clone(), _self.reserve1.clone());
    if _reserve0 < amount0_out || _reserve1 < amount1_out {
        return Err(BusinessError::Swap("INSUFFICIENT_LIQUIDITY".into()));
    }

    // do transfer out and fetch balance
    let (balance0, balance1) = {
        let _token0 = _self.token0;
        let _token1 = _self.token1;
        if to.owner == _token0 || to.owner == _token1 {
            return Err(BusinessError::Swap("INVALID_TO".into())); // The output token target address cannot be the token itself
        }
        if amount0_out > *ZERO {
            guard.token_transfer(TransferToken {
                token: _token0,
                from: pool_account,
                amount: amount0_out.clone(),
                to,
                fee: None,
            })?; // * transfer and trace
        }
        if amount1_out > *ZERO {
            guard.token_transfer(TransferToken {
                token: _token1,
                from: pool_account,
                amount: amount1_out.clone(),
                to,
                fee: None,
            })?; // * transfer and trace
        }
        let balance0 = guard.token_balance_of(_token0, pool_account)?;
        let balance1 = guard.token_balance_of(_token1, pool_account)?;
        (balance0, balance1)
    };

    // Calculate the 2 tokens to get the number, and you should transfer it in advance before calling this function.
    let (amount0_in, amount1_in) = {
        let amount0_in = if balance0 > _reserve0.clone() - amount0_out.clone() {
            balance0.clone() - (_reserve0.clone() - amount0_out.clone())
        } else {
            zero()
        };
        let amount1_in = if balance1 > _reserve1.clone() - amount1_out.clone() {
            balance1.clone() - (_reserve1.clone() - amount1_out.clone())
        } else {
            zero()
        };
        (amount0_in, amount1_in)
    };
    // The inputs of both tokens cannot be 0
    if amount0_in == *ZERO && amount1_in == *ZERO {
        return Err(BusinessError::Swap("INSUFFICIENT_INPUT_AMOUNT".into()));
    }

    // check after changed
    {
        let n = _self.fee_rate.numerator;
        let d = _self.fee_rate.denominator;
        let balance0_adjusted = balance0.clone() * d - amount0_in * n;
        let balance1_adjusted = balance1.clone() * d - amount1_in * n;
        if balance0_adjusted * balance1_adjusted < _reserve0.clone() * _reserve1.clone() * d * d {
            // return back
            let _token0 = _self.token0;
            let _token1 = _self.token1;
            if amount0_out > *ZERO {
                guard.token_transfer(TransferToken {
                    token: _token0,
                    from: to,
                    amount: amount0_out.clone(),
                    to: pool_account,
                    fee: None,
                })?; // * transfer and trace
            }
            if amount1_out > *ZERO {
                guard.token_transfer(TransferToken {
                    token: _token1,
                    from: to,
                    amount: amount1_out.clone(),
                    to: pool_account,
                    fee: None,
                })?; // * transfer and trace
            }

            return Err(BusinessError::Swap("K".into()));
        }
    }
    Ok(InnerSwapResult {
        balance: (balance0, balance1),
        _reserve: (_reserve0, _reserve1),
    })
}

/// Be sure to transfer the corresponding token first, and then call this method to transfer the token
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn swap<T: TokenPairArg>(
    _self: &mut SwapV2MarketMaker,
    guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, T>,
    transaction: SwapTransaction,
    trace: String,
    self_canister: &SelfCanister,
    amount0_out: Nat,
    amount1_out: Nat,
    to: Account,
) -> Result<(), BusinessError> {
    let InnerSwapResult {
        balance: (balance0, balance1),
        _reserve: (_reserve0, _reserve1),
    } = guard.mint_swap_block(
        guard.arg.now,
        transaction,
        |guard| inner_swap(_self, guard, self_canister, amount0_out, amount1_out, to),
        trace,
    )?;

    update(_self, guard, balance0, balance1, _reserve0, _reserve1)?;

    Ok(())
}
