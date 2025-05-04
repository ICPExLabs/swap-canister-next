/// Constant product market maker（Constant Product AMM）
/// - Formula：x * y = k（x and y are the number of two assets, and k is the constant）
/// - Representative Project：Uniswap V2/V3、SushiSwap、PancakeSwap
/// - Features
///   - Simple and efficient, suitable for most trading scenarios.
///   - There is significant slippage in large-scale transactions (significant price changes).
///   - Uniswap V3 introduces "concentrated liquidity", allowing LPs to set price ranges and improve capital efficiency.
#[cfg(feature = "cdk")]
use std::borrow::Cow;
#[cfg(feature = "cdk")]
use std::collections::HashMap;

use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

#[allow(unused)]
use crate::{
    types::{BusinessError, CanisterId, PoolLp, SelfCanister, SwapRatio, SwapRatioView, TokenInfo, TokenPairAmm},
    utils::{
        math::{ZERO, zero},
        principal::sort_tokens,
    },
};

/// The required data under the current algorithm processing fee
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub struct SwapV2MarketMaker {
    pub subaccount: Subaccount, // ! fixed. Fund balance storage location self_canister_id.subaccount
    pub fee_rate: SwapRatio,    // ! fixed. Transaction rates

    pub token0: CanisterId, // ! Canister_id of the current token0
    pub token1: CanisterId, // ! Canister_id of the current token1
    pub reserve0: Nat,      // ! The current balance deposited by token0
    pub reserve1: Nat,      // ! The current balance deposited by token1
    pub block_timestamp_last: u64,

    pub price_cumulative_exponent: u8, // Exponential calculation
    pub price0_cumulative_last: Nat,
    pub price1_cumulative_last: Nat,
    pub k_last: Nat, // ! Current k value

    pub lp: PoolLp, // lp token information, Once the new pool is successfully created, other data cannot be changed except for supply
    pub protocol_fee: Option<SwapRatio>, // The processing fee and lp fee for the agreement sharing should be equal to 1
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

    #[cfg(feature = "cdk")]
    pub fn dummy_tokens(&self, tokens: &HashMap<CanisterId, Cow<'_, TokenInfo>>, pa: &TokenPairAmm) -> Vec<TokenInfo> {
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
    pub fn get_reserves(&self, token_a: CanisterId, token_b: CanisterId) -> (Nat, Nat) {
        let (token0, _) = sort_tokens(token_a, token_b);
        let (reserve0, reserve1) = (self.reserve0.clone(), self.reserve1.clone());
        if token_a == token0 {
            (reserve0, reserve1)
        } else {
            (reserve1, reserve0)
        }
    }

    /// Calculate the amount required for another token
    pub fn quote(amount_a: &Nat, reserve_a: &Nat, reserve_b: &Nat) -> Nat {
        assert!(*amount_a > *ZERO, "INSUFFICIENT_AMOUNT");
        assert!(*reserve_a > *ZERO, "INSUFFICIENT_LIQUIDITY");
        assert!(*reserve_b > *ZERO, "INSUFFICIENT_LIQUIDITY");
        amount_a.to_owned() * reserve_b.to_owned() / reserve_a.to_owned()
    }

    pub fn check_liquidity_removable<F>(
        &self,
        token_balance_of: F,
        from: &Account,
        liquidity_without_fee: &Nat,
        fee_to: Option<Account>,
    ) -> Result<(), BusinessError>
    where
        F: Fn(CanisterId, Account) -> Result<Nat, BusinessError>,
    {
        self.lp
            .check_liquidity_removable(|token| token_balance_of(token, *from), liquidity_without_fee, fee_to)
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
        if balance0_adjusted * balance1_adjusted < self.reserve0.clone() * self.reserve1.clone() * d * d {
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

    pub fn removable(&self) -> bool {
        self.lp.removable()
    }

    pub fn text(&self) -> String {
        format!(
            "token0:[{}]({})\ntoken1:[{}]({})\nAmm:SwapV2({})\nProtocolFee:{}\nLP Supply:{}",
            self.token0.to_text(),
            self.reserve0,
            self.token1.to_text(),
            self.reserve1,
            self.fee_rate,
            self.protocol_fee
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or("None".to_string()),
            self.lp.get_total_supply()
        )
    }

    pub fn delta(&self, next: &Self, tokens: &[TokenInfo]) -> String {
        let symbol0 = tokens
            .iter()
            .find(|v| v.canister_id == self.token0)
            .map(|t| t.symbol.clone())
            .unwrap_or_default();
        let symbol1 = tokens
            .iter()
            .find(|v| v.canister_id == self.token1)
            .map(|t| t.symbol.clone())
            .unwrap_or_default();
        let decimals0 = tokens
            .iter()
            .find(|v| v.canister_id == self.token0)
            .map(|t| t.decimals)
            .unwrap_or_default();
        let decimals1 = tokens
            .iter()
            .find(|v| v.canister_id == self.token1)
            .map(|t| t.decimals)
            .unwrap_or_default();
        let decimals_lp = self.lp.get_decimals();
        fn show_nat(v: &Nat, decimals: u8) -> String {
            let mut text = v.0.to_string();
            if decimals > 0 {
                if text.len() <= decimals as usize + 1 {
                    text = "0".repeat(decimals as usize + 1 - text.len()) + &text;
                }
                text.insert(text.len() - decimals as usize, '.');
                while text.ends_with('0') {
                    text.pop();
                }
                if text.ends_with('.') {
                    text.pop();
                }
            }
            text
        }
        fn get_delta(previous: &Nat, next: &Nat, decimals: u8) -> Option<String> {
            match previous.cmp(next) {
                std::cmp::Ordering::Less => {
                    Some(format!("+ {}", show_nat(&(next.clone() - previous.clone()), decimals)))
                }
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => {
                    Some(format!("- {}", show_nat(&(previous.clone() - next.clone()), decimals)))
                }
            }
        }

        let delta_reserve0 = get_delta(&self.reserve0, &next.reserve0, decimals0);
        let delta_reserve1 = get_delta(&self.reserve1, &next.reserve1, decimals1);
        let delta_total_supply = get_delta(&self.lp.get_total_supply(), &next.lp.get_total_supply(), decimals_lp);

        let token0 = format!(
            "token0 ({symbol0}): [{}]({})",
            self.token0.to_text(),
            match &delta_reserve0 {
                Some(delta) => format!(
                    "{} {delta} = {}",
                    show_nat(&self.reserve0, decimals0),
                    show_nat(&next.reserve0, decimals0)
                ),
                None => show_nat(&next.reserve0, decimals0),
            }
        );
        let token1 = format!(
            "token1 ({symbol1}): [{}]({})",
            self.token1.to_text(),
            match &delta_reserve1 {
                Some(delta) => format!(
                    "{} {delta} = {}",
                    show_nat(&self.reserve1, decimals1),
                    show_nat(&next.reserve1, decimals1)
                ),
                None => show_nat(&next.reserve1, decimals1),
            }
        );

        format!(
            "{}\n{}\nAmm: SwapV2({})\nProtocolFee: {}\nLP Supply: [{}]({})",
            if delta_reserve1.as_ref().is_some_and(|d| d.starts_with("+")) {
                &token1
            } else {
                &token0
            },
            if delta_reserve1.as_ref().is_some_and(|d| d.starts_with("+")) {
                &token0
            } else {
                &token1
            },
            self.fee_rate,
            self.protocol_fee.as_ref().map(|v| v.to_string()).unwrap_or_default(),
            match self.lp.dummy_canisters().first() {
                Some(canister_id) => canister_id.to_text(),
                None => "".to_string(),
            },
            match delta_total_supply {
                Some(delta) => format!(
                    "{} {delta} = {}",
                    show_nat(&self.lp.get_total_supply(), decimals_lp),
                    show_nat(&next.lp.get_total_supply(), decimals_lp)
                ),
                None => show_nat(&next.lp.get_total_supply(), decimals_lp),
            },
        )
    }

    pub fn swap_to(&self, from: &CanisterId, from_amount: f64) -> Option<(CanisterId, f64)> {
        if self.token0 == *from {
            Some((
                self.token1,
                from_amount * self.reserve1.0.to_f64()? / self.reserve0.0.to_f64()?,
            ))
        } else if self.token1 == *from {
            Some((
                self.token0,
                from_amount * self.reserve0.0.to_f64()? / self.reserve1.0.to_f64()?,
            ))
        } else {
            None
        }
    }

    pub fn get_reserve(&self, token: &CanisterId) -> Option<f64> {
        if self.token0 == *token {
            self.reserve0.0.to_f64()
        } else if self.token1 == *token {
            self.reserve1.0.to_f64()
        } else {
            None
        }
    }

    pub fn get_fee(&self, amount_in: f64) -> f64 {
        let n = self.fee_rate.numerator as f64;
        let d = self.fee_rate.denominator as f64;
        amount_in * n / d
    }
}

// ========================== view ==========================

/// swap v2
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
