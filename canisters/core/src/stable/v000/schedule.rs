use ic_canister_kit::{common::option::display_option_by, times::now};

#[allow(unused)]
use super::super::*;
#[allow(unused)]
use super::types::*;

#[allow(unused)]
#[allow(unused_variables)]
pub async fn schedule_task(record_by: Option<CallerId>) {
    // 如果有定时任务
    ic_cdk::println!(
        "{}: do schedule task... ({})",
        display_option_by(&record_by, |p| p.to_text()),
        now()
    );

    // ! 为了保证记录的完整性，不应当发生 panic
    inner_task(record_by).await;
}

async fn inner_task(caller: Option<CallerId>) {
    ic_cdk::println!("do something: {:?}", caller.map(|c| c.to_text()));
}
