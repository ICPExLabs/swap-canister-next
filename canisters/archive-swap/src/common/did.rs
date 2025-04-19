#[candid::candid_method(query)]
#[cfg(test)]
fn __get_candid_interface_tmp_hack() -> String {
    todo!()
}

#[ic_cdk::query]
#[cfg(not(test))]
fn __get_candid_interface_tmp_hack() -> String {
    #[allow(unused_imports)]
    use crate::types::*;

    candid::export_service!();
    __export_service()
}

/// `cargo test update_candid -- --nocapture`
#[test]
fn update_candid() {
    #[allow(unused_imports)]
    use crate::types::*;

    candid::export_service!();

    let text = __export_service();

    // std::println!("{}", text);

    use std::io::Write;
    let filename = "sources/source.did";
    let _ = std::fs::remove_file(filename);
    std::fs::File::create(&filename)
        .expect("create failed")
        .write_all(text.as_bytes())
        .expect("write candid failed");
}
