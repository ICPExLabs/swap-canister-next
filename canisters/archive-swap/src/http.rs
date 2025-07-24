use std::{borrow::Cow, collections::HashMap};

use percent_encoding::percent_decode_str;

use crate::stable::State;
use crate::types::*;

// https://github.com/dfinity/examples/blob/8b01d548d8548a9d4558a7a1dbb49234d02d7d03/motoko/http_counter/src/main.mo

// #[ic_cdk::update]
// fn http_request_update(request: CustomHttpRequest) -> CustomHttpResponse {
//     todo!()
// }

// http request
#[ic_cdk::query]
fn http_request(request: CustomHttpRequest) -> CustomHttpResponse {
    crate::stable::with_state(|state| inner_http_request(state, request))
}

#[inline]
fn inner_http_request(state: &State, req: CustomHttpRequest) -> CustomHttpResponse {
    let mut split_url = req.url.split('?');

    let path = split_url.next().unwrap_or("/");
    let path = percent_decode_str(path).decode_utf8().unwrap_or(Cow::Borrowed(path));

    // ic_cdk::println!("============== path: {} -> {}", req.url, path);
    // for (key, value) in request_headers.iter() {
    //     ic_cdk::println!("header: {}: {}", key, value);
    // }

    let mut code = 200; // default response code is 200
    let mut headers: HashMap<&str, Cow<str>> = HashMap::new();
    let body: Vec<u8>;
    let streaming_strategy: Option<StreamingStrategy> = None;

    if path == "/metrics" {
        let mut writer = ic_metrics_encoder::MetricsEncoder::new(vec![], ic_cdk::api::time() as i64 / 1_000_000);
        match state.business_metrics(&mut writer) {
            Ok(_) => {
                headers.insert("Content-Type", Cow::Borrowed("text/plain"));
                body = writer.into_inner();
            }
            Err(err) => {
                code = 500;
                body = format!("Failed to encode metrics: {err}").into_bytes();
            }
        }
    } else {
        code = 404;
        body = "Not Found".into()
    }

    CustomHttpResponse {
        status_code: code,
        headers: headers
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        body,
        streaming_strategy,
        upgrade: None,
    }
}
