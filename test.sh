#!/usr/bin/env bash

# 运行完毕自动停止
dfx stop
trap 'say test over && dfx stop' EXIT

dfx start --background --clean # 开启新的 dfx 环境
# dfx start --background --clean >/dev/null 2>&1 # 开启新的 dfx 环境

function red { echo "\033[31m$1\033[0m"; }
function green { echo "\033[32m$1\033[0m"; }
function yellow { echo "\033[33m$1\033[0m"; }
function blue { echo "\033[34m$1\033[0m"; }

function canister_id {
    # cat ".dfx/local/canister_ids.json"
    # echo $(cat ".dfx/local/canister_ids.json" | tr -d '\n' | awk -F "$1" '{print $2}' | awk -F "\": \"" '{print $2}' | awk -F "\"" '{print $1}')
    echo $(dfx canister id $1)
}

function test {
    tips="$1"
    result="$(echo $2 | tr -d '\n')"
    need1="$3"
    need2="$4"
    need3="$5"
    # echo $result
    # echo $need1
    # echo $need2
    # echo $need3
    if [[ $(echo $result | grep "$need1") != "" ]]; then
        green "* Passed: $tips -> $result -> $need1"
    else
        red "* Failed: $tips"
        green "Expected: $need1"
        yellow "Got: $result"
        exit 1
    fi
    if [[ $need2 != "" ]]; then
        if [[ $(echo $result | grep "$need2") != "" ]]; then
            green "* Passed: $tips -> $result -> $need2"
        else
            red "* Failed: $tips"
            green "Expected: $need2"
            yellow "Got: $result"
            exit 1
        fi
    fi
    if [[ $need3 != "" ]]; then
        if [[ $(echo $result | grep "$need3") != "" ]]; then
            green "* Passed: $tips -> $result -> $need3"
        else
            red "* Failed: $tips"
            green "Expected: $need3"
            yellow "Got: $result"
            exit 1
        fi
    fi
}

ANONYMOUS="2vxsx-fae"
DEFAULT=$(dfx identity get-principal)
ALICE=$(dfx --identity alice identity get-principal)
BOB=$(dfx --identity bob identity get-principal)

# cargo test
cargo clippy
# cargo audit --no-fetch --quiet

