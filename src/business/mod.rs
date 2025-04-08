#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

pub mod config;

pub mod token;

pub mod pair;

pub mod test;

#[allow(unused)]
#[inline(always)]
async fn with_token_balance_lock<T, R1, F, R2, A, E>(
    token_accounts: &[TokenAccount],
    retries: u8,
    execute: F, // do execute
    call: A,    // async call
    error: E,   // error handle
) -> T
where
    R1: Future<Output = T> + Send,
    F: FnOnce() -> R1 + Send,
    R2: Future<Output = T> + Send,
    A: FnOnce(u8) -> R2 + Send,
    E: FnOnce(Vec<TokenAccount>) -> T,
{
    #[allow(clippy::panic)] // ? SAFETY
    if 5 < retries {
        panic!("Too many retries");
    }

    match with_mut_state_without_record(|s| s.business_token_balance_lock(token_accounts)) {
        Ok(guard) => {
            let r = execute().await;
            drop(guard);
            r
        }
        Err(accounts) => {
            if 0 < retries {
                call(retries - 1).await
            } else {
                error(accounts)
            }
        }
    }
}

// 查询
#[ic_cdk::query(guard = "has_business_example_query")]
fn business_example_query() -> String {
    with_state(|s| s.business_example_query())
}

// 修改
#[ic_cdk::update(guard = "has_business_example_set")]
fn business_example_set(test: String) {
    let _guard = call_once_guard(); // post 接口应该拦截

    let caller = caller();
    let arg_content = format!("set test: {}", test); // * 记录参数内容

    with_mut_state(
        |s, _done| {
            s.business_example_update(test);
        },
        caller,
        RecordTopics::Example.topic(),
        arg_content,
    )
}

// 查询
#[ic_cdk::query(guard = "has_business_example_query")]
fn business_example_cell_query() -> String {
    with_state(|s| s.business_example_cell_query().cell_data)
}

// 修改
#[ic_cdk::update(guard = "has_business_example_set")]
fn business_example_cell_set(test: String) {
    let _guard = call_once_guard(); // post 接口应该拦截

    let caller = caller();
    let arg_content = format!("set test: {}", test); // * 记录参数内容

    with_mut_state(
        |s, _done| {
            s.business_example_cell_update(test);
        },
        caller,
        RecordTopics::ExampleCell.topic(),
        arg_content,
    )
}

// 查询
#[ic_cdk::query(guard = "has_business_example_query")]
fn business_example_vec_query() -> Vec<ExampleVec> {
    with_state(|s| s.business_example_vec_query())
}

// 修改
#[ic_cdk::update(guard = "has_business_example_set")]
fn business_example_vec_push(test: u64) {
    let _guard = call_once_guard(); // post 接口应该拦截

    let caller = caller();
    let arg_content = format!("set test: {}", test); // * 记录参数内容

    with_mut_state(
        |s, _done| {
            s.business_example_vec_push(test);
        },
        caller,
        RecordTopics::ExampleVec.topic(),
        arg_content,
    )
}

// 修改
#[ic_cdk::update(guard = "has_business_example_set")]
fn business_example_vec_pop() -> Option<ExampleVec> {
    let _guard = call_once_guard(); // post 接口应该拦截

    let caller = caller();
    let arg_content = format!("set test: {}", ""); // * 记录参数内容

    with_mut_state(
        |s, _done| s.business_example_vec_pop(),
        caller,
        RecordTopics::ExampleVec.topic(),
        arg_content,
    )
}

// 查询
#[ic_cdk::query(guard = "has_business_example_query")]
fn business_example_map_query() -> HashMap<u64, String> {
    with_state(|s| s.business_example_map_query())
}

// 修改
#[ic_cdk::update(guard = "has_business_example_set")]
fn business_example_map_update(key: u64, value: Option<String>) -> Option<String> {
    let _guard = call_once_guard(); // post 接口应该拦截

    let caller = caller();
    let arg_content = format!("set test: {} {:?}", key, value); // * 记录参数内容

    with_mut_state(
        |s, _done| s.business_example_map_update(key, value),
        caller,
        RecordTopics::ExampleMap.topic(),
        arg_content,
    )
}

// 查询
#[ic_cdk::query(guard = "has_business_example_query")]
fn business_example_log_query() -> Vec<String> {
    with_state(|s| s.business_example_log_query())
}

// 修改
#[ic_cdk::update(guard = "has_business_example_set")]
fn business_example_log_update(item: String) -> u64 {
    let _guard = call_once_guard(); // post 接口应该拦截

    let caller = caller();
    let arg_content = format!("set test: {}", item); // * 记录参数内容

    with_mut_state(
        |s, _done| s.business_example_log_update(item),
        caller,
        RecordTopics::ExampleLog.topic(),
        arg_content,
    )
}

// 查询
#[ic_cdk::query(guard = "has_business_example_query")]
fn business_example_priority_queue_query() -> Vec<u64> {
    with_state(|s| {
        s.business_example_priority_queue_query()
            .iter()
            .map(|v| v.vec_data)
            .collect()
    })
}

// 修改
#[ic_cdk::update(guard = "has_business_example_set")]
fn business_example_priority_queue_push(item: u64) {
    let _guard = call_once_guard(); // post 接口应该拦截

    let caller = caller();
    let arg_content = format!("set test: {}", item); // * 记录参数内容

    with_mut_state(
        |s, _done| {
            s.business_example_priority_queue_push(item);
        },
        caller,
        RecordTopics::ExamplePriorityQueue.topic(),
        arg_content,
    )
}

// 修改
#[ic_cdk::update(guard = "has_business_example_set")]
fn business_example_priority_queue_pop() -> Option<u64> {
    let _guard = call_once_guard(); // post 接口应该拦截

    let caller = caller();
    let arg_content = format!("set test: {}", ""); // * 记录参数内容

    with_mut_state(
        |s, _done| s.business_example_priority_queue_pop().map(|v| v.vec_data),
        caller,
        RecordTopics::ExamplePriorityQueue.topic(),
        arg_content,
    )
}
