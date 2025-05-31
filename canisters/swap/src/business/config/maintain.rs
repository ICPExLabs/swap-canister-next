#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// ============================== query ==============================

#[ic_cdk::query]
fn config_maintain_archives_query() -> MaintainArchives {
    with_state(|s| s.business_config_maintain_archives_query().clone())
}

// ============================== update ==============================

#[ic_cdk::update(guard = "has_business_config_maintaining")]
fn config_maintain_archives_set(config: MaintainArchivesConfig) {
    with_mut_state(|s| s.business_config_maintain_archives_set(config))
}

// ============================== fix ==============================

#[ic_cdk::update(guard = "has_business_config_maintaining")]
fn config_maintain_pools() -> String {
    let (self_canister, _) = match check_caller(&ic_canister_kit::identity::caller()) {
        Ok(v) => v,
        Err(e) => return e.to_string(),
    };
    match with_mut_state(|s| s.business_fix_bg_pool(self_canister)) {
        Ok(_) => "success".to_string(),
        Err(e) => e.to_string(),
    }
}
