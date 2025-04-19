#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

fn main() {
    println!("cargo:rerun-if-changed=proto/");
    println!("cargo:rerun-if-changed=build.rs");

    let mut config = prost_build::Config::new();
    config.bytes(["."]);
    // config.enum_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.enum_attribute(".", "#[non_exhaustive]");
    // config.type_attribute(".", "#[derive(PartialOrd)]"); // No sorting required

    config
        .out_dir("src/proto")
        .compile_protos(
            &[
                "../common/proto/common.proto",
                "../common/proto/block.proto",
                "../common/proto/token.proto",
                "../common/proto/swap.proto",
            ],
            &[".."],
        )
        .unwrap();
}
