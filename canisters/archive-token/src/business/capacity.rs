#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

#[ic_cdk::query]
fn remaining_capacity() -> u64 {
    with_state(|s| s.business_remaining_capacity())
}
