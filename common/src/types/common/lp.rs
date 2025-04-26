use std::borrow::Cow;
#[cfg(feature = "cdk")]
use std::collections::HashMap;

use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

use super::*;

#[allow(unused)]
use crate::{
    types::{AmmText, CanisterId, TokenPairAmm},
    utils::math::zero,
};

/// pool lp
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub enum PoolLp {
    /// inner
    #[serde(rename = "inner")]
    InnerLP(InnerLP),
    /// outer
    #[serde(rename = "outer")]
    OuterLP(OuterLP),
}

impl PoolLp {
    #[cfg(feature = "cdk")]
    pub fn dummy_tokens(&self, tokens: &HashMap<CanisterId, Cow<'_, TokenInfo>>, pa: &TokenPairAmm) -> Vec<TokenInfo> {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.dummy_tokens(tokens, pa),
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

    pub fn get_decimals(&self) -> u8 {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.decimals,
            PoolLp::OuterLP(outer_lp) => outer_lp.decimals,
        }
    }

    pub fn mint_fee<F>(&mut self, token_liquidity_mint_fee: F, to: Account, amount: Nat) -> Result<(), BusinessError>
    where
        F: FnOnce(CanisterId, Account, Nat) -> Result<(), BusinessError>,
    {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.mint_fee(token_liquidity_mint_fee, to, amount),
            PoolLp::OuterLP(_outer_lp) => unimplemented!(),
        }
    }

    pub fn mint<F>(&mut self, token_liquidity_mint: F, to: Account, amount: Nat) -> Result<(), BusinessError>
    where
        F: FnOnce(CanisterId, Account, Nat) -> Result<(), BusinessError>,
    {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.mint(token_liquidity_mint, to, amount),
            PoolLp::OuterLP(_outer_lp) => unimplemented!(),
        }
    }

    pub fn burn<F>(
        &mut self,
        token_liquidity_burn: F,
        from: Account,
        amount: Nat,
        fee: Option<BurnFee>,
    ) -> Result<(), BusinessError>
    where
        F: FnOnce(CanisterId, Account, Nat, Option<BurnFee>) -> Result<(), BusinessError>,
    {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.burn(token_liquidity_burn, from, amount, fee),
            PoolLp::OuterLP(_outer_lp) => unimplemented!(),
        }
    }

    pub fn check_liquidity_removable<F>(
        &self,
        balance_of: F,
        liquidity_without_fee: &Nat,
        fee_to: Option<Account>,
    ) -> Result<(), BusinessError>
    where
        F: Fn(CanisterId) -> Result<Nat, BusinessError>,
    {
        match self {
            PoolLp::InnerLP(inner_lp) => inner_lp.check_liquidity_removable(balance_of, liquidity_without_fee, fee_to),
            PoolLp::OuterLP(_outer_lp) => unimplemented!(),
        }
    }
}

/// Internal storage lp
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct InnerLP {
    /// dummy canister id
    pub dummy_canister_id: DummyCanisterId, // Generate a fake canister number to mark the pool lp

    /// total supply
    pub total_supply: Nat, // Total lp needs to be recorded, and the corresponding tokens need to be returned proportionally when adding and removing liquidity.
    /// decimals
    pub decimals: u8, // Decimal places need to be recorded, display
    /// fee
    pub fee: Nat, // The handling fee needs to be recorded, and it is required to be used during transfer
    /// minimum liquidity
    pub minimum_liquidity: Nat, // Minimum liquidity needs to be recorded, and when removing liquidity, you need to check whether it is achieved.
}

impl InnerLP {
    #[cfg(feature = "cdk")]
    pub fn dummy_tokens(&self, tokens: &HashMap<CanisterId, Cow<'_, TokenInfo>>, pa: &TokenPairAmm) -> Vec<TokenInfo> {
        use ic_canister_kit::common::trap;
        let TokenPairAmm { pair, amm } = pa;
        let amm: AmmText = (*amm).into();
        let token0 = trap(tokens.get(&pair.token0).ok_or("can not be"));
        let token1 = trap(tokens.get(&pair.token1).ok_or("can not be"));
        vec![TokenInfo {
            canister_id: self.dummy_canister_id.id(),
            name: format!("{}_{}_LP({})", token0.symbol, token1.symbol, amm.as_ref()),
            symbol: format!("{}_{}_LP({})", token0.symbol, token1.symbol, amm.as_ref()),
            decimals: self.decimals,
            fee: self.fee.clone(),
            is_lp_token: true,
        }]
    }

