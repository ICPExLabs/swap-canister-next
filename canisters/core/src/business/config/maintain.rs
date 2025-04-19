#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== query ==============================

#[ic_cdk::query(guard = "has_pause_replace")]
fn config_maintain_archives_query() -> MaintainArchives {
    with_state(|s| s.business_config_maintain_archives_query().clone())
}

// ============================== update ==============================

#[ic_cdk::update(guard = "has_pause_replace")]
fn config_maintain_archives_set(config: MaintainArchivesConfig) {
    with_mut_state_without_record(|s| s.business_config_maintain_archives_set(config))
}
