use std::collections::HashMap;

use super::*;

use crate::utils::math::zero;

use super::{AmmText, BusinessError, DummyCanisterId, TokenInfo, TokenPair};

#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub enum PoolLp {
    InnerLP(InnerLP),
    OuterLP(OuterLP),
}

impl PoolLp {
    pub fn dummy_tokens(
        &self,
        tokens: &HashMap<CanisterId, TokenInfo>,
        pair: &TokenPair,
        amm: &AmmText,
    ) -> Vec<TokenInfo> {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.dummy_tokens(tokens, pair, amm),
            PoolLp::OuterLP(_outer_lp) => vec![],
        }
    }

    pub fn dummy_canisters(&self) -> Vec<CanisterId> {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.dummy_canisters(),
            PoolLp::OuterLP(_outer_lp) => vec![],
        }
    }

    pub fn get_total_supply(&self) -> Nat {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.total_supply.clone(),
            PoolLp::OuterLP(outer_lp) => outer_lp.total_supply.clone(),
        }
    }

    pub fn mint(
        &mut self,
        guard: &mut TokenBalancesGuard,
        to: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.mint(guard, to, amount),
            PoolLp::OuterLP(_outer_lp) => unimplemented!(),
        }
    }

    pub fn burn(
        &mut self,
        guard: &mut TokenBalancesGuard,
        from: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.burn(guard, from, amount),
            PoolLp::OuterLP(_outer_lp) => unimplemented!(),
        }
    }

    pub fn check_liquidity_removable<F>(
        &self,
        balance_of: F,
        liquidity: &Nat,
    ) -> Result<(), BusinessError>
    where
        F: Fn(CanisterId) -> Result<Nat, BusinessError>,
    {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.check_liquidity_removable(balance_of, liquidity),
            PoolLp::OuterLP(_outer_lp) => unimplemented!(),
        }
    }
}

// 内部存储 lp
#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct InnerLP {
    pub dummy_canister_id: DummyCanisterId, // 生成一个假的罐子序号用来标记池子 lp

    pub total_supply: Nat, // 需要记录总 lp，新增和移除流动性时候需要按比例退回对应的代币
    pub decimals: u8,      // 需要记录小数位数，显示需要
    pub fee: Nat,          // 需要记录手续费，转移时候需要用到
    pub minimum_liquidity: Nat, // 需要记录最小流动性，移除流动性时候需要检查是否达到最小流动性
}

impl InnerLP {
    pub fn dummy_tokens(
        &self,
        tokens: &HashMap<CanisterId, TokenInfo>,
        pair: &TokenPair,
        amm: &AmmText,
    ) -> Vec<TokenInfo> {
        use ic_canister_kit::common::trap;
        let token0 = trap(tokens.get(&pair.token0).ok_or("can not be"));
        let token1 = trap(tokens.get(&pair.token1).ok_or("can not be"));
        vec![TokenInfo {
            canister_id: self.dummy_canister_id.id(),
            name: format!("{}_{}_LP({})", token0.symbol, token1.symbol, amm.as_ref()),
            symbol: format!("{}_{}_LP({})", token0.symbol, token1.symbol, amm.as_ref()),
            decimals: self.decimals,
            fee: self.fee.clone(),
        }]
    }

    pub fn dummy_canisters(&self) -> Vec<CanisterId> {
        vec![self.dummy_canister_id.id()]
    }

    pub fn mint(
        &mut self,
        guard: &mut TokenBalancesGuard,
        to: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        guard.token_deposit(self.dummy_canister_id.id(), to, amount.clone())?;
        self.total_supply += amount; // Nat 不会超出精度
        Ok(())
    }

    pub fn burn(
        &mut self,
        guard: &mut TokenBalancesGuard,
        from: Account,
        amount: Nat,
    ) -> Result<(), BusinessError> {
        guard.token_withdraw(self.dummy_canister_id.id(), from, amount.clone())?;
        if self.total_supply < amount {
            return Err(BusinessError::Liquidity("INSUFFICIENT_LIQUIDITY".into()));
        }
        self.total_supply -= amount; // 如果变成负值会 panic
        Ok(())
    }

    pub fn check_liquidity_removable<F>(
        &self,
        balance_of: F,
        liquidity: &Nat,
    ) -> Result<(), BusinessError>
    where
        F: Fn(CanisterId) -> Result<Nat, BusinessError>,
    {
        // check balance
        let balance = balance_of(self.dummy_canister_id.id())?;
        if balance < *liquidity {
            return Err(BusinessError::Liquidity("INSUFFICIENT_LIQUIDITY".into()));
        }

        // check minimum liquidity
        let remain = self.total_supply.clone() - liquidity.to_owned();
        if remain < self.minimum_liquidity {
            return Err(BusinessError::Liquidity(
                "REMAIN_TOTAL_LIQUIDITY_TOO_SMALL".into(),
            ));
        }

        Ok(())
    }
}

// 外部存储 lp，是一个单独的罐子，有权限对其 mint 和 burn LP 代币，// ! 罐子手续费不应该销毁
#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct OuterLP {
    pub token_canister_id: CanisterId, // 需要记录外部的罐子 id

    pub total_supply: Nat, // 需要记录总 lp，新增和移除流动性时候需要按比例退回对应的代币
    pub decimals: u8,      // 需要记录小数位数，显示需要
    pub fee: Nat,          // 需要记录手续费，转移时候需要用到
    pub minimum_liquidity: Nat, // 需要记录最小流动性，移除流动性时候需要检查是否达到最小流动性
}

impl PoolLp {
    pub fn new_inner_lp(
        dummy_canister_id: DummyCanisterId,
        token0: &TokenInfo,
        token1: &TokenInfo,
    ) -> Self {
        let decimals = get_decimals(token0.decimals, token1.decimals);
        let fee = get_fee(&token0.fee, &token1.fee);

        // fee * 1000
        let minimum_liquidity = fee.clone() * Nat::from(1000_u64);

        Self::InnerLP(InnerLP {
            dummy_canister_id,
            total_supply: zero(),
            decimals,
            fee,
            minimum_liquidity,
        })
    }
}

fn get_decimals(decimals0: u8, decimals1: u8) -> u8 {
    let decimals = decimals0 + decimals1;
    decimals / 2 + if decimals % 2 == 0 { 0 } else { 1 }
}

fn get_fee(fee1: &Nat, fee2: &Nat) -> Nat {
    let fee1 = fee1.0.to_str_radix(10).len() - 1;
    let fee2 = fee2.0.to_str_radix(10).len() - 1;
    let size = fee1 + fee2;
    let size = size / 2 + if size % 2 == 0 { 0 } else { 1 };
    Nat::from(10_u64.pow(size as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let decimals = get_decimals(6, 18);
        assert_eq!(decimals, 12);

        let fee = get_fee(&Nat::from(10000_u64), &Nat::from(1000000_u64));
        assert_eq!(fee, Nat::from(100000_u64));

        let fee = get_fee(&Nat::from(10000_u64), &Nat::from(2000000_u64));
        assert_eq!(fee, Nat::from(100000_u64));

        let fee = get_fee(&Nat::from(10_000_u64), &Nat::from(2_000_000_000_000_u64));
        assert_eq!(fee, Nat::from(100_000_000_u64));
    }
}
