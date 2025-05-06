#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

/// for some update calls
pub fn inner_push_blocks(token: bool, swap: bool) {
    if token {
        ic_cdk::futures::spawn(async {
            match crate::business::config::blockchain::token::inner_config_token_blocks_push().await {
                Ok(data) => {
                    ic_cdk::println!("push token blocks success: {data:?}")
                }
                Err(err) => {
                    ic_cdk::println!("push token blocks failed: {err:?}")
                }
            }
        });
    }
    if swap {
        ic_cdk::futures::spawn(async {
            match crate::business::config::blockchain::swap::inner_config_swap_blocks_push().await {
                Ok(data) => {
                    ic_cdk::println!("push swap blocks success: {data:?}")
                }
                Err(err) => {
                    ic_cdk::println!("push swap blocks failed: {err:?}")
                }
            }
        });
    }
}
