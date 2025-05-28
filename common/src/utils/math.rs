use candid::Nat;
use once_cell::sync::Lazy;

/// zero
pub fn zero() -> Nat {
    Nat::from(0_u64)
}

/// ZERO
#[allow(unused)]
pub static ZERO: Lazy<Nat> = Lazy::new(zero);

#[test]
fn test_nat() {
    let values: Vec<u32> = vec![3150766848, 7362];
    let v = num_bigint::BigUint::from_slice(&values);
    let v = Nat::from(v);
    assert_eq!(v.to_string(), "31_622_700_000_000")
}
