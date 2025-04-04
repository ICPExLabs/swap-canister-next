use candid::Nat;
use once_cell::sync::Lazy;

pub fn zero() -> Nat {
    Nat::from(0u64)
}

#[allow(unused)]
pub static ZERO: Lazy<Nat> = Lazy::new(zero);
