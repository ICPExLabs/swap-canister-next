#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

#[ic_cdk::update(guard = "has_business_blocks_append")]
fn append_blocks(args: Vec<EncodedBlock>) {
    with_mut_state(|s| s.business_blocks_append(args))
}
