/// 常数乘积做市商（Constant Product AMM）
/// - 核心公式：x * y = k（x、y 为两资产数量，k 为常数）
/// - 代表项目：Uniswap V2/V3、SushiSwap、PancakeSwap
/// - 特点
///   - 简单高效，适合大多数交易场景。
///   - 大额交易时滑点显著（价格变化剧烈）。
///   - Uniswap V3 引入「集中流动性」，允许 LP 设定价格区间，提升资本效率。
use candid::{CandidType, Nat};
use ic_canister_kit::{times::now, types::CanisterId};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::{Deserialize, Serialize};

use crate::{
    types::{
        BusinessError, PairAmm, PoolLp, SelfCanister, SwapRatio, SwapRatioView, TokenBalances,
        TokenPair, TokenPairLiquidityAddArg, TokenPairLiquidityAddSuccess,
    },
    utils::{
        math::{ZERO, zero},
        principal::sort_tokens,
    },
};

/// 当前算法手续费下，需要的数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SwapV2MarketMaker {
    subaccount: Subaccount, // ! fixed. 资金余额存放位置 self_canister_id.subaccount
    fee_rate: SwapRatio,    // ! fixed. 交易费率

    reserve0: Nat, // ! 当前 token0 存入的余额
    reserve1: Nat, // ! 当前 token0 存入的余额
    block_timestamp_last: u64,

    price_unit: Nat,
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
        lp: PoolLp,
        protocol_fee: Option<SwapRatio>,
    ) -> Self {
        Self {
            subaccount,
            fee_rate,
            reserve0: zero(),
            reserve1: zero(),
            block_timestamp_last: 0,
            price_unit: candid::Nat::from(u64::MAX),
            price0_cumulative_last: zero(),
            price1_cumulative_last: zero(),
            k_last: zero(),
            lp,
            protocol_fee,
        }
    }

    pub fn accounts(&self, self_canister: &SelfCanister) -> Vec<Account> {
        vec![Account {
            owner: self_canister.id(),
            subaccount: Some(self.subaccount),
        }]
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

    fn quote(amount_a: &Nat, reserve_a: &Nat, reserve_b: &Nat) -> Nat {
        assert!(*amount_a > *ZERO, "INSUFFICIENT_AMOUNT");
        assert!(*reserve_a > *ZERO, "INSUFFICIENT_LIQUIDITY");
        assert!(*reserve_a > *ZERO, "INSUFFICIENT_LIQUIDITY");
        amount_a.to_owned() * reserve_b.to_owned() / reserve_a.to_owned()
    }

    fn inner_add_liquidity(
        &self,
        arg: &TokenPairLiquidityAddArg,
    ) -> Result<(Nat, Nat), BusinessError> {
        let (reserve_a, reserve_b) = self.get_reserves(arg.token_a, arg.token_b);

        if reserve_a == *ZERO && reserve_b == *ZERO {
            Ok((arg.amount_a_desired.clone(), arg.amount_b_desired.clone()))
        } else {
            let amount_b_optimal = Self::quote(&arg.amount_a_desired, &reserve_a, &reserve_b);
            if amount_b_optimal <= arg.amount_b_desired {
                if amount_b_optimal < arg.amount_b_min {
                    return Err(BusinessError::Liquidity("INSUFFICIENT_B_AMOUNT".into()));
                }
                Ok((arg.amount_a_desired.clone(), amount_b_optimal))
            } else {
                let amount_a_optimal = Self::quote(&arg.amount_b_desired, &reserve_b, &reserve_a);
                if arg.amount_a_desired < amount_a_optimal || amount_a_optimal < arg.amount_a_min {
                    return Err(BusinessError::Liquidity("INSUFFICIENT_A_AMOUNT".into()));
                }
                Ok((amount_a_optimal, arg.amount_b_desired.clone()))
            }
        }
    }

    fn mint_fee(
        &mut self,
        fee_to: Option<Account>,
        token_balances: &mut TokenBalances,
        _reserve0: &Nat,
        _reserve1: &Nat,
    ) -> bool {
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
                        let numerator = (root_k.clone() - root_k_last.clone())
                            * self.lp.get_total_supply()
                            * n.clone();
                        let denominator = (d - n.clone()) * root_k + n * root_k_last;
                        let liquidity = numerator / denominator;
                        if liquidity > *ZERO {
                            self.lp.mint(token_balances, fee_to, liquidity);
                        }
                    }
                }
            }
        } else if _k_last != *ZERO {
            self.k_last = zero()
        }

        fee_on
    }

    fn update(&mut self, balance0: Nat, balance1: Nat, _reserve0: Nat, _reserve1: Nat) {
        let block_timestamp = now().into_inner() as u64;
        let time_elapsed = block_timestamp - self.block_timestamp_last;
        if time_elapsed > 0 && _reserve0 > *ZERO && _reserve1 > *ZERO {
            let e = Nat::from(time_elapsed);
            self.price0_cumulative_last +=
                e.clone() * _reserve1.clone() * self.price_unit.clone() / _reserve0.clone();
            self.price1_cumulative_last +=
                e.clone() * _reserve0.clone() * self.price_unit.clone() / _reserve1.clone();
        }
        self.reserve0 = balance0;
        self.reserve1 = balance1;
        self.block_timestamp_last = block_timestamp;

        // ! push log for record cumulative price
    }

    fn mint(
        &mut self,
        fee_to: Option<Account>,
        token_balances: &mut TokenBalances,
        pa: PairAmm,
        pool_account: &Account,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<Nat, BusinessError> {
        let TokenPair { token0, token1 } = pa.pair;

        let (_reserve0, _reserve1) = self.get_reserves(token0, token1);
        let balance0 = token_balances.token_balance_of(token0, *pool_account);
        let balance1 = token_balances.token_balance_of(token1, *pool_account);
        let amount0 = balance0.clone() - _reserve0.clone();
        let amount1 = balance1.clone() - _reserve1.clone();

        let fee_on = self.mint_fee(fee_to, token_balances, &_reserve0, &_reserve1);
        let _total_supply = self.lp.get_total_supply();
        let liquidity = if _total_supply == *ZERO {
            Nat::from((amount0 * amount1).0.sqrt())
        } else {
            let liquidity0 = amount0 * _total_supply.clone() / _reserve0.clone();
            let liquidity1 = amount1 * _total_supply.clone() / _reserve1.clone();
            liquidity0.min(liquidity1)
        };

        // do mint
        self.lp.mint(token_balances, arg.to, liquidity.clone());

        self.update(balance0, balance1, _reserve0, _reserve1);
        if fee_on {
            self.k_last = self.reserve0.clone() * self.reserve1.clone();
        }

        // ! push log

        Ok(liquidity)
    }

    pub fn add_liquidity(
        &mut self,
        fee_to: Option<Account>,
        token_balances: &mut TokenBalances,
        self_canister: SelfCanister,
        pa: PairAmm,
        arg: TokenPairLiquidityAddArg,
    ) -> Result<TokenPairLiquidityAddSuccess, BusinessError> {
        let (amount_a, amount_b) = self.inner_add_liquidity(&arg)?;
        let pool_account = Account {
            owner: self_canister.id(),
            subaccount: Some(self.subaccount),
        };
        token_balances.token_transfer(arg.token_a, arg.from, pool_account, amount_a.clone());
        token_balances.token_transfer(arg.token_b, arg.from, pool_account, amount_b.clone());
        let liquidity = self.mint(fee_to, token_balances, pa, &pool_account, arg)?;
        Ok(TokenPairLiquidityAddSuccess {
            amount: (amount_a, amount_b),
            liquidity,
        })
    }
}

// ========================== view ==========================

#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct SwapV2MarketMakerView {
    subaccount: String,
    fee_rate: SwapRatioView,

    reserve0: Nat,
    reserve1: Nat,
    block_timestamp_last: u64,

    price_unit: Nat,
    price0_cumulative_last: Nat,
    price1_cumulative_last: Nat,
    k_last: Nat,

    lp: PoolLp,
    protocol_fee: Option<SwapRatioView>,
}

impl From<SwapV2MarketMaker> for SwapV2MarketMakerView {
    fn from(value: SwapV2MarketMaker) -> Self {
        Self {
            subaccount: hex::encode(value.subaccount),
            fee_rate: value.fee_rate.into(),
            reserve0: value.reserve0,
            reserve1: value.reserve1,
            block_timestamp_last: value.block_timestamp_last,
            price_unit: value.price_unit,
            price0_cumulative_last: value.price0_cumulative_last,
            price1_cumulative_last: value.price1_cumulative_last,
            k_last: value.k_last,
            lp: value.lp,
            protocol_fee: value.protocol_fee.map(|f| f.into()),
        }
    }
}
