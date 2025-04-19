/// Constant product market maker（Constant Product AMM）
/// - Core formula：x * y = k（x and y are the number of two assets, and k is the constant）
/// - Representative Project：Uniswap V2/V3、SushiSwap、PancakeSwap
/// - Features
///   - Simple and efficient, suitable for most trading scenarios.
///   - There is significant slippage in large-scale transactions (significant price changes).
///   - Uniswap V3 introduces "concentrated liquidity", allowing LPs to set price ranges and improve capital efficiency.
use std::collections::HashMap;

use ::common::utils::principal::sort_tokens;
use num_bigint::BigUint;

use super::*;

use crate::{
    types::{
        BusinessError, PoolLp, SelfCanister, SwapRatio, SwapRatioView, TokenBalances, TokenInfo,
        TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess, TokenPairLiquidityRemoveArg,
        TokenPairLiquidityRemoveSuccess,
    },
    utils::math::{ZERO, zero},
};

/// The required data under the current algorithm processing fee
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapV2MarketMaker {
    subaccount: Subaccount, // ! fixed. Fund balance storage location self_canister_id.subaccount
    fee_rate: SwapRatio,    // ! fixed. Transaction rates

    token0: CanisterId, // ! Canister_id of the current token0
    token1: CanisterId, // ! Canister_id of the current token1
    reserve0: Nat,      // ! The current balance deposited by token0
    reserve1: Nat,      // ! The current balance deposited by token1
    block_timestamp_last: u64,

    price_cumulative_exponent: u8, // Exponential calculation
    price0_cumulative_last: Nat,
    price1_cumulative_last: Nat,
    k_last: Nat, // ! Current k value

    lp: PoolLp, // lp token information, Once the new pool is successfully created, other data cannot be changed except for supply
    protocol_fee: Option<SwapRatio>, // The processing fee and lp fee for the agreement sharing should be equal to 1
}

impl SwapV2MarketMaker {
    pub fn new(
        subaccount: Subaccount,
        fee_rate: SwapRatio,
        token0: CanisterId,
        token1: CanisterId,
        lp: PoolLp,
        protocol_fee: Option<SwapRatio>,
    ) -> Self {
        Self {
            subaccount,
            fee_rate,
            token0,
            token1,
            reserve0: zero(),
            reserve1: zero(),
            block_timestamp_last: 0,
            price_cumulative_exponent: 64,
            price0_cumulative_last: zero(),
            price1_cumulative_last: zero(),
            k_last: zero(),
            lp,
            protocol_fee,
        }
    }

    pub fn price_cumulative_unit(&self) -> Nat {
        let price_cumulative_unit = BigUint::from(2_u8).pow(self.price_cumulative_exponent as u32);
        Nat::from(price_cumulative_unit)
    }

    pub fn dummy_tokens(
        &self,
        tokens: &HashMap<CanisterId, TokenInfo>,
        pa: &TokenPairAmm,
    ) -> Vec<TokenInfo> {
        self.lp.dummy_tokens(tokens, pa)
    }

    pub fn accounts(&self, self_canister: &SelfCanister) -> Vec<Account> {
        vec![Account {
            owner: self_canister.id(),
            subaccount: Some(self.subaccount),
        }]
    }

    pub fn dummy_canisters(&self) -> Vec<CanisterId> {
        self.lp.dummy_canisters()
    }

    // fetches and sorts the reserves for a pair
    fn get_reserves(&self, token_a: CanisterId, token_b: CanisterId) -> (Nat, Nat) {
        let (token0, _) = sort_tokens(token_a, token_b);
        let (reserve0, reserve1) = (self.reserve0.clone(), self.reserve1.clone());
        if token_a == token0 {
            (reserve0, reserve1)
        } else {
            (reserve1, reserve0)
        }
    }

    /// Calculate the amount required for another token
    fn quote(amount_a: &Nat, reserve_a: &Nat, reserve_b: &Nat) -> Nat {
        assert!(*amount_a > *ZERO, "INSUFFICIENT_AMOUNT");
        assert!(*reserve_a > *ZERO, "INSUFFICIENT_LIQUIDITY");
        assert!(*reserve_b > *ZERO, "INSUFFICIENT_LIQUIDITY");
        amount_a.to_owned() * reserve_b.to_owned() / reserve_a.to_owned()
    }

