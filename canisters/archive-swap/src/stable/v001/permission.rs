// Business permissions
pub fn has_business_queryable() -> Result<(), String> {
    use super::super::Business;
    let caller = ic_canister_kit::identity::caller();
    crate::types::with_state(|s| s.business_queryable(&caller))
}

pub fn has_business_blocks_append() -> Result<(), String> {
    use super::super::Business;
    let caller = ic_canister_kit::identity::caller();
    crate::types::with_state(|s| s.business_blocks_append_authorized(&caller))
}
