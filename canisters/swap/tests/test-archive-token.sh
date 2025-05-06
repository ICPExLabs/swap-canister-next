#!/usr/bin/env bash
start_time=$(date +%H:%M:%S)
start_time_s=$(date +%s)

trap 'say test over' EXIT

if [ "$1" = "update" ]; then
    cargo test
    cargo clippy

    cargo test -p archive-token update_candid -- --nocapture
    cargo build -p archive-token --target wasm32-unknown-unknown --release
    ic-wasm target/wasm32-unknown-unknown/release/archive_token.wasm -o canisters/archive-token/sources/source_opt.wasm metadata candid:service -f canisters/archive-token/sources/source.did -v public
    ic-wasm canisters/archive-token/sources/source_opt.wasm -o canisters/archive-token/sources/source_opt.wasm shrink
fi

set -e
cargo test test_archive_token_common_apis -- --ignored
cargo test test_archive_token_business_apis -- --ignored

end_time=$(date +%H:%M:%S)
end_time_s=$(date +%s)
spend=$(($end_time_s - $start_time_s))
spend_minutes=$(($spend / 60))
echo "✅ $start_time -> $end_time" "Total: $spend seconds ($spend_minutes mins) 🎉🎉🎉\n"

say test successful
