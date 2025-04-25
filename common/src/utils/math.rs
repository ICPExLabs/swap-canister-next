use candid::Nat;
use once_cell::sync::Lazy;

/// zero
pub fn zero() -> Nat {
    Nat::from(0_u64)
}

/// ZERO
#[allow(unused)]
pub static ZERO: Lazy<Nat> = Lazy::new(zero);