    /// Calculate the number of tokens required to add liquidity
    fn inner_add_liquidity(
        &self,
        arg: &TokenPairLiquidityAddArg,
    ) -> Result<(Nat, Nat), BusinessError> {
        let (reserve_a, reserve_b) = self.get_reserves(arg.token_a, arg.token_b);

        if reserve_a == *ZERO && reserve_b == *ZERO {
            Ok((arg.amount_a_desired.clone(), arg.amount_b_desired.clone())) // No calculation required for the first time
        } else {
            let amount_b_optimal = Self::quote(&arg.amount_a_desired, &reserve_a, &reserve_b); // Calculate the number of b with a
            if amount_b_optimal <= arg.amount_b_desired {
                if amount_b_optimal < arg.amount_b_min {
                    // Too few b required in a quantity of a, less than the minimum b
                    return Err(BusinessError::Liquidity("INSUFFICIENT_B_AMOUNT".into()));
                }
                Ok((arg.amount_a_desired.clone(), amount_b_optimal))
            } else {
                // Too much b is required in a quantity of a, greater than the maximum b

                // Switch to another token calculation
                let amount_a_optimal = Self::quote(&arg.amount_b_desired, &reserve_b, &reserve_a); // Calculate the number of a with b
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
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, T>,
        _reserve0: &Nat,
        _reserve1: &Nat,
    ) -> Result<bool, BusinessError> {
        let fee_to = guard.fee_to.swap_fee_to;
        let fee_on =
            fee_to.is_some() && self.protocol_fee.as_ref().is_some_and(|fee| !fee.is_zero());

        let _k_last = self.k_last.clone();
        if fee_on {
            if _k_last != *ZERO {
                let root_k = Nat::from((_reserve0.to_owned() * _reserve1.to_owned()).0.sqrt());
                let root_k_last = Nat::from(_k_last.0.sqrt());
                if root_k > root_k_last {
                    if let (Some(fee_to), Some(protocol_fee)) = (fee_to, &self.protocol_fee) {
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
                        let total_supply = self.lp.get_total_supply();
                        let numerator =
                            (root_k.clone() - root_k_last.clone()) * total_supply * n.clone();
                        let denominator = (d - n.clone()) * root_k + n * root_k_last;
                        let liquidity = numerator / denominator;
                        if liquidity > *ZERO {
                            self.lp.mint_fee(guard, fee_to, liquidity)?;
                        }
                    }
                }
            }
        } else if _k_last != *ZERO {
            self.k_last = zero()
        }

        Ok(fee_on)
    }

    fn update<T: TokenPairArg>(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, T>,
        balance0: Nat,
        balance1: Nat,
        _reserve0: Nat,
        _reserve1: Nat,
    ) -> Result<(), BusinessError> {
        let block_timestamp = guard.arg.now.into_inner();
        let time_elapsed = block_timestamp - self.block_timestamp_last;
        if time_elapsed > 0 && _reserve0 > *ZERO && _reserve1 > *ZERO {
            let e = Nat::from(time_elapsed);
            let price_cumulative_unit = self.price_cumulative_unit();
            self.price0_cumulative_last +=
                e.clone() * _reserve1.clone() * price_cumulative_unit.clone() / _reserve0.clone();
            self.price1_cumulative_last +=
                e.clone() * _reserve0.clone() * price_cumulative_unit.clone() / _reserve1.clone();

            guard.mint_cumulative_price(
                self.price_cumulative_exponent,
                self.price0_cumulative_last.clone(),
                self.price1_cumulative_last.clone(),
            )?;
        }
        self.reserve0 = balance0;
        self.reserve1 = balance1;
        self.block_timestamp_last = block_timestamp;
        Ok(())
    }

    /// mint liquidity
    fn mint(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg>,
        amount_a: &Nat,
        amount_b: &Nat,
        pool_account: &Account,
    ) -> Result<Nat, BusinessError> {
        let (token0, token1) = (self.token0, self.token1);

        // Get the current hold
        let (_reserve0, _reserve1) = self.get_reserves(token0, token1);
        // Get the current balance
        let balance0 = guard.token_balance_of(token0, *pool_account)?;
        let balance1 = guard.token_balance_of(token1, *pool_account)?;
        // Calculate the increase amount
        let amount0 = balance0.clone() - _reserve0.clone();
        let amount1 = balance1.clone() - _reserve1.clone();

        // Charge a handling fee to mint LP tokens before the additional liquidity can be calculated
        let fee_on = self.mint_fee(guard, &_reserve0, &_reserve1)?;
        // Calculate increased liquidity
        let _total_supply = self.lp.get_total_supply();
        let liquidity = if _total_supply == *ZERO {
            Nat::from((amount0 * amount1).0.sqrt())
        } else {
            let liquidity0 = amount0 * _total_supply.clone() / _reserve0.clone();
            let liquidity1 = amount1 * _total_supply.clone() / _reserve1.clone();
            liquidity0.min(liquidity1)
        };

        // do mint，Mint LP tokens for users
        let arg = &guard.arg.arg;
        self.lp
            .mint(guard, amount_a, amount_b, arg.to, liquidity.clone())?;

        // Update the current balance
        self.update(guard, balance0, balance1, _reserve0, _reserve1)?;
        if fee_on {
            self.k_last = self.reserve0.clone() * self.reserve1.clone(); // Record the current k
        }

        Ok(liquidity)
    }

    pub fn add_liquidity(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg>,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        // ! check balance
        {
            let arg = &guard.arg.arg;
            guard.assert_token_balance(arg.token_a, arg.from, &arg.amount_a_desired)?;
            guard.assert_token_balance(arg.token_b, arg.from, &arg.amount_b_desired)?;
            guard.trace(format!(
                "*Add Liquidity* `tokenA:[{}], tokenB:[{}], amm:{}, required: {} <= amount_a <= {} && {} <= amount_b <= {}`",
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
        let (amount_a, amount_b) = self.inner_add_liquidity(arg)?;
        guard.trace(format!(
            "*Add Liquidity* `amount_a:{amount_a}, amount_b:{amount_b}`",
        )); // * trace
        // Pool token account
        let arg = &guard.arg.arg;
        let pool_account = Account {
            owner: arg.self_canister.id(),
            subaccount: Some(self.subaccount),
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
        let liquidity = self.mint(guard, &amount_a, &amount_b, &pool_account)?;
        Ok(TokenPairLiquidityAddSuccess {
            amount: (amount_a, amount_b),
            liquidity,
        })
    }

    pub fn check_liquidity_removable(
        &self,
        token_balances: &TokenBalances,
        from: &Account,
        liquidity_without_fee: &Nat,
    ) -> Result<(), BusinessError> {
        self.lp.check_liquidity_removable(
            |token| token_balances.token_balance_of(token, *from),
            liquidity_without_fee,
        )
    }

    fn burn(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityRemoveArg>,
        pool_account: &Account,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        let arg = &guard.arg.arg;
        let (token0, token1) = (self.token0, self.token1);

        let (_reserve0, _reserve1) = self.get_reserves(token0, token1);
        let _token0 = token0;
        let _token1 = token1;
        let balance0 = guard.token_balance_of(_token0, *pool_account)?;
        let balance1 = guard.token_balance_of(_token1, *pool_account)?;
        let liquidity = arg.liquidity_without_fee.clone();

        let fee_on = self.mint_fee(guard, &_reserve0, &_reserve1)?;
        let _total_supply = self.lp.get_total_supply();
        let amount0 = liquidity.clone() * balance0 / _total_supply.clone();
        let amount1 = liquidity.clone() * balance1 / _total_supply.clone();

        // ! check amount before change data
        let arg = &guard.arg.arg;
        if amount0 == *ZERO || amount1 == *ZERO {
            return Err(BusinessError::Liquidity(
                "INSUFFICIENT_LIQUIDITY_BURNED".into(),
            ));
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
        let arg = &guard.arg.arg;
        self.lp.burn(
            guard,
            &amount_a,
            &amount_b,
            arg.from,
            liquidity.clone(),
            guard.fee_to.token_fee_to, // burn fee to use token fee to
        )?;

        // return token
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

        let balance0 = guard.token_balance_of(_token0, *pool_account)?;
        let balance1 = guard.token_balance_of(_token1, *pool_account)?;

        // Update the current balance
        self.update(guard, balance0, balance1, _reserve0, _reserve1)?;
        if fee_on {
            self.k_last = self.reserve0.clone() * self.reserve1.clone();
        }

        Ok(TokenPairLiquidityRemoveSuccess {
            amount: (amount_a, amount_b),
        })
    }

    pub fn remove_liquidity(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityRemoveArg>,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        // ! check balance
        {
            let arg = &guard.arg.arg;
            self.lp.check_liquidity_removable(
                |token| guard.token_balance_of(token, arg.from),
                &arg.liquidity_without_fee,
            )?;
            guard.trace(format!(
                "*Remove Liquidity* `tokenA:[{}], tokenB:[{}], amm:{}, liquidity_without_fee:{}, required: {} <= amount_a && {} <= amount_b`",
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
            subaccount: Some(self.subaccount),
        };

        self.burn(guard, &pool_account)
    }

    fn check_k_on_calculate_amount(
        &self,
        token_out: &CanisterId,
        amount_in: &Nat,
        amount_out: &Nat,
    ) -> Result<(), BusinessError> {
        let n = self.fee_rate.numerator;
        let d = self.fee_rate.denominator;
        let (balance0, balance1, amount0_in, amount1_in) = if *token_out == self.token1 {
            (
                self.reserve0.clone() + amount_in.clone(),
                self.reserve1.clone() - amount_out.clone(),
                amount_in.clone(),
                zero(),
            )
        } else {
            (
                self.reserve0.clone() - amount_out.clone(),
                self.reserve1.clone() + amount_in.clone(),
                zero(),
                amount_in.clone(),
            )
        };
        let balance0_adjusted = balance0.clone() * d - amount0_in * n;
        let balance1_adjusted = balance1.clone() * d - amount1_in * n;
        if balance0_adjusted * balance1_adjusted
            < self.reserve0.clone() * self.reserve1.clone() * d * d
        {
            return Err(BusinessError::Swap("K".into()));
        }

        Ok(())
    }

    // given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    pub fn get_amount_out(
        &self,
        self_canister: &SelfCanister,
        amount_in: &Nat,
        token_in: CanisterId,
        token_out: CanisterId,
    ) -> Result<(Account, Nat), BusinessError> {
        let pool_account = Account {
            owner: self_canister.id(),
            subaccount: Some(self.subaccount),
        };

        let (reserve_in, reserve_out) = self.get_reserves(token_in, token_out);

        // check
        if *amount_in == *ZERO {
            return Err(BusinessError::Swap("INSUFFICIENT_INPUT_AMOUNT".into()));
        }
        if reserve_in == *ZERO || reserve_out == *ZERO {
            return Err(BusinessError::Swap("INSUFFICIENT_LIQUIDITY".into()));
        }

        // (in + amount_in) * (out - amount_out) = in * out
        // amount_out = (in * out) / (in + amount_in) - out
        // amount_out = (out * amount_in) / (in + amount_in)
        // amount_out = (out * (amount_in * (1-r))) / (in + (amount_in * (1-r)))

        //              (out * (amount_in * (1-n/d)))
        // amount_out = -----------------------------
        //              (in + (amount_in * (1-n/d)))

        //              out * amount_in * (d-n)
        // amount_out = -----------------------------
        //              in * d + amount_in * (d-n)

        let n = Nat::from(self.fee_rate.numerator);
        let d = Nat::from(self.fee_rate.denominator);
        let amount_in_with_fee = amount_in.clone() * (d.clone() - n);
        let numerator = reserve_out * amount_in_with_fee.clone();
        let denominator = reserve_in * d + amount_in_with_fee;

        let amount_out = numerator / denominator; // ! You can turn out less and get downward

        // Check if the transfer balance is sufficient
        if (token_out == self.token0 && self.reserve0 < amount_out)
            || (token_out == self.token1 && self.reserve1 < amount_out)
        {
            return Err(BusinessError::Swap("INSUFFICIENT_LIQUIDITY".into()));
        }

        // check K on calculate amount
        self.check_k_on_calculate_amount(&token_out, amount_in, &amount_out)?;

        Ok((pool_account, amount_out))
    }

    // given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    pub fn get_amount_in(
        &self,
        self_canister: &SelfCanister,
        amount_out: &Nat,
        token_in: CanisterId,
        token_out: CanisterId,
    ) -> Result<(Account, Nat), BusinessError> {
        let pool_account = Account {
            owner: self_canister.id(),
            subaccount: Some(self.subaccount),
        };

        let (reserve_in, reserve_out) = self.get_reserves(token_in, token_out);

        // check
        if *amount_out == *ZERO {
            return Err(BusinessError::Swap("INSUFFICIENT_OUTPUT_AMOUNT".into()));
        }
        if reserve_in == *ZERO || reserve_out == *ZERO {
            return Err(BusinessError::Swap("INSUFFICIENT_LIQUIDITY".into()));
        }

        // (in + amount_in) * (out - amount_out) = in * out
        // amount_in = (in * out) / (out - amount_out) - in
        // amount_in = (in * amount_out) / (out - amount_out)
        // amount_in * (1-r) = (in * amount_out) / (out - amount_out)
        // amount_in * (1-n/d) = (in * amount_out) / (out - amount_out)
        // amount_in * (d-n)/d = (in * amount_out) / (out - amount_out)
        // amount_in = (in * amount_out) * d / ((out - amount_out) *(d-n))

        //               in * amount_out * d
        // amount_in = -----------------------------
        //              (out - amount_out) * (d - n)

        let n = Nat::from(self.fee_rate.numerator);
        let d = Nat::from(self.fee_rate.denominator);
        let numerator = reserve_in * amount_out.clone() * d.clone();
        let denominator = (reserve_out - amount_out.clone()) * (d - n);

        let amount_in = (numerator / denominator) + 1_u32; // ! You must not miss the transfer, and you can get it upward

        // Check if the transfer balance is sufficient
        if (token_out == self.token0 && self.reserve0 < *amount_out)
            || (token_out == self.token1 && self.reserve1 < *amount_out)
        {
            return Err(BusinessError::Swap("INSUFFICIENT_LIQUIDITY".into()));
        }

        // check K on calculate amount
        self.check_k_on_calculate_amount(&token_out, &amount_in, amount_out)?;

        Ok((pool_account, amount_in))
    }

    /// Be sure to transfer the corresponding token first, and then call this method to transfer the token
    pub fn swap<T: TokenPairArg>(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, T>,
        self_canister: &SelfCanister,
        amount0_out: Nat,
        amount1_out: Nat,
        to: Account,
    ) -> Result<(), BusinessError> {
        // Pool's account
        let pool_account = Account {
            owner: self_canister.id(),
            subaccount: Some(self.subaccount),
        };

        // The output of both tokens cannot be 0
        if amount0_out == *ZERO && amount1_out == *ZERO {
            return Err(BusinessError::Swap("INSUFFICIENT_OUTPUT_AMOUNT".into()));
        }

        // The output of each token cannot be greater than the pool holder
        let (_reserve0, _reserve1) = (self.reserve0.clone(), self.reserve1.clone());
        if _reserve0 < amount0_out || _reserve1 < amount1_out {
            return Err(BusinessError::Swap("INSUFFICIENT_LIQUIDITY".into()));
        }

        // do transfer out and fetch balance
        let (balance0, balance1) = {
            let _token0 = self.token0;
            let _token1 = self.token1;
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
            let n = self.fee_rate.numerator;
            let d = self.fee_rate.denominator;
            let balance0_adjusted = balance0.clone() * d - amount0_in * n;
            let balance1_adjusted = balance1.clone() * d - amount1_in * n;
            if balance0_adjusted * balance1_adjusted < _reserve0.clone() * _reserve1.clone() * d * d
            {
                // return back
                let _token0 = self.token0;
                let _token1 = self.token1;
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

        self.update(guard, balance0, balance1, _reserve0, _reserve1)?;

        Ok(())
    }
}

// ========================== view ==========================

#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct SwapV2MarketMakerView {
    subaccount: String,
    fee_rate: SwapRatioView,

    token0: String,
    token1: String,
    reserve0: String,
    reserve1: String,
    block_timestamp_last: u64,

    price_cumulative_exponent: u8,
    price0_cumulative_last: String,
    price1_cumulative_last: String,
    k_last: String,

    lp: PoolLp,
    protocol_fee: Option<SwapRatioView>,
}

impl From<SwapV2MarketMaker> for SwapV2MarketMakerView {
    fn from(value: SwapV2MarketMaker) -> Self {
        Self {
            subaccount: hex::encode(value.subaccount),
            fee_rate: value.fee_rate.into(),
            token0: value.token0.to_string(),
            token1: value.token1.to_string(),
            reserve0: value.reserve0.to_string(),
            reserve1: value.reserve1.to_string(),
            block_timestamp_last: value.block_timestamp_last,
            price_cumulative_exponent: value.price_cumulative_exponent,
            price0_cumulative_last: value.price0_cumulative_last.to_string(),
            price1_cumulative_last: value.price1_cumulative_last.to_string(),
            k_last: value.k_last.to_string(),
            lp: value.lp,
            protocol_fee: value.protocol_fee.map(|f| f.into()),
        }
    }
}
