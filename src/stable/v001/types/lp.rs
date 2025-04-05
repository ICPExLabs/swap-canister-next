use candid::{CandidType, Nat};
use ic_canister_kit::types::CanisterId;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use crate::utils::math::zero;

use super::{BusinessError, DummyCanisterId, TokenBalances, TokenInfo};

#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub enum PoolLp {
    InnerLP(InnerLP),
    OuterLP(OuterLP),
}

impl PoolLp {
    pub fn get_total_supply(&self) -> Nat {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.total_supply.clone(),
            PoolLp::OuterLP(outer_lp) => outer_lp.total_supply.clone(),
        }
    }

    pub fn mint(&mut self, token_balances: &mut TokenBalances, to: Account, amount: Nat) {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.mint(token_balances, to, amount),
            PoolLp::OuterLP(_outer_lp) => unimplemented!(),
        }
    }

    pub fn burn(&mut self, token_balances: &mut TokenBalances, from: Account, amount: Nat) {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.burn(token_balances, from, amount),
            PoolLp::OuterLP(_outer_lp) => unimplemented!(),
        }
    }

    pub fn check_liquidity_removable(
        &self,
        token_balances: &TokenBalances,
        from: &Account,
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        match self {
            PoolLp::InnerLP(inner_lp) => {
                inner_lp.check_liquidity_removable(token_balances, from, liquidity)
            }
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
    pub fn mint(&mut self, token_balances: &mut TokenBalances, to: Account, amount: Nat) {
        token_balances.token_deposit(self.dummy_canister_id.id(), to, amount.clone());
        self.total_supply += amount;
    }

    pub fn burn(&mut self, token_balances: &mut TokenBalances, from: Account, amount: Nat) {
        token_balances.token_withdraw(self.dummy_canister_id.id(), from, amount.clone());
        self.total_supply -= amount;
    }

    pub fn check_liquidity_removable(
        &self,
        token_balances: &TokenBalances,
        from: &Account,
        liquidity: &Nat,
    ) -> Result<(), BusinessError> {
        // check balance
        let balance = token_balances.token_balance_of(self.dummy_canister_id.id(), *from);
        if balance < *liquidity {
            return Err(BusinessError::InsufficientBalance((
                self.dummy_canister_id.id(),
                balance,
            )));
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
        let fee = get_fee(&Nat::from(10000_u64), &Nat::from(1000000_u64));
        assert_eq!(fee, Nat::from(100000_u64));

        let fee = get_fee(&Nat::from(10000_u64), &Nat::from(2000000_u64));
        assert_eq!(fee, Nat::from(100000_u64));
    }
}