    pub fn dummy_canisters(&self) -> Vec<CanisterId> {
        vec![self.dummy_canister_id.id()]
    }

    pub fn mint_fee<F>(&mut self, token_liquidity_mint_fee: F, to: Account, amount: Nat) -> Result<(), BusinessError>
    where
        F: FnOnce(CanisterId, Account, Nat) -> Result<(), BusinessError>,
    {
        token_liquidity_mint_fee(self.dummy_canister_id.id(), to, amount.clone())?;
        self.total_supply += amount; // Nat will not exceed the accuracy
        Ok(())
    }

    pub fn mint<F>(&mut self, token_liquidity_mint: F, to: Account, amount: Nat) -> Result<(), BusinessError>
    where
        F: FnOnce(CanisterId, Account, Nat) -> Result<(), BusinessError>,
    {
        token_liquidity_mint(self.dummy_canister_id.id(), to, amount.clone())?;
        self.total_supply += amount; // Nat will not exceed the accuracy
        Ok(())
    }

    pub fn burn<F>(
        &mut self,
        token_liquidity_burn: F,
        from: Account,
        amount: Nat,
        fee: Option<BurnFee>,
    ) -> Result<(), BusinessError>
    where
        F: FnOnce(CanisterId, Account, Nat, Option<BurnFee>) -> Result<(), BusinessError>,
    {
        if let Some(fee) = &fee {
            if fee.fee != self.fee {
                return Err(BusinessError::Liquidity(format!(
                    "burn fee is not matched: {} != {}",
                    fee.fee, self.fee
                )));
            }
        }
        token_liquidity_burn(self.dummy_canister_id.id(), from, amount.clone(), fee.clone())?;
        let fee = fee.map(|_| self.fee.clone()).unwrap_or_default();
        let total = amount + fee;
        if self.total_supply < total {
            return Err(BusinessError::Liquidity("INSUFFICIENT_LIQUIDITY".into()));
        }
        self.total_supply -= total; // If it becomes negative, it will panic
        Ok(())
    }

    pub fn check_liquidity_removable<F>(
        &self,
        balance_of: F,
        liquidity_without_fee: &Nat,
        fee_to: Option<Account>,
    ) -> Result<(), BusinessError>
    where
        F: Fn(CanisterId) -> Result<Nat, BusinessError>,
    {
        // check balance
        let required = liquidity_without_fee.clone() + fee_to.map(|_| self.fee.clone()).unwrap_or_default();
        let balance = balance_of(self.dummy_canister_id.id())?;
        if balance < required {
            return Err(BusinessError::Liquidity("INSUFFICIENT_LIQUIDITY".into()));
        }

        // check minimum liquidity
        let remain = self.total_supply.clone() - required;
        if remain < self.minimum_liquidity {
            return Err(BusinessError::Liquidity("REMAIN_TOTAL_LIQUIDITY_TOO_SMALL".into()));
        }

        Ok(())
    }
}

/// External storage lp, is a separate canister, with permission to its mint and burn LP tokens，// ! The canister handling fee should not be destroyed
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct OuterLP {
    /// outer canister id
    pub token_canister_id: CanisterId, // Need to record the external canister id

    /// total supply
    pub total_supply: Nat, // Total lp needs to be recorded, and the corresponding tokens need to be returned proportionally when adding and removing liquidity.
    /// decimals
    pub decimals: u8, // Decimal places need to be recorded, display
    /// fee
    pub fee: Nat, // The handling fee needs to be recorded, and it is required to be used during transfer
    /// minimum liquidity
    pub minimum_liquidity: Nat, // Minimum liquidity needs to be recorded, and when removing liquidity, you need to check whether it is achieved.
}

impl PoolLp {
    /// new
    pub fn new_inner_lp(dummy_canister_id: DummyCanisterId, token0: &TokenInfo, token1: &TokenInfo) -> Self {
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