# ! 0. deploy tokens
token_ICP="ryjl3-tyaaa-aaaaa-aaaba-cai"
dfx canister create token_ICP --specified-id "$token_ICP"
dfx deploy token_ICP --argument "(variant {\
    Init = record {\
        token_name = \"ICP\";\
        token_symbol = \"ICP\";\
        decimals = opt (8 : nat8);\
        transfer_fee = 10000 : nat;\
        metadata = vec {};\
        minting_account = record {\
            owner = principal \"${BOB}\";\
            subaccount = null;\
        };
        initial_balances = vec {\
            record { record { owner=principal \"$DEFAULT\"; subaccount=null }; 1_000_000_000_000:nat }\
        };\
        fee_collector_account = opt record { owner=principal \"$ALICE\"; subaccount=null };\
        archive_options = record {\
            num_blocks_to_archive = 1000 : nat64;\
            max_transactions_per_response = null;\
            trigger_threshold = 1000 : nat64;\
            more_controller_ids = null;\
            max_message_size_bytes = null;\
            cycles_for_archive_creation = null;\
            node_max_memory_size_bytes = null;\
            controller_id = principal \"aaaaa-aa\";\
        };\
        max_memo_length = null;\
        feature_flags = opt record { icrc2 = true };\
    }\
})"

token_ckBTC="mxzaz-hqaaa-aaaar-qaada-cai"
dfx canister create token_ckBTC --specified-id "$token_ckBTC"
dfx deploy token_ckBTC --argument "(variant {\
    Init = record {\
        token_name = \"ckBTC\";\
        token_symbol = \"ckBTC\";\
        decimals = opt (8 : nat8);\
        transfer_fee = 10 : nat;\
        metadata = vec {};\
        minting_account = record {\
            owner = principal \"${BOB}\";\
            subaccount = null;\
        };
        initial_balances = vec {\
            record { record { owner=principal \"$DEFAULT\"; subaccount=null }; 100000000:nat }\
        };\
        fee_collector_account = opt record { owner=principal \"$ALICE\"; subaccount=null };\
        archive_options = record {\
            num_blocks_to_archive = 1000 : nat64;\
            max_transactions_per_response = null;\
            trigger_threshold = 1000 : nat64;\
            more_controller_ids = null;\
            max_message_size_bytes = null;\
            cycles_for_archive_creation = null;\
            node_max_memory_size_bytes = null;\
            controller_id = principal \"aaaaa-aa\";\
        };\
        max_memo_length = null;\
        feature_flags = opt record { icrc2 = true };\
    }\
})"

token_ckETH="ss2fx-dyaaa-aaaar-qacoq-cai"
dfx canister create token_ckETH --specified-id "$token_ckETH"
dfx deploy token_ckETH --argument "(variant {\
    Init = record {\
        token_name = \"ckETH\";\
        token_symbol = \"ckETH\";\
        decimals = opt (18 : nat8);\
        transfer_fee = 2_000_000_000_000 : nat;\
        metadata = vec {};\
        minting_account = record {\
            owner = principal \"${BOB}\";\
            subaccount = null;\
        };
        initial_balances = vec {\
            record { record { owner=principal \"$DEFAULT\"; subaccount=null }; 10_000_000_000_000_000_000:nat }\
        };\
        fee_collector_account = opt record { owner=principal \"$ALICE\"; subaccount=null };\
        archive_options = record {\
            num_blocks_to_archive = 1000 : nat64;\
            max_transactions_per_response = null;\
            trigger_threshold = 1000 : nat64;\
            more_controller_ids = null;\
            max_message_size_bytes = null;\
            cycles_for_archive_creation = null;\
            node_max_memory_size_bytes = null;\
            controller_id = principal \"aaaaa-aa\";\
        };\
        max_memo_length = null;\
        feature_flags = opt record { icrc2 = true };\
    }\
})"

token_ckUSDT="cngnf-vqaaa-aaaar-qag4q-cai"
dfx canister create token_ckUSDT --specified-id "$token_ckUSDT"
dfx deploy token_ckUSDT --argument "(variant {\
    Init = record {\
        token_name = \"ckUSDT\";\
        token_symbol = \"ckUSDT\";\
        decimals = opt (6 : nat8);\
        transfer_fee = 10000 : nat;\
        metadata = vec {};\
        minting_account = record {\
            owner = principal \"${BOB}\";\
            subaccount = null;\
        };
        initial_balances = vec {\
            record { record { owner=principal \"$DEFAULT\"; subaccount=null }; 100_000_000_000_000:nat }\
        };\
        fee_collector_account = opt record { owner=principal \"$ALICE\"; subaccount=null };\
        archive_options = record {\
            num_blocks_to_archive = 1000 : nat64;\
            max_transactions_per_response = null;\
            trigger_threshold = 1000 : nat64;\
            more_controller_ids = null;\
            max_message_size_bytes = null;\
            cycles_for_archive_creation = null;\
            node_max_memory_size_bytes = null;\
            controller_id = principal \"aaaaa-aa\";\
        };\
        max_memo_length = null;\
        feature_flags = opt record { icrc2 = true };\
    }\
})"

token_snsCHAT="2ouva-viaaa-aaaaq-aaamq-cai"
dfx canister create token_snsCHAT --specified-id "$token_snsCHAT"
dfx deploy token_snsCHAT --argument "(variant {\
    Init = record {\
        token_name = \"CHAT\";\
        token_symbol = \"CHAT\";\
        decimals = opt (8 : nat8);\
        transfer_fee = 100000 : nat;\
        metadata = vec {};\
        minting_account = record {\
            owner = principal \"${BOB}\";\
            subaccount = null;\
        };
        initial_balances = vec {\
            record { record { owner=principal \"$DEFAULT\"; subaccount=null }; 1000000000000:nat }\
        };\
        fee_collector_account = opt record { owner=principal \"$ALICE\"; subaccount=null };\
        archive_options = record {\
            num_blocks_to_archive = 1000 : nat64;\
            max_transactions_per_response = null;\
            trigger_threshold = 1000 : nat64;\
            more_controller_ids = null;\
            max_message_size_bytes = null;\
            cycles_for_archive_creation = null;\
            node_max_memory_size_bytes = null;\
            controller_id = principal \"aaaaa-aa\";\
        };\
        max_memo_length = null;\
        feature_flags = opt record { icrc2 = true };\
    }\
})"

blue "0 query balances"
test "icrc1_balance_of ICP/default" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(1_000_000_000_000 : nat)'
test "icrc1_balance_of ICP/alice" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ICP/bob" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckBTC/default" "$(dfx --identity default canister call token_ckBTC icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(100_000_000 : nat)'
test "icrc1_balance_of ckBTC/alice" "$(dfx --identity default canister call token_ckBTC icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckBTC/bob" "$(dfx --identity default canister call token_ckBTC icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckETH/default" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(10_000_000_000_000_000_000 : nat)'
test "icrc1_balance_of ckETH/alice" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckETH/bob" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckUSDT/default" "$(dfx --identity default canister call token_ckUSDT icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(100_000_000_000_000 : nat)'
test "icrc1_balance_of ckUSDT/alice" "$(dfx --identity default canister call token_ckUSDT icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckUSDT/bob" "$(dfx --identity default canister call token_ckUSDT icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of snsCHAT/default" "$(dfx --identity default canister call token_snsCHAT icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(1_000_000_000_000 : nat)'
test "icrc1_balance_of snsCHAT/alice" "$(dfx --identity default canister call token_snsCHAT icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of snsCHAT/bob" "$(dfx --identity default canister call token_snsCHAT icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'

# ! 1. 测试 swap
red "\n=========== 1. swap ===========\n"
dfx canister create swap --specified-id "bkyz2-fmaaa-aaaaa-qaaaq-cai" # --with-cycles 50T
dfx deploy --mode=reinstall --yes --argument "(null)" swap
swap=$(canister_id "swap")
blue "Swap Canister: $swap"

if [ -z "$swap" ]; then
    say deploy failed
    exit 1
fi

blue "\n1 business tokens"
test "tokens_query" "$(dfx --identity alice canister call swap tokens_query 2>&1)" '"ICP"' '"ckUSDT'
test "token_query" "$(dfx --identity alice canister call swap token_query "(principal \"$token_ICP\")" 2>&1)" '"Internet Computer"'
test "token_balance_of" "$(dfx --identity alice canister call swap token_balance_of "(principal \"$token_ICP\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '(0 : nat)'
test "tokens_balance_of" "$(dfx --identity alice canister call swap tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '( vec { record { principal "' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 0 : nat;}'

test "token_deposit" "$(dfx --identity default canister call swap token_deposit "(record { token=principal \"$token_ICP\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; amount_without_fee=100_000_000: nat }, null)" 2>&1)" '( variant { Err = variant { TransferFromError = variant { InsufficientAllowance = record { allowance = 0 : nat } } } }, )'
test "icrc2_approve ICP/default" "$(dfx --identity default canister call token_ICP icrc2_approve "(record { spender=record{owner=principal \"$swap\";}; amount=100_000_000:nat })" 2>&1)" '(variant { Ok = 1 : nat })'
test "icrc1_balance_of ICP/default" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(999_999_990_000 : nat)'
test "icrc1_balance_of ICP/alice" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "token_deposit" "$(dfx --identity default canister call swap token_deposit "(record { token=principal \"$token_ICP\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; amount_without_fee=100_000_000: nat }, null)" 2>&1)" '( variant { Err = variant { TransferFromError = variant { InsufficientAllowance = record { allowance = 100_000_000 : nat } } } }, )'
test "icrc2_approve ICP/default" "$(dfx --identity default canister call token_ICP icrc2_approve "(record { spender=record{owner=principal \"$swap\";}; amount=100_000_000_000:nat })" 2>&1)" '(variant { Ok = 2 : nat })'
test "icrc1_balance_of ICP/default" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(999_999_980_000 : nat)'
test "icrc1_balance_of ICP/alice" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "token_deposit" "$(dfx --identity default canister call swap token_deposit "(record { token=principal \"$token_ICP\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; amount_without_fee=50_000_000_000: nat }, null)" 2>&1)" '(variant { Ok = 3 : nat })'

test "icrc1_balance_of ICP/default" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(949_999_970_000 : nat)'
test "icrc1_balance_of ICP/alice" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(10_000 : nat)'
test "token_balance_of" "$(dfx --identity alice canister call swap token_balance_of "(principal \"$token_ICP\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '(50_000_000_000 : nat)'
test "tokens_balance_of" "$(dfx --identity alice canister call swap tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '( vec { record { principal "' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 50_000_000_000 : nat;}'

test "token_withdraw" "$(dfx --identity default canister call swap token_withdraw "(record { token=principal \"$token_ICP\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; amount_without_fee=100_000_000_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, null)" 2>&1)" '( variant { Err = variant { InsufficientBalance = record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 50_000_000_000 : nat; } } }, )'
test "token_withdraw" "$(dfx --identity default canister call swap token_withdraw "(record { token=principal \"$token_ICP\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; amount_without_fee=29_999_990_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, null)" 2>&1)" '(variant { Ok = 4 : nat })'

test "icrc1_balance_of ICP/default" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(979_999_960_000 : nat)'
test "icrc1_balance_of ICP/alice" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(20_000 : nat)'
test "token_balance_of" "$(dfx --identity alice canister call swap token_balance_of "(principal \"$token_ICP\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '(20_000_000_000 : nat)'
test "tokens_balance_of" "$(dfx --identity alice canister call swap tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '( vec { record { principal "' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 20_000_000_000 : nat;}'

blue "\n2 business pairs"
test "pairs_query" "$(dfx --identity alice canister call swap pairs_query 2>&1)" '(vec {})'
test "pair_query" "$(dfx --identity alice canister call swap pair_query "(record { amm = \"swap_v2_0.3%\"; pair = record { principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\"; principal \"cngnf-vqaaa-aaaar-qag4q-cai\"; }; })" >&1)" '(null)'
test "pair_create" "$(dfx --identity alice canister call swap pair_create "(record { pair=record{ principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\" })" 2>&1)" "Permission 'BusinessTokenPairCreate'"
test "pair_create" "$(dfx --identity default canister call swap pair_create "(record { pair=record{ principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\" })" 2>&1)" '(variant { Ok })'
test "pairs_query" "$(dfx --identity alice canister call swap pairs_query 2>&1)" '( vec { record { record { amm = "swap_v2_0.3%"; pair = record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; principal "cngnf-vqaaa-aaaar-qag4q-cai"; }; }; variant { SwapV2 = record { lp = variant { InnerLP = record { fee = 10_000 : nat; decimals = 7 : nat8; dummy_canister_id = principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; minimum_liquidity = 10_000_000 : nat; total_supply = 0 : nat; } }; block_timestamp_last = 0 : nat64; reserve0 = 0 : nat; reserve1 = 0 : nat; subaccount = "81c09f0abbdbab8ad406107db3d18588b667eb94f3be6a556ce36b8875cb8996"; price1_cumulative_last = 0 : nat; fee_rate = "3/1000"; k_last = 0 : nat; protocol_fee = opt "1/6"; price0_cumulative_last = 0 : nat; price_unit = 18_446_744_073_709_551_615 : nat; } }; }; }, )'
test "pair_query" "$(dfx --identity alice canister call swap pair_query "(record { amm = \"swap_v2_0.3%\"; pair = record { principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\"; principal \"cngnf-vqaaa-aaaar-qag4q-cai\"; }; })" >&1)" '( opt variant { SwapV2 = record { lp = variant { InnerLP = record { fee = 10_000 : nat; decimals = 7 : nat8; dummy_canister_id = principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; minimum_liquidity = 10_000_000 : nat; total_supply = 0 : nat; } }; block_timestamp_last = 0 : nat64; reserve0 = 0 : nat; reserve1 = 0 : nat; subaccount = "81c09f0abbdbab8ad406107db3d18588b667eb94f3be6a556ce36b8875cb8996"; price1_cumulative_last = 0 : nat; fee_rate = "3/1000"; k_last = 0 : nat; protocol_fee = opt "1/6"; price0_cumulative_last = 0 : nat; price_unit = 18_446_744_073_709_551_615 : nat; } }, )'

blue "\n3 business pair liquidity"
test "pair_liquidity_add" "$(dfx --identity alice canister call swap pair_liquidity_add "(record { from=record{owner=principal\"$DEFAULT\"}; pool=record { pair=record{ principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\" }; amount_desired=record{1:nat;1:nat}; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '( variant { Err = variant { NotOwner = principal "ko6sb-zwe67-hhmjq-x4f77-vk2os-h5cc4-lv55n-gmw3k-dv2cz-gkwgg-bqe" } }, )'
test "pair_liquidity_add" "$(dfx --identity default canister call swap pair_liquidity_add "(record { from=record{owner=principal\"$DEFAULT\"}; pool=record { pair=record{ principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\" }; amount_desired=record{10_000_000_000:nat;200_000_000_000:nat}; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '( variant { Err = variant { InsufficientBalance = record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 0 : nat; } } }, )'
test "icrc2_approve token_ckUSDT/default" "$(dfx --identity default canister call token_ckUSDT icrc2_approve "(record { spender=record{owner=principal \"$swap\";}; amount=900_000_000_000:nat })" 2>&1)" '(variant { Ok = 1 : nat })'
test "token_deposit" "$(dfx --identity default canister call swap token_deposit "(record { token=principal \"$token_ckUSDT\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; amount_without_fee=800_000_000_000: nat }, null)" 2>&1)" '(variant { Ok = 2 : nat })'
test "pair_liquidity_add" "$(dfx --identity default canister call swap pair_liquidity_add "(record { from=record{owner=principal\"$DEFAULT\"}; pool=record { pair=record{ principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\" }; amount_desired=record{20_000_000_000:nat;400_000_000_000:nat}; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '( variant { Ok = record { liquidity = 89_442_719_099 : nat; amount = record { 20_000_000_000 : nat; 400_000_000_000 : nat }; } }, )'
test "token_balance_of" "$(dfx --identity alice canister call swap token_balance_of "(principal \"$token_ICP\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '(0 : nat)'
test "token_balance_of" "$(dfx --identity alice canister call swap token_balance_of "(principal \"$token_ckUSDT\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '(400_000_000_000 : nat)'
test "pairs_query" "$(dfx --identity alice canister call swap pairs_query 2>&1)" '( vec { record { record { amm = "swap_v2_0.3%"; pair = record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; principal "cngnf-vqaaa-aaaar-qag4q-cai"; }; }; variant { SwapV2 = record { lp = variant { InnerLP = record { fee = 10_000 : nat; decimals = 7 : nat8; dummy_canister_id = principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; minimum_liquidity = 10_000_000 : nat; total_supply = 89_442_719_099 : nat; } }; block_timestamp_last = ' ' : nat64; reserve0 = 20_000_000_000 : nat; reserve1 = 400_000_000_000 : nat; subaccount = "81c09f0abbdbab8ad406107db3d18588b667eb94f3be6a556ce36b8875cb8996"; price1_cumulative_last = 0 : nat; fee_rate = "3/1000"; k_last = 0 : nat; protocol_fee = opt "1/6"; price0_cumulative_last = 0 : nat; price_unit = 18_446_744_073_709_551_615 : nat; } }; }; }, )'
test "pair_query" "$(dfx --identity alice canister call swap pair_query "(record { amm = \"swap_v2_0.3%\"; pair = record { principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\"; principal \"cngnf-vqaaa-aaaar-qag4q-cai\"; }; })" >&1)" '( opt variant { SwapV2 = record { lp = variant { InnerLP = record { fee = 10_000 : nat; decimals = 7 : nat8; dummy_canister_id = principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; minimum_liquidity = 10_000_000 : nat; total_supply = 89_442_719_099 : nat; } }; block_timestamp_last = ' ' : nat64; reserve0 = 20_000_000_000 : nat; reserve1 = 400_000_000_000 : nat; subaccount = "81c09f0abbdbab8ad406107db3d18588b667eb94f3be6a556ce36b8875cb8996"; price1_cumulative_last = 0 : nat; fee_rate = "3/1000"; k_last = 0 : nat; protocol_fee = opt "1/6"; price0_cumulative_last = 0 : nat; price_unit = 18_446_744_073_709_551_615 : nat; } }, )'
test "token_balance_of" "$(dfx --identity alice canister call swap token_balance_of "(principal \"kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '(89_442_719_099 : nat)'

blue "\n1.1 permission permission_query"
test "version" "$(dfx --identity alice canister call swap version 2>&1)" '(1 : nat32)'
test "permission_all" "$(dfx --identity alice canister call swap permission_all 2>&1)" 'vec { variant { Forbidden = "PauseQuery" }; variant { Permitted = "PauseReplace" }'
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_query" "$(dfx canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_update" "$(dfx --identity bob canister call swap permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "'PermissionUpdate' is required"
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" "'PermissionQuery' is required"
test "permission_query" "$(dfx canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_find_by_user" "$(dfx canister call swap permission_find_by_user "(principal \"$ALICE\")" 2>&1)" '( vec { "PauseQuery"; "PermissionUpdate"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_update" "$(dfx --identity alice canister call swap permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_query" "$(dfx canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'

blue "\n1.2 permission permission update"
test "permission_query" "$(dfx canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_find_by_user" "$(dfx canister call swap permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_find_by_user" "$(dfx canister call swap permission_find_by_user "(principal \"$ALICE\")" 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_find_by_user" "$(dfx --identity alice canister call swap permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" "'PermissionFind' is required"
test "permission_find_by_user" "$(dfx --identity alice canister call swap permission_find_by_user "(principal \"$ALICE\")" 2>&1)" "'PermissionFind' is required"

blue "\n1.3 permission roles"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateRolePermission=record{\"Admin\"; opt vec {\"PauseReplace\"; \"PauseQuery\"} } } })" 2>&1)" "()"
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; opt vec {\"Admin\"} } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" '( vec { "PauseReplace"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'

blue "\n2.1 pause permission"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call swap pause_query_reason 2>&1)" "(null)"
test "pause_replace" "$(dfx canister call swap pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx canister call swap pause_query_reason 2>&1)" "message = \"reason\""

blue "\n2.2 pause permission by alice"
test "pause_query" "$(dfx --identity alice canister call swap pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx --identity alice canister call swap pause_query_reason 2>&1)" "message = \"reason\""

blue "\n2.3 pause no permission"
test "pause_replace" "$(dfx --identity alice canister call swap pause_replace "(null)" 2>&1)" "'PauseReplace' is required"
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PauseReplace\";\"PauseQuery\" } } } })" 2>&1)" "()"
test "pause_replace" "$(dfx --identity alice canister call swap pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx --identity alice canister call swap pause_query 2>&1)" "'PauseQuery' is required"
test "pause_query_reason" "$(dfx --identity alice canister call swap pause_query_reason 2>&1)" "'PauseQuery' is required"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call swap pause_query_reason 2>&1)" "(null)"

blue "\n3 record no permission"
test "record_topics" "$(dfx --identity alice canister call swap record_topics 2>&1)" "'RecordFind' is required"
test "record_topics" "$(dfx canister call swap record_topics 2>&1)" '"Example"' '"CyclesCharge"'
test "record_find_by_page" "$(dfx canister call swap record_find_by_page "(record{page=1:nat64;size=1:nat32},opt record{topic=opt vec{\"Pause\"}})" 2>&1)" "record { total = "
test "record_migrate" "$(dfx canister call swap record_migrate "(1:nat32)" 2>&1)" "removed = 0"

blue "\n4 schedule"
test "schedule_find" "$(dfx --identity alice canister call swap schedule_find 2>&1)" "'ScheduleFind' is required"
test "schedule_find" "$(dfx canister call swap schedule_find 2>&1)" "(null)"
test "schedule_replace" "$(dfx --identity alice canister call swap schedule_replace "(opt (1000000000:nat64))" 2>&1)" "'ScheduleReplace' is required"
test "schedule_replace" "$(dfx canister call swap schedule_replace "(opt (1000000000:nat64))" 2>&1)" "()"
sleep 3
test "schedule_replace" "$(dfx canister call swap schedule_replace "(null)" 2>&1)" "()"
sleep 2
test "schedule_trigger" "$(dfx --identity alice canister call swap schedule_trigger 2>&1)" "'ScheduleTrigger' is required"
test "schedule_trigger" "$(dfx canister call swap schedule_trigger 2>&1)" "()"

blue "\n5 example business"
test "business_example_query" "$(dfx --identity alice canister call swap business_example_query 2>&1)" "\"\""
test "business_example_query" "$(dfx canister call swap business_example_query 2>&1)" "\"\""
test "business_example_set" "$(dfx --identity alice canister call swap business_example_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_set" "$(dfx canister call swap business_example_set "(\"test string\")" 2>&1)" "()"
test "business_example_query" "$(dfx --identity alice canister call swap business_example_query 2>&1)" "test string"
test "business_example_query" "$(dfx canister call swap business_example_query 2>&1)" "test string"

blue "\n6 test swap data"
test "pause_replace" "$(dfx canister call swap pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" swap
test "pause_replace" "$(dfx canister call swap pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call swap business_example_query 2>&1)" "test string"

blue "\n7 test swap cell"
test "business_example_cell_query" "$(dfx --identity alice canister call swap business_example_cell_query 2>&1)" "\"\""
test "business_example_cell_query" "$(dfx canister call swap business_example_cell_query 2>&1)" "\"\""
test "business_example_cell_set" "$(dfx --identity alice canister call swap business_example_cell_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_cell_set" "$(dfx canister call swap business_example_cell_set "(\"test string\")" 2>&1)" "()"
test "business_example_cell_query" "$(dfx --identity alice canister call swap business_example_cell_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx canister call swap business_example_cell_query 2>&1)" "test string"

blue "\n8 test swap vec"
test "business_example_vec_query" "$(dfx --identity alice canister call swap business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call swap business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_pop" "$(dfx --identity alice canister call swap business_example_vec_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_pop" "$(dfx canister call swap business_example_vec_pop "()" 2>&1)" "(null)"
test "business_example_vec_push" "$(dfx --identity alice canister call swap business_example_vec_push "(5: nat64)" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_push" "$(dfx canister call swap business_example_vec_push "(5: nat64)" 2>&1)" "()"
test "business_example_vec_query" "$(dfx --identity alice canister call swap business_example_vec_query 2>&1)" "(vec { record { vec_data = 5 : nat64 } })"
test "business_example_vec_query" "$(dfx canister call swap business_example_vec_query 2>&1)" "(vec { record { vec_data = 5 : nat64 } })"
test "business_example_vec_pop" "$(dfx --identity alice canister call swap business_example_vec_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_pop" "$(dfx canister call swap business_example_vec_pop "()" 2>&1)" "(opt record { vec_data = 5 : nat64 })"
test "business_example_vec_query" "$(dfx --identity alice canister call swap business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call swap business_example_vec_query 2>&1)" "(vec {})"

blue "\n9 test swap map"
test "business_example_map_query" "$(dfx --identity alice canister call swap business_example_map_query 2>&1)" "(vec {})"
test "business_example_map_query" "$(dfx canister call swap business_example_map_query 2>&1)" "(vec {})"
test "business_example_map_update" "$(dfx --identity alice canister call swap business_example_map_update "(1:nat64, opt \"111\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_map_update" "$(dfx canister call swap business_example_map_update "(1:nat64, opt \"111\")" 2>&1)" "(null)"
test "business_example_map_query" "$(dfx --identity alice canister call swap business_example_map_query 2>&1)" '(vec { record { 1 : nat64; "111" } })'
test "business_example_map_query" "$(dfx canister call swap business_example_map_query 2>&1)" '(vec { record { 1 : nat64; "111" } })'
test "business_example_map_update" "$(dfx canister call swap business_example_map_update "(1:nat64, opt \"123\")" 2>&1)" "(opt \"111\")"
test "business_example_map_update" "$(dfx canister call swap business_example_map_update "(1:nat64, null)" 2>&1)" "(opt \"123\")"
test "business_example_map_update" "$(dfx canister call swap business_example_map_update "(2:nat64, opt \"222\")" 2>&1)" "(null)"
test "business_example_map_query" "$(dfx --identity alice canister call swap business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_map_query" "$(dfx canister call swap business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'

blue "\n10 test swap log"
test "business_example_log_query" "$(dfx --identity alice canister call swap business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_query" "$(dfx canister call swap business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_update" "$(dfx --identity alice canister call swap business_example_log_update "(\"111\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_log_update" "$(dfx canister call swap business_example_log_update "(\"111\")" 2>&1)" "(0 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call swap business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_query" "$(dfx canister call swap business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_update" "$(dfx canister call swap business_example_log_update "(\"123\")" 2>&1)" "(1 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call swap business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_log_query" "$(dfx canister call swap business_example_log_query 2>&1)" '(vec { "111"; "123" })'

blue "\n11 test swap priority queue"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call swap business_example_priority_queue_query 2>&1)" "(vec {})"
test "business_example_priority_queue_query" "$(dfx canister call swap business_example_priority_queue_query 2>&1)" "(vec {})"
test "business_example_priority_queue_pop" "$(dfx --identity alice canister call swap business_example_priority_queue_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_pop" "$(dfx canister call swap business_example_priority_queue_pop "()" 2>&1)" "(null)"
test "business_example_priority_queue_push" "$(dfx --identity alice canister call swap business_example_priority_queue_push "(5: nat64)" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_push" "$(dfx canister call swap business_example_priority_queue_push "(5: nat64)" 2>&1)" "()"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call swap business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call swap business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_push" "$(dfx canister call swap business_example_priority_queue_push "(2: nat64)" 2>&1)" "()"
test "business_example_priority_queue_query" "$(dfx canister call swap business_example_priority_queue_query 2>&1)" "(vec { 2 : nat64; 5 : nat64 })"
test "business_example_priority_queue_pop" "$(dfx --identity alice canister call swap business_example_priority_queue_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_pop" "$(dfx canister call swap business_example_priority_queue_pop "()" 2>&1)" "(opt (2 : nat64))"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call swap business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call swap business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"

blue "\n12 test swap priority queue"
test "pause_replace" "$(dfx canister call swap pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" swap
test "pause_replace" "$(dfx canister call swap pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call swap business_example_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx --identity alice canister call swap business_example_cell_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx canister call swap business_example_cell_query 2>&1)" "test string"
test "business_example_vec_query" "$(dfx --identity alice canister call swap business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call swap business_example_vec_query 2>&1)" "(vec {})"
test "business_example_map_query" "$(dfx --identity alice canister call swap business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_map_query" "$(dfx canister call swap business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_log_query" "$(dfx --identity alice canister call swap business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_log_query" "$(dfx canister call swap business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_priority_queue_query" "$(dfx --identity alice canister call swap business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call swap business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"

# test completed

echo ""
green "=================== TEST COMPLETED AND SUCCESSFUL ==================="
echo ""

say test successful

# sleep 10000
# read -s -n1 -p "按任意键结束..."
