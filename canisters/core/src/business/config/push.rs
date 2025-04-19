#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

/// 给所有更新的
pub fn inner_push_blocks(token: bool, swap: bool) {
    if token {
        ic_cdk::spawn(async {
            let _ =
                crate::business::config::blockchain::token::inner_config_token_blocks_push().await;
        });
    }
    if swap {
        ic_cdk::spawn(async {
            let _ =
                crate::business::config::blockchain::swap::inner_config_swap_blocks_push().await;
        });
    }
}
