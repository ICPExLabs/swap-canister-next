#!/usr/bin/env bash

if [ ! -f "canisters/archive-token/sources/source_opt.wasm.gz" ]; then
    ic-wasm target/wasm32-unknown-unknown/release/archive_token.wasm -o canisters/archive-token/sources/source_opt.wasm metadata candid:service -f canisters/archive-token/sources/source.did -v public
    gzip -kfn canisters/archive-token/sources/source_opt.wasm
fi
if [ ! -f "canisters/archive-swap/sources/source_opt.wasm.gz" ]; then
    ic-wasm target/wasm32-unknown-unknown/release/archive_swap.wasm -o canisters/archive-swap/sources/source_opt.wasm metadata candid:service -f canisters/archive-swap/sources/source.did -v public
    gzip -kfn canisters/archive-swap/sources/source_opt.wasm
fi
if [ ! -f "canisters/swap/sources/source_opt.wasm.gz" ]; then
    ic-wasm target/wasm32-unknown-unknown/release/swap.wasm -o canisters/swap/sources/source_opt.wasm metadata candid:service -f canisters/swap/sources/source.did -v public
    gzip -kfn canisters/swap/sources/source_opt.wasm
fi

sh canisters/swap/tests/test-archive-token.sh update

sh canisters/swap/tests/test-archive-swap.sh update

sh canisters/swap/tests/test-swap.sh update
