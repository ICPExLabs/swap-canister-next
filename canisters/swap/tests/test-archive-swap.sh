#!/usr/bin/env bash
start_time=$(date +%H:%M:%S)
start_time_s=$(date +%s)

trap 'say test over' EXIT

if [ "$1" = "update" ]; then
    cargo test
    cargo clippy

    cargo test -p archive-swap update_candid -- --nocapture
    cargo build -p archive-swap --target wasm32-unknown-unknown --release
    ic-wasm target/wasm32-unknown-unknown/release/archive_swap.wasm -o canisters/archive-swap/sources/source_opt.wasm metadata candid:service -f canisters/archive-swap/sources/source.did -v public
    ic-wasm canisters/archive-swap/sources/source_opt.wasm -o canisters/archive-swap/sources/source_opt.wasm shrink
fi

set -e
cargo test test_archive_swap_common_apis -- --ignored
cargo test test_archive_swap_business_apis -- --ignored

end_time=$(date +%H:%M:%S)
end_time_s=$(date +%s)
spend=$(($end_time_s - $start_time_s))
spend_minutes=$(($spend / 60))
echo "✅ $start_time -> $end_time" "Total: $spend seconds ($spend_minutes mins) 🎉🎉🎉\n"

say test successful
