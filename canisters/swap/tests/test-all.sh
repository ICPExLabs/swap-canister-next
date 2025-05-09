#!/usr/bin/env bash

sh canisters/swap/tests/test-archive-token.sh update

sh canisters/swap/tests/test-archive-swap.sh update

sh canisters/swap/tests/test-swap.sh update
