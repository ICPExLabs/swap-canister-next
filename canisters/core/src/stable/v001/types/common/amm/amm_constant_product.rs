/// 常数乘积做市商（Constant Product AMM）
/// - 核心公式：x * y = k（x、y 为两资产数量，k 为常数）
/// - 代表项目：Uniswap V2/V3、SushiSwap、PancakeSwap
/// - 特点
///   - 简单高效，适合大多数交易场景。
///   - 大额交易时滑点显著（价格变化剧烈）。
///   - Uniswap V3 引入「集中流动性」，允许 LP 设定价格区间，提升资本效率。
use std::collections::HashMap;

use ::common::utils::principal::sort_tokens;
use num_bigint::BigUint;

use super::*;

use crate::{
    types::{
        AmmText, BusinessError, PoolLp, SelfCanister, SwapRatio, SwapRatioView, TokenBalances,
        TokenInfo, TokenPair, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess,
        TokenPairLiquidityRemoveArg, TokenPairLiquidityRemoveSuccess,
    },
    utils::math::{ZERO, zero},
};

/// 当前算法手续费下，需要的数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapV2MarketMaker {
    subaccount: Subaccount, // ! fixed. 资金余额存放位置 self_canister_id.subaccount
    fee_rate: SwapRatio,    // ! fixed. 交易费率

    token0: CanisterId, // ! 当前 token0 的 canister_id
    token1: CanisterId, // ! 当前 token1 的 canister_id
    reserve0: Nat,      // ! 当前 token0 存入的余额
    reserve1: Nat,      // ! 当前 token0 存入的余额
    block_timestamp_last: u64,

    price_cumulative_exponent: u8, // 指数计算
    price0_cumulative_last: Nat,
    price1_cumulative_last: Nat,
    k_last: Nat, // ! 当前 k 值

    lp: PoolLp, // lp 代币信息, 一旦新建池子成功，除了 supply，其他数据不可更改
    protocol_fee: Option<SwapRatio>, // 协议分享的手续费 和 lp fee 累计应该等于 1
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

    /// 计算另一个代币所需的数量
    fn quote(amount_a: &Nat, reserve_a: &Nat, reserve_b: &Nat) -> Nat {
        assert!(*amount_a > *ZERO, "INSUFFICIENT_AMOUNT");
        assert!(*reserve_a > *ZERO, "INSUFFICIENT_LIQUIDITY");
        assert!(*reserve_b > *ZERO, "INSUFFICIENT_LIQUIDITY");
        amount_a.to_owned() * reserve_b.to_owned() / reserve_a.to_owned()
    }

    /// 计算添加流动性所需的代币数量
    fn inner_add_liquidity(
        &self,
        arg: &TokenPairLiquidityAddArg,
    ) -> Result<(Nat, Nat), BusinessError> {
        let (reserve_a, reserve_b) = self.get_reserves(arg.token_a, arg.token_b);

        if reserve_a == *ZERO && reserve_b == *ZERO {
            Ok((arg.amount_a_desired.clone(), arg.amount_b_desired.clone())) // 第一次添加无需计算
        } else {
            let amount_b_optimal = Self::quote(&arg.amount_a_desired, &reserve_a, &reserve_b); // 以 a 计算 b 的数量
            if amount_b_optimal <= arg.amount_b_desired {
                if amount_b_optimal < arg.amount_b_min {
                    // 以 a 数量所需的 b 太少，小于最少的 b
                    return Err(BusinessError::Liquidity("INSUFFICIENT_B_AMOUNT".into()));
                }
                Ok((arg.amount_a_desired.clone(), amount_b_optimal))
            } else {
                // 以 a 数量所需的 b 太多，大于最大的 b

                // 切换另一个代币计算
                let amount_a_optimal = Self::quote(&arg.amount_b_desired, &reserve_b, &reserve_a); // 以 b 计算 a 的数量
                if amount_a_optimal > arg.amount_a_desired || amount_a_optimal < arg.amount_a_min {
                    // 以 b 数量所需的 a 太多，大于最大的 a
                    // 以 b 数量所需的 a 太少，小于最少的 a
                    return Err(BusinessError::Liquidity("INSUFFICIENT_A_AMOUNT".into()));
                }
                Ok((amount_a_optimal, arg.amount_b_desired.clone()))
            }
        }
    }

    fn mint_fee(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg>,
        _reserve0: &Nat,
        _reserve1: &Nat,
    ) -> Result<bool, BusinessError> {
        let fee_to = guard.fee_to;
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

    fn update<T>(
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

    /// 铸造流动性
    fn mint(
        &mut self,
        guard: &mut InnerTokenPairSwapGuard<'_, '_, '_, TokenPairLiquidityAddArg>,
        amount_a: &Nat,
        amount_b: &Nat,
        pool_account: &Account,
    ) -> Result<Nat, BusinessError> {
        let (token0, token1) = (self.token0, self.token1);

        // 获取当前持有
        let (_reserve0, _reserve1) = self.get_reserves(token0, token1);
        // 获取当前余额
        let balance0 = guard.token_balance_of(token0, *pool_account)?;
        let balance1 = guard.token_balance_of(token1, *pool_account)?;
        // 计算增加额度
        let amount0 = balance0.clone() - _reserve0.clone();
        let amount1 = balance1.clone() - _reserve1.clone();

        // 收取手续费，铸造成 LP 代币，然后才能计算新增流动性
        let fee_on = self.mint_fee(guard, &_reserve0, &_reserve1)?;
        // 计算增加的流动性
        let _total_supply = self.lp.get_total_supply();
        let liquidity = if _total_supply == *ZERO {
            Nat::from((amount0 * amount1).0.sqrt())
        } else {
            let liquidity0 = amount0 * _total_supply.clone() / _reserve0.clone();
            let liquidity1 = amount1 * _total_supply.clone() / _reserve1.clone();
            liquidity0.min(liquidity1)
        };

        // do mint，为用户铸造 LP 代币
        self.lp.mint(
            guard,
            amount_a,
            amount_b,
            guard.arg.arg.to,
            liquidity.clone(),
        )?;

        // 更新当前余额
        self.update(guard, balance0, balance1, _reserve0, _reserve1)?;
        if fee_on {
            self.k_last = self.reserve0.clone() * self.reserve1.clone(); // 记录当前 k
        }

        // ! push log

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
                "Add Liquidity | TokenA: [{}] TokenB: [{}] Amm: {} | {} <= amount_a <= {} and {} <= amount_b <= {}",
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
            "Pending | amount_a: {amount_a} amount_b: {amount_b}",
        )); // * trace
        // 池子接收账户
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
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        self.lp.check_liquidity_removable(
            |token| token_balances.token_balance_of(token, *from),
            liquidity,
        )
    }

    fn burn(
        &mut self,
        fee_to: Option<Account>,
        guard: &mut TokenBalancesGuard,
        pool_account: &Account,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        todo!()
        // let (token0, token1) = (self.token0, self.token1);

        // let (_reserve0, _reserve1) = self.get_reserves(token0, token1);
        // let _token0 = token0;
        // let _token1 = token1;
        // let balance0 = guard.token_balance_of(_token0, *pool_account)?;
        // let balance1 = guard.token_balance_of(_token1, *pool_account)?;
        // let liquidity = arg.liquidity;

        // let fee_on = self.mint_fee(fee_to, guard, &_reserve0, &_reserve1)?;
        // let _total_supply = self.lp.get_total_supply();
        // let amount0 = liquidity.clone() * balance0 / _total_supply.clone();
        // let amount1 = liquidity.clone() * balance1 / _total_supply.clone();

        // // ! check amount before change data
        // if amount0 == *ZERO || amount1 == *ZERO {
        //     return Err(BusinessError::Liquidity(
        //         "INSUFFICIENT_LIQUIDITY_BURNED".into(),
        //     ));
        // }
        // let (amount_a, amount_b) = if arg.token_a == token0 {
        //     (amount0.clone(), amount1.clone())
        // } else {
        //     (amount1.clone(), amount0.clone())
        // };
        // if amount_a < arg.amount_a_min {
        //     return Err(BusinessError::Liquidity("INSUFFICIENT_A_AMOUNT".into()));
        // }
        // if amount_b < arg.amount_b_min {
        //     return Err(BusinessError::Liquidity("INSUFFICIENT_B_AMOUNT".into()));
        // }

        // // do burn
        // self.lp.burn(guard, arg.from, liquidity.clone())?;

        // // return token
        // guard.token_transfer(_token0, *pool_account, arg.to, amount0)?;
        // guard.token_transfer(_token1, *pool_account, arg.to, amount1)?;

        // let balance0 = guard.token_balance_of(_token0, *pool_account)?;
        // let balance1 = guard.token_balance_of(_token1, *pool_account)?;

        // self.update(balance0, balance1, _reserve0, _reserve1);
        // if fee_on {
        //     self.k_last = self.reserve0.clone() * self.reserve1.clone();
        // }

        // // ! push log

        // Ok(TokenPairLiquidityRemoveSuccess {
        //     amount: (amount_a, amount_b),
        // })
    }

    pub fn remove_liquidity(
        &mut self,
        fee_to: Option<Account>,
        guard: &mut TokenBalancesGuard,
        self_canister: &SelfCanister,
        arg: TokenPairLiquidityRemoveArg,
    ) -> Result<TokenPairLiquidityRemoveSuccess, BusinessError> {
        // ! check balance
        {
            self.lp.check_liquidity_removable(
                |token| guard.token_balance_of(token, arg.from),
                &arg.liquidity,
            )?;
        }

        let pool_account = Account {
            owner: self_canister.id(),
            subaccount: Some(self.subaccount),
        };

        self.burn(fee_to, guard, &pool_account, arg)
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

        let amount_out = numerator / denominator; // ! 转出可以少，向下取整

        // 检查转出余额是否足够
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

        let amount_in = (numerator / denominator) + 1_u32; // ! 转入不可以少，向上取整

        // 检查转出余额是否足够
        if (token_out == self.token0 && self.reserve0 < *amount_out)
            || (token_out == self.token1 && self.reserve1 < *amount_out)
        {
            return Err(BusinessError::Swap("INSUFFICIENT_LIQUIDITY".into()));
        }

        // check K on calculate amount
        self.check_k_on_calculate_amount(&token_out, &amount_in, amount_out)?;

        Ok((pool_account, amount_in))
    }

    pub fn swap(
        &mut self,
        guard: &mut TokenBalancesGuard,
        self_canister: &SelfCanister,
        amount0_out: Nat,
        amount1_out: Nat,
        to: Account,
    ) -> Result<(), BusinessError> {
        todo!()

        // let pool_account = Account {
        //     owner: self_canister.id(),
        //     subaccount: Some(self.subaccount),
        // };

        // if amount0_out == *ZERO && amount1_out == *ZERO {
        //     return Err(BusinessError::Swap("INSUFFICIENT_OUTPUT_AMOUNT".into()));
        // }

        // let (_reserve0, _reserve1) = (self.reserve0.clone(), self.reserve1.clone());
        // if _reserve0 < amount0_out || _reserve1 < amount1_out {
        //     return Err(BusinessError::Swap("INSUFFICIENT_LIQUIDITY".into()));
        // }

        // // do transfer out and fetch balance
        // let (balance0, balance1) = {
        //     let _token0 = self.token0;
        //     let _token1 = self.token1;
        //     if to.owner == _token0 || to.owner == _token1 {
        //         return Err(BusinessError::Swap("INVALID_TO".into()));
        //     }
        //     if amount0_out > *ZERO {
        //         guard.token_transfer(_token0, pool_account, to, amount0_out.clone())?;
        //     }
        //     if amount1_out > *ZERO {
        //         guard.token_transfer(_token1, pool_account, to, amount1_out.clone())?;
        //     }
        //     let balance0 = guard.token_balance_of(_token0, pool_account)?;
        //     let balance1 = guard.token_balance_of(_token1, pool_account)?;
        //     (balance0, balance1)
        // };

        // // get in
        // let (amount0_in, amount1_in) = {
        //     let amount0_in = if balance0 > _reserve0.clone() - amount0_out.clone() {
        //         balance0.clone() - (_reserve0.clone() - amount0_out.clone())
        //     } else {
        //         zero()
        //     };
        //     let amount1_in = if balance1 > _reserve1.clone() - amount1_out.clone() {
        //         balance1.clone() - (_reserve1.clone() - amount1_out.clone())
        //     } else {
        //         zero()
        //     };
        //     (amount0_in, amount1_in)
        // };
        // if amount0_in == *ZERO && amount1_in == *ZERO {
        //     return Err(BusinessError::Swap("INSUFFICIENT_INPUT_AMOUNT".into()));
        // }

        // // check after changed
        // {
        //     let n = self.fee_rate.numerator;
        //     let d = self.fee_rate.denominator;
        //     let balance0_adjusted = balance0.clone() * d - amount0_in * n;
        //     let balance1_adjusted = balance1.clone() * d - amount1_in * n;
        //     if balance0_adjusted * balance1_adjusted < _reserve0.clone() * _reserve1.clone() * d * d
        //     {
        //         // return back
        //         let _token0 = self.token0;
        //         let _token1 = self.token1;
        //         if amount0_out > *ZERO {
        //             guard.token_transfer(_token0, to, pool_account, amount0_out.clone())?;
        //         }
        //         if amount1_out > *ZERO {
        //             guard.token_transfer(_token1, to, pool_account, amount1_out.clone())?;
        //         }

        //         return Err(BusinessError::Swap("K".into()));
        //     }
        // }

        // self.update(balance0, balance1, _reserve0, _reserve1);

        // // ! push log

        // Ok(())
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
