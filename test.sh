#!/usr/bin/env bash
start_time=$(date +%H:%M:%S)
start_time_s=$(date +%s)

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

find_string_position() {
    local haystack="$1"
    local needle="$2"
    local needle_length=${#needle}
    local haystack_length=${#haystack}
    local i
    for ((i = 0; i <= haystack_length - needle_length; i++)); do
        local substring="${haystack:i:needle_length}"
        if [[ "$substring" == "$needle" ]]; then
            echo $((i + 1))
            return
        fi
    done
    echo -1
}

function check {
    if [ -n "$3" ]; then
        if [[ $(echo $2 | grep -F "$3") != "" ]]; then
            green "✅ Passed: $1 -> $2 -> $3"
        else
            FILE=$(echo "$5" | cut -d' ' -f3)
            LINE_NUMBER=$(echo "$5" | cut -d' ' -f1)
            LINE=$(sed -n "${LINE_NUMBER}p" "$FILE")
            COL_NUMBER=$(find_string_position "$LINE" "$3")
            red "❌ Failed: $1"
            green "Expected: $3"
            yellow "     Got: $2"
            red "Line: $FILE:$LINE_NUMBER:$COL_NUMBER 👉 $4"
            exit 1
        fi
    fi
}

function test {
    tips="$1"
    result="$(echo $2 | tr -d '\n')"
    check "$tips" "$result" "$3" "1" "$(caller 0)"
    check "$tips" "$result" "$4" "2" "$(caller 0)"
    check "$tips" "$result" "$5" "3" "$(caller 0)"
    check "$tips" "$result" "$6" "4" "$(caller 0)"
    check "$tips" "$result" "$7" "5" "$(caller 0)"
    check "$tips" "$result" "$8" "6" "$(caller 0)"
    check "$tips" "$result" "$9" "7" "$(caller 0)"
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
            record { record { owner=principal \"$DEFAULT\"; subaccount=null }; 10_000_000_000_000_000_000:nat };\
            record { record { owner=principal \"$ALICE\"; subaccount=null }; 8_000_000_000_000_000_000:nat }\
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

blue "\n0 query balances"
test "icrc1_balance_of ICP/default" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(1_000_000_000_000 : nat)'
test "icrc1_balance_of ICP/alice" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ICP/bob" "$(dfx --identity default canister call token_ICP icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckBTC/default" "$(dfx --identity default canister call token_ckBTC icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(100_000_000 : nat)'
test "icrc1_balance_of ckBTC/alice" "$(dfx --identity default canister call token_ckBTC icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckBTC/bob" "$(dfx --identity default canister call token_ckBTC icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckETH/default" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(10_000_000_000_000_000_000 : nat)'
test "icrc1_balance_of ckETH/alice" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(8_000_000_000_000_000_000 : nat)'
test "icrc1_balance_of ckETH/bob" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckUSDT/default" "$(dfx --identity default canister call token_ckUSDT icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(100_000_000_000_000 : nat)'
test "icrc1_balance_of ckUSDT/alice" "$(dfx --identity default canister call token_ckUSDT icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of ckUSDT/bob" "$(dfx --identity default canister call token_ckUSDT icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of snsCHAT/default" "$(dfx --identity default canister call token_snsCHAT icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(1_000_000_000_000 : nat)'
test "icrc1_balance_of snsCHAT/alice" "$(dfx --identity default canister call token_snsCHAT icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(0 : nat)'
test "icrc1_balance_of snsCHAT/bob" "$(dfx --identity default canister call token_snsCHAT icrc1_balance_of "(record{owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'

# ! 1. 测试 core
red "\n=========== 1. core ===========\n"
dfx canister create core --specified-id "piwiu-wiaaa-aaaaj-azzka-cai" # --with-cycles 50T
dfx deploy --mode=reinstall --yes --argument "(null)" core
core=$(canister_id "core")
blue "Core Canister: $core"

if [ -z "$core" ]; then
    say deploy failed
    exit 1
fi

blue "\n🚩 1 business tokens"
test "tokens_query" "$(dfx --identity alice canister call core tokens_query 2>&1)" '"ICP"' '"ckUSDT'
test "token_query" "$(dfx --identity alice canister call core token_query "(principal \"$token_ICP\")" 2>&1)" '"Internet Computer"'
test "token_balance_of" "$(dfx --identity alice canister call core token_balance_of "(principal \"$token_ICP\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" 'You can only query your own balance'
test "token_balance_by" "$(dfx --identity default canister call core token_balance_by "(principal \"$token_ICP\", record { owner=principal \"$ALICE\"; subaccount=null})" 2>&1)" '(0 : nat)'
test "token_balance_of" "$(dfx --identity default canister call core token_balance_of "(principal \"$token_ICP\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '(0 : nat)'
test "tokens_balance_of" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '( vec { record { principal "' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 0 : nat;}'

blue "\n🚩 1.1 business tokens deposit"
test "token_deposit" "$(dfx --identity default canister call core token_deposit "(record { token=principal \"$token_ckETH\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; deposit_amount_without_fee=5_000_000_000_000_000_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, opt 100)" 2>&1)" 'Too many retries'
test "token_deposit" "$(dfx --identity default canister call core token_deposit "(record { token=principal \"$token_ckETH\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; deposit_amount_without_fee=5_000_000_000_000_000_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, null)" 2>&1)" '( variant { Err = variant { TransferFromError = variant { InsufficientAllowance = record { allowance = 0 : nat } } } }, )'
test "icrc2_approve token_ckETH/default" "$(dfx --identity default canister call token_ckETH icrc2_approve "(record { spender=record{owner=principal \"$core\";}; amount=1_000_000_000_000_000_000:nat })" 2>&1)" '(variant { Ok = 2 : nat })'
test "icrc1_balance_of token_ckETH/default" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(9_999_998_000_000_000_000 : nat)'
test "icrc1_balance_of token_ckETH/alice" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(8_000_000_000_000_000_000 : nat)'
test "token_deposit" "$(dfx --identity default canister call core token_deposit "(record { token=principal \"$token_ckETH\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; deposit_amount_without_fee=5_000_000_000_000_000_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, null)" 2>&1)" '( variant { Err = variant { TransferFromError = variant { InsufficientAllowance = record { allowance = 1_000_000_000_000_000_000 : nat; } } } }, )'
test "icrc2_approve token_ckETH/default" "$(dfx --identity default canister call token_ckETH icrc2_approve "(record { spender=record{owner=principal \"$core\";}; amount=10_000_000_000_000_000_000:nat })" 2>&1)" '(variant { Ok = 3 : nat })'
test "icrc1_balance_of token_ckETH/default" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(9_999_996_000_000_000_000 : nat)'
test "icrc1_balance_of token_ckETH/alice" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(8_000_000_000_000_000_000 : nat)'
test "🙈 request_trace_index_get" "$(dfx --identity alice canister call core request_trace_index_get 2>&1)" "Permission 'PauseReplace' is required"
test "request_trace_index_get" "$(dfx --identity default canister call core request_trace_index_get 2>&1)" '(0 : nat64, 0 : nat64)'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(0:nat64)" 2>&1)" '(null)'
test "🙈 block_token_get" "$(dfx --identity alice canister call core block_token_get "(1:nat64)" 2>&1)" 'Only Maintainers are allowed to query data'
test "block_token_get" "$(dfx --identity default canister call core block_token_get "(1:nat64)" 2>&1)" 'invalid block height'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(0:nat64)" 2>&1)" 'invalid block height'
test "token_deposit" "$(dfx --identity default canister call core token_deposit "(record { token=principal \"$token_ckETH\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; deposit_amount_without_fee=5_000_000_000_000_000_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, null)" 2>&1)" '(variant { Ok = 4 : nat })'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(0:nat64)" 2>&1)" '( variant { block = record { transaction = record { created = null; memo = null; operation = variant { deposit = record { to = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; token = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; from = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; amount = 5_000_000_000_000_000_000 : nat; } }; }; timestamp = ' ' : nat64; parent_hash = blob ""; } }, )'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(0:nat64)" 2>&1)" '( opt record { args = variant { token_deposit = record { arg = record { to = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; token = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; from = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; amount = 5_000_000_000_000_000_000 : nat; }; now = ' ' : nat64; created = null; memo = null; caller = principal "'"$DEFAULT"'"; } }; done = opt record { ' ' : nat64; variant { Ok = "4" }; }; traces = vec { record { ' ' : nat64; "*Deposit* `token:[ss2fx-dyaaa-aaaar-qacoq-cai], from:('"$DEFAULT"'.), to:('"$DEFAULT"'.), amount:5_000_000_000_000_000_000, height:4`"; }; record { ' ' : nat64; "Deposit Done." }; }; locks = record { token = opt true; swap = null; balances = opt vec { record { token = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; account = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; }; }; }; index = 0 : nat64; }, )'

blue "\n🚩 1.2 business tokens balance of"
test "icrc1_balance_of token_ckETH/default" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(4_999_994_000_000_000_000 : nat)'
test "icrc1_balance_of token_ckETH/alice" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(8_000_002_000_000_000_000 : nat)'
test "token_balance_of" "$(dfx --identity default canister call core token_balance_of "(principal \"$token_ckETH\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '(5_000_000_000_000_000_000 : nat)'
test "tokens_balance_of" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '( vec { record { principal "' 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 5_000_000_000_000_000_000 : nat;}'

blue "\n🚩 1.3 business tokens withdraw"
test "token_withdraw" "$(dfx --identity default canister call core token_withdraw "(record { token=principal \"$token_ckETH\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; withdraw_amount_without_fee=15_000_000_000_000_000_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, null)" 2>&1)" '( variant { Err = variant { InsufficientBalance = record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 5_000_000_000_000_000_000 : nat; } } }, )'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(1:nat64)" 2>&1)" '(null)'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(1:nat64)" 2>&1)" 'invalid block height'
test "token_withdraw" "$(dfx --identity default canister call core token_withdraw "(record { token=principal \"$token_ckETH\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; withdraw_amount_without_fee=999_998_000_000_000_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, null)" 2>&1)" '(variant { Ok = 5 : nat })'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(1:nat64)" 2>&1)" "( variant { block = record { transaction = record { created = null; memo = null; operation = variant { withdraw = record { to = record { owner = principal \"$DEFAULT\"; subaccount = null; }; token = principal \"ss2fx-dyaaa-aaaar-qacoq-cai\"; from = record { owner = principal \"$DEFAULT\"; subaccount = null; }; amount = 1_000_000_000_000_000_000 : nat; } }; }; timestamp = " " : nat64; parent_hash = blob \""
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(1:nat64)" 2>&1)" '( opt record { args = variant { token_withdraw = record { arg = record { to = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; token = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; from = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; amount = 1_000_000_000_000_000_000 : nat; }; now = ' ' : nat64; created = null; memo = null; caller = principal "'"$DEFAULT"'"; } }; done = opt record { ' ' : nat64; variant { Ok = "5" }; }; traces = vec { record { ' ' : nat64; "*Withdraw* `token:[ss2fx-dyaaa-aaaar-qacoq-cai], to:('"$DEFAULT"'.), amount:1_000_000_000_000_000_000, height:5`"; }; record { ' ' : nat64; "Withdraw Done." }; }; locks = record { token = opt true; swap = null; balances = opt vec { record { token = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; account = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; }; }; }; index = 1 : nat64; }, )'

blue "\n🚩 1.4 business tokens balance of"
test "icrc1_balance_of token_ckETH/default" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$DEFAULT\"})" 2>&1)" '(5_999_992_000_000_000_000 : nat)'
test "icrc1_balance_of token_ckETH/alice" "$(dfx --identity default canister call token_ckETH icrc1_balance_of "(record{owner=principal \"$ALICE\"})" 2>&1)" '(8_000_004_000_000_000_000 : nat)'
test "token_balance_of" "$(dfx --identity default canister call core token_balance_of "(principal \"$token_ckETH\", record { owner=principal \"$DEFAULT\"})" 2>&1)" '(4_000_000_000_000_000_000 : nat)'
test "tokens_balance_of" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '( vec { record { principal "' 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 4_000_000_000_000_000_000 : nat;}'

blue "\n🚩 1.5 business tokens transfer"
test "token_balance_of" "$(dfx --identity alice canister call core token_balance_of "(principal \"$token_ckETH\", record { owner=principal \"$ALICE\"; subaccount=null})" 2>&1)" '(0 : nat)'
test "token_transfer" "$(dfx --identity default canister call core token_transfer "(record { token=principal \"$token_ckETH\"; from=record {owner=principal \"$DEFAULT\"}; transfer_amount_without_fee=1_000_000_000_000_000_000_000 : nat; to=record {owner=principal \"$ALICE\"} }, null)" 2>&1)" '( variant { Err = variant { InsufficientBalance = record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 4_000_000_000_000_000_000 : nat; } } }, )'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(2:nat64)" 2>&1)" '(null)'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(2:nat64)" 2>&1)" 'invalid block height'
test "token_transfer" "$(dfx --identity default canister call core token_transfer "(record { token=principal \"$token_ckETH\"; from=record {owner=principal \"$DEFAULT\"}; transfer_amount_without_fee=1_000_000_000_000_000_000 : nat; to=record {owner=principal \"$ALICE\"} }, null)" 2>&1)" '(variant { Ok = 1_000_000_000_000_000_000 : nat })'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(2:nat64)" 2>&1)" "( variant { block = record { transaction = record { created = null; memo = null; operation = variant { transfer = record { to = record { owner = principal \"$ALICE\"; subaccount = null; }; fee = null; token = principal \"ss2fx-dyaaa-aaaar-qacoq-cai\"; from = record { owner = principal \"$DEFAULT\"; subaccount = null; }; amount = 1_000_000_000_000_000_000 : nat; } }; }; timestamp = " " : nat64; parent_hash = blob \""
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(2:nat64)" 2>&1)" '( opt record { args = variant { token_transfer = record { arg = record { to = record { owner = principal "'"$ALICE"'"; subaccount = null; }; fee = null; token = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; from = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; amount = 1_000_000_000_000_000_000 : nat; }; now = ' ' : nat64; created = null; memo = null; caller = principal "'"$DEFAULT"'"; } }; done = opt record { ' ' : nat64; variant { Ok = "1_000_000_000_000_000_000" }; }; traces = vec { record { ' ' : nat64; "*Transfer* `token:[ss2fx-dyaaa-aaaar-qacoq-cai], from:('"$DEFAULT"'.), to:('"$ALICE"'.), amount:1_000_000_000_000_000_000`"; }; record { ' ' : nat64; "Transfer Done: 1_000_000_000_000_000_000."; }; }; locks = record { token = opt true; swap = null; balances = opt vec { record { token = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; account = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; }; record { token = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; account = record { owner = principal "'"$ALICE"'"; subaccount = null; }; }; }; }; index = 2 : nat64; }, )'
test "token_balance_of" "$(dfx --identity default canister call core token_balance_of "(principal \"$token_ckETH\", record { owner=principal \"$DEFAULT\"})" 2>&1)" '(3_000_000_000_000_000_000 : nat)'
test "token_balance_of" "$(dfx --identity alice canister call core token_balance_of "(principal \"$token_ckETH\", record { owner=principal \"$ALICE\"; subaccount=null})" 2>&1)" '(1_000_000_000_000_000_000 : nat)'

blue "\n🚩 2 business pairs"
test "pairs_query" "$(dfx --identity alice canister call core pairs_query 2>&1)" '(vec {})'
test "pair_query" "$(dfx --identity alice canister call core pair_query "(record { token0 = principal \"$token_ckETH\"; token1 = principal \"$token_ckUSDT\"; amm = \"swap_v2_0.3%\"; })" >&1)" '(null)'
test "pair_create" "$(dfx --identity alice canister call core pair_create "(record { pool=record { token0 = principal \"$token_ckETH\"; token1 = principal \"$token_ckUSDT\"; amm=\"swap_v2_0.3%\"; } })" 2>&1)" "Permission 'BusinessTokenPairCreate'"
test "block_swap_get" "$(dfx --identity alice canister call core block_swap_get "(1:nat64)" 2>&1)" 'invalid block height'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(3:nat64)" 2>&1)" '(null)'
test "👁︎ block_swap_get" "$(dfx --identity default canister call core block_swap_get "(0:nat64)" 2>&1)" 'invalid block height'
test "pair_create" "$(dfx --identity default canister call core pair_create "(record { pool=record { token0 = principal \"$token_ckETH\"; token1 = principal \"$token_ckUSDT\"; amm=\"swap_v2_0.3%\"; }})" 2>&1)" '( variant { Ok = variant { swap_v2 = record { lp = variant { inner = record { fee = 100_000_000 : nat; decimals = 12 : nat8; dummy_canister_id = principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; minimum_liquidity = 100_000_000_000 : nat; total_supply = 0 : nat; } }; price_cumulative_exponent = 64 : nat8; block_timestamp_last = 0 : nat64; reserve0 = "0"; reserve1 = "0"; subaccount = "11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c"; price1_cumulative_last = "0"; token0 = "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = "cngnf-vqaaa-aaaar-qag4q-cai"; fee_rate = "3/1000"; k_last = "0"; protocol_fee = opt "1/6"; price0_cumulative_last = "0"; } } }, )'
test "👁︎ block_swap_get" "$(dfx --identity default canister call core block_swap_get "(0:nat64)" 2>&1)" '( variant { block = record { transaction = record { created = null; memo = null; operation = variant { pair = variant { create = record { pa = record { amm = variant { "swap_v2_0.3%" }; pair = record { token0 = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = principal "cngnf-vqaaa-aaaar-qag4q-cai"; }; }; creator = principal "'"$DEFAULT"'"; } } }; }; timestamp = ' ' : nat64; parent_hash = blob ""; } }, )'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(3:nat64)" 2>&1)" '( opt record { args = variant { pair_liquidity_create = record { arg = record { amm = variant { "swap_v2_0.3%" }; pair = record { token0 = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = principal "cngnf-vqaaa-aaaar-qag4q-cai"; }; }; now = ' ' : nat64; created = null; memo = null; caller = principal "'"$DEFAULT"'"; } }; done = opt record { ' ' : nat64; variant { Ok = "{\"swap_v2\":{\"subaccount\":\"11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c\",\"fee_rate\":\"3/1000\",\"token0\":\"ss2fx-dyaaa-aaaar-qacoq-cai\",\"token1\":\"cngnf-vqaaa-aaaar-qag4q-cai\",\"reserve0\":\"0\",\"reserve1\":\"0\",\"block_timestamp_last\":0,\"price_cumulative_exponent\":64,\"price0_cumulative_last\":\"0\",\"price1_cumulative_last\":\"0\",\"k_last\":\"0\",\"lp\":{\"inner\":{\"dummy_canister_id\":\"vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4\",\"total_supply\":[],\"decimals\":12,\"fee\":[100000000],\"minimum_liquidity\":[1215752192,23]}},\"protocol_fee\":\"1/6\"}}" }; }; traces = vec { record { ' ' : nat64; "*CreateTokenPair* `token0:[ss2fx-dyaaa-aaaar-qacoq-cai], token1:[cngnf-vqaaa-aaaar-qag4q-cai], amm:swap_v2_0.3%, subaccount:(11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c), dummyCanisterId:[vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4]`"; }; }; locks = record { token = null; swap = opt true; balances = null }; index = 3 : nat64; }, )'
test "pairs_query" "$(dfx --identity alice canister call core pairs_query 2>&1)" '( vec { record { record { amm = "swap_v2_0.3%"; token0 = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = principal "cngnf-vqaaa-aaaar-qag4q-cai"; }; variant { swap_v2 = record { lp = variant { inner = record { fee = 100_000_000 : nat; decimals = 12 : nat8; dummy_canister_id = principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; minimum_liquidity = 100_000_000_000 : nat; total_supply = 0 : nat; } }; price_cumulative_exponent = 64 : nat8; block_timestamp_last = 0 : nat64; reserve0 = "0"; reserve1 = "0"; subaccount = "11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c"; price1_cumulative_last = "0"; token0 = "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = "cngnf-vqaaa-aaaar-qag4q-cai"; fee_rate = "3/1000"; k_last = "0"; protocol_fee = opt "1/6"; price0_cumulative_last = "0"; } }; }; }, )'
test "pair_query" "$(dfx --identity alice canister call core pair_query "(record { token0 = principal \"$token_ckETH\"; token1 = principal \"$token_ckUSDT\"; amm = \"swap_v2_0.3%\"; })" >&1)" '( opt variant { swap_v2 = record { lp = variant { inner = record { fee = 100_000_000 : nat; decimals = 12 : nat8; dummy_canister_id = principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; minimum_liquidity = 100_000_000_000 : nat; total_supply = 0 : nat; } }; price_cumulative_exponent = 64 : nat8; block_timestamp_last = 0 : nat64; reserve0 = "0"; reserve1 = "0"; subaccount = "11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c"; price1_cumulative_last = "0"; token0 = "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = "cngnf-vqaaa-aaaar-qag4q-cai"; fee_rate = "3/1000"; k_last = "0"; protocol_fee = opt "1/6"; price0_cumulative_last = "0"; } }, )'
test "tokens_balance_of" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '( vec { record { principal "' 'record { principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; 0 : nat;}'
token_ckETH_token_ckUSDT_subaccount="\11\df\fa\35\fd\42\81\0f\9b\24\9c\39\74\9d\4a\dc\73\a3\97\f7\99\f9\07\03\bf\6d\8f\cc\1e\f7\d9\2c"
test "tokens_balance_of" "$(dfx --identity alice canister call core tokens_balance_of "(record { owner=principal \"$core\"; subaccount=opt blob \"$token_ckETH_token_ckUSDT_subaccount\"})" 2>&1)" '( vec { record { principal "' 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 0 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 0 : nat;}'

blue "\n🚩 2.1 business pair liquidity add"
test "pair_liquidity_add" "$(dfx --identity alice canister call core pair_liquidity_add "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { token = record { principal \"$token_ckETH\"; principal \"$token_ckUSDT\" }; amm=\"swap_v2_0.3%\"; }; amount_desired=record{1:nat;1:nat}; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" "( variant { Err = variant { NotOwner = principal \"$DEFAULT\" } }, )"
test "pair_liquidity_add" "$(dfx --identity default canister call core pair_liquidity_add "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { token = record { principal \"$token_ckETH\"; principal \"$token_ckUSDT\" }; amm=\"swap_v2_0.3%\"; }; amount_desired=record{2_000_000_000_000_000_000:nat;200_000_000_000:nat}; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '( variant { Err = variant { InsufficientBalance = record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 0 : nat; } } }, )'
test "icrc2_approve token_ckUSDT/default" "$(dfx --identity default canister call token_ckUSDT icrc2_approve "(record { spender=record{owner=principal \"$core\";}; amount=900_000_000_000:nat })" 2>&1)" '(variant { Ok = 1 : nat })'
test "token_deposit" "$(dfx --identity default canister call core token_deposit "(record { token=principal \"$token_ckUSDT\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; deposit_amount_without_fee=800_000_000_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, null)" 2>&1)" '(variant { Ok = 2 : nat })'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(3:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "deposit": { "amount": "800_000_000_000", "from": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "to": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "cngnf-vqaaa-aaaar-qag4q-cai" } } } } }'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(4:nat64)" 2>&1)" '( opt record { args = variant { token_deposit = record { arg = record { to = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; token = principal "cngnf-vqaaa-aaaar-qag4q-cai"; from = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; amount = 800_000_000_000 : nat; }; now = ' ' : nat64; created = null; memo = null; caller = principal "'"$DEFAULT"'"; } }; done = opt record { ' ' : nat64; variant { Ok = "2" }; }; traces = vec { record { ' ' : nat64; "*Deposit* `token:[cngnf-vqaaa-aaaar-qag4q-cai], from:('"$DEFAULT"'.), to:('"$DEFAULT"'.), amount:800_000_000_000, height:2`"; }; record { ' ' : nat64; "Deposit Done." }; }; locks = record { token = opt true; swap = null; balances = opt vec { record { token = principal "cngnf-vqaaa-aaaar-qag4q-cai"; account = record { owner = principal "'"$DEFAULT"'"; subaccount = null; }; }; }; }; index = 4 : nat64; }, )'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(5:nat64)" 2>&1)" '(null)'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(4:nat64)" --output json 2>&1)" 'invalid block height'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(5:nat64)" --output json 2>&1)" 'invalid block height'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(6:nat64)" --output json 2>&1)" 'invalid block height'
test "👁︎ block_swap_get" "$(dfx --identity default canister call core block_swap_get "(1:nat64)" --output json 2>&1)" 'invalid block height'
test "pair_liquidity_add" "$(dfx --identity default canister call core pair_liquidity_add "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { token = record { principal \"$token_ckETH\"; principal \"$token_ckUSDT\" }; amm=\"swap_v2_0.3%\"; }; amount_desired=record{2_000_000_000_000_000_000:nat;400_000_000_000:nat}; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '( variant { Ok = record { liquidity = 894_427_190_999_915 : nat; amount = record { 2_000_000_000_000_000_000 : nat; 400_000_000_000 : nat; }; } }, )'
test "👁︎ block_swap_get" "$(dfx --identity default canister call core block_swap_get "(1:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "pair": { "swap_v2": { "mint": { "amount": "894_427_190_999_915", "amount0": "2_000_000_000_000_000_000", "amount1": "400_000_000_000", "from": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "pa": { "amm": { "swap_v2_0.3%": null }, "pair": { "token0": "ss2fx-dyaaa-aaaar-qacoq-cai", "token1": "cngnf-vqaaa-aaaar-qag4q-cai" } }, "to": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4", "token0": "ss2fx-dyaaa-aaaar-qacoq-cai", "token1": "cngnf-vqaaa-aaaar-qag4q-cai" } } } } } } }'
test "👁︎ block_swap_get" "$(dfx --identity default canister call core block_swap_get "(2:nat64)" --output json 2>&1)" 'invalid block height'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(4:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "transfer": { "amount": "2_000_000_000_000_000_000", "fee": [], "from": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "to": { "owner": "piwiu-wiaaa-aaaaj-azzka-cai", "subaccount": [ [ 17, 223, 250, 53, 253, 66, 129, 15, 155, 36, 156, 57, 116, 157, 74, 220, 115, 163, 151, 247, 153, 249, 7, 3, 191, 109, 143, 204, 30, 247, 217, 44 ] ] }, "token": "ss2fx-dyaaa-aaaar-qacoq-cai" } } } } }'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(5:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "transfer": { "amount": "400_000_000_000", "fee": [], "from": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "to": { "owner": "piwiu-wiaaa-aaaaj-azzka-cai", "subaccount": [ [ 17, 223, 250, 53, 253, 66, 129, 15, 155, 36, 156, 57, 116, 157, 74, 220, 115, 163, 151, 247, 153, 249, 7, 3, 191, 109, 143, 204, 30, 247, 217, 44 ] ] }, "token": "cngnf-vqaaa-aaaar-qag4q-cai" } } } } }'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(6:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "deposit": { "amount": "894_427_190_999_915", "from": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "to": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4" } } } } }'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(7:nat64)" --output json 2>&1)" 'invalid block height'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(5:nat64)" --output json 2>&1)" '[ { "args": { "pair_liquidity_add": { "arg": { "amount_a_desired": "2_000_000_000_000_000_000", "amount_a_min": "1", "amount_b_desired": "400_000_000_000", "amount_b_min": "1", "from": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "pa": { "amm": { "swap_v2_0.3%": null }, "pair": { "token0": "ss2fx-dyaaa-aaaar-qacoq-cai", "token1": "cngnf-vqaaa-aaaar-qag4q-cai" } }, "self_canister": "piwiu-wiaaa-aaaaj-azzka-cai", "to": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token_a": "ss2fx-dyaaa-aaaar-qacoq-cai", "token_b": "cngnf-vqaaa-aaaar-qag4q-cai" }, "caller": "'"$DEFAULT"'", "created": [], "memo": [], "now": "' '" } }, "done": [ { "0": "' '", "1": { "Ok": "{\"amount\":[\"2_000_000_000_000_000_000\",\"400_000_000_000\"],\"liquidity\":\"894_427_190_999_915\"}" } } ], "index": "5", "locks": { "balances": [ [ { "account": { "owner": "piwiu-wiaaa-aaaaj-azzka-cai", "subaccount": [ [ 17, 223, 250, 53, 253, 66, 129, 15, 155, 36, 156, 57, 116, 157, 74, 220, 115, 163, 151, 247, 153, 249, 7, 3, 191, 109, 143, 204, 30, 247, 217, 44 ] ] }, "token": "ss2fx-dyaaa-aaaar-qacoq-cai" }, { "account": { "owner": "piwiu-wiaaa-aaaaj-azzka-cai", "subaccount": [ [ 17, 223, 250, 53, 253, 66, 129, 15, 155, 36, 156, 57, 116, 157, 74, 220, 115, 163, 151, 247, 153, 249, 7, 3, 191, 109, 143, 204, 30, 247, 217, 44 ] ] }, "token": "cngnf-vqaaa-aaaar-qag4q-cai" }, { "account": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4" }, { "account": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "ss2fx-dyaaa-aaaar-qacoq-cai" }, { "account": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "cngnf-vqaaa-aaaar-qag4q-cai" } ] ], "swap": [ true ], "token": [ true ] }, "traces": [ { "0": "' '", "1": "*Add Liquidity* `tokenA:[ss2fx-dyaaa-aaaar-qacoq-cai], tokenB:[cngnf-vqaaa-aaaar-qag4q-cai], amm:swap_v2_0.3%, required: 1 <= amount_a <= 2_000_000_000_000_000_000 && 1 <= amount_b <= 400_000_000_000`" }, { "0": "' '", "1": "*Add Liquidity* `amount_a:2_000_000_000_000_000_000, amount_b:400_000_000_000`" }, { "0": "' '", "1": "*Transfer Token* `token:[ss2fx-dyaaa-aaaar-qacoq-cai], from:('"$DEFAULT"'.), to:(piwiu-wiaaa-aaaaj-azzka-cai.11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c), amount:2_000_000_000_000_000_000, done:2_000_000_000_000_000_000`" }, { "0": "' '", "1": "*Transfer Token* `token:[cngnf-vqaaa-aaaar-qag4q-cai], from:('"$DEFAULT"'.), to:(piwiu-wiaaa-aaaaj-azzka-cai.11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c), amount:400_000_000_000, done:400_000_000_000`" }, { "0": "' '", "1": "*Mint Liquidity (Deposit)* `token:[vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4], from<pay 2 tokens>:('"$DEFAULT"'.), to<got liquidity>:('"$DEFAULT"'.), amount:894_427_190_999_915`" }, { "0": "' '", "1": "*Mint Liquidity* `token:[vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4], to:('"$DEFAULT"'.), amount:894_427_190_999_915`" }, { "0": "' '", "1": "Token Pair Add liquidity Done." } ] } ]'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(6:nat64)" 2>&1)" '(null)'
test "pairs_query" "$(dfx --identity alice canister call core pairs_query 2>&1)" '( vec { record { record { amm = "swap_v2_0.3%"; token0 = principal "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = principal "cngnf-vqaaa-aaaar-qag4q-cai"; }; variant { swap_v2 = record { lp = variant { inner = record { fee = 100_000_000 : nat; decimals = 12 : nat8; dummy_canister_id = principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; minimum_liquidity = 100_000_000_000 : nat; total_supply = 894_427_190_999_915 : nat; } }; price_cumulative_exponent = 64 : nat8; block_timestamp_last = ' ' : nat64; reserve0 = "2_000_000_000_000_000_000"; reserve1 = "400_000_000_000"; subaccount = "11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c"; price1_cumulative_last = "0"; token0 = "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = "cngnf-vqaaa-aaaar-qag4q-cai"; fee_rate = "3/1000"; k_last = "0"; protocol_fee = opt "1/6"; price0_cumulative_last = "0"; } }; }; }, )'
test "pair_query" "$(dfx --identity alice canister call core pair_query "(record { token0 = principal \"$token_ckETH\"; token1 = principal \"$token_ckUSDT\"; amm = \"swap_v2_0.3%\"; })" >&1)" '( opt variant { swap_v2 = record { lp = variant { inner = record { fee = 100_000_000 : nat; decimals = 12 : nat8; dummy_canister_id = principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; minimum_liquidity = 100_000_000_000 : nat; total_supply = 894_427_190_999_915 : nat; } }; price_cumulative_exponent = 64 : nat8; block_timestamp_last = ' ' : nat64; reserve0 = "2_000_000_000_000_000_000"; reserve1 = "400_000_000_000"; subaccount = "11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c"; price1_cumulative_last = "0"; token0 = "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = "cngnf-vqaaa-aaaar-qag4q-cai"; fee_rate = "3/1000"; k_last = "0"; protocol_fee = opt "1/6"; price0_cumulative_last = "0"; } }, )'
test "tokens_balance_of user default" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\";                                    subaccount=null})" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 1_000_000_000_000_000_000 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 400_000_000_000 : nat;}' 'record { principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; 894_427_190_999_915 : nat;}'
test "tokens_balance_of pool" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$core\"; subaccount=opt blob \"$token_ckETH_token_ckUSDT_subaccount\" })" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 2_000_000_000_000_000_000 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 400_000_000_000 : nat;}' 'record { principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; 0 : nat;}'

blue "\n🚩 2.2 business pair liquidity remove"
test "pair_liquidity_remove" "$(dfx --identity default canister call core pair_liquidity_remove "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { token = record { principal \"$token_ckETH\"; principal \"$token_ckUSDT\" }; amm=\"swap_v2_0.3%\"; }; liquidity=1_894_427_190_999_915:nat; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '(variant { Err = variant { Liquidity = "INSUFFICIENT_LIQUIDITY" } })'
test "pair_liquidity_remove" "$(dfx --identity default canister call core pair_liquidity_remove "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { token = record { principal \"$token_ckETH\"; principal \"$token_ckUSDT\" }; amm=\"swap_v2_0.3%\"; }; liquidity=894_427_190_999_915:nat; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '(variant { Err = variant { Liquidity = "REMAIN_TOTAL_LIQUIDITY_TOO_SMALL" } })'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(6:nat64)" 2>&1)" '(null)'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(7:nat64)" --output json 2>&1)" 'invalid block height'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(8:nat64)" --output json 2>&1)" 'invalid block height'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(9:nat64)" --output json 2>&1)" 'invalid block height'
test "👁︎ block_swap_get" "$(dfx --identity default canister call core block_swap_get "(2:nat64)" --output json 2>&1)" 'invalid block height'
test "pair_liquidity_remove" "$(dfx --identity default canister call core pair_liquidity_remove "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { token = record { principal \"$token_ckETH\"; principal \"$token_ckUSDT\" }; amm=\"swap_v2_0.3%\"; }; liquidity=447_213_595_499_958:nat; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '( variant { Ok = record { amount = record { 1_000_000_000_000_001_118 : nat; 200_000_000_000 : nat; }; } }, )'
test "👁︎ block_swap_get" "$(dfx --identity default canister call core block_swap_get "(2:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "pair": { "swap_v2": { "burn": { "amount": "447_213_595_499_958", "amount0": "1_000_000_000_000_001_118", "amount1": "200_000_000_000", "from": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "pa": { "amm": { "swap_v2_0.3%": null }, "pair": { "token0": "ss2fx-dyaaa-aaaar-qacoq-cai", "token1": "cngnf-vqaaa-aaaar-qag4q-cai" } }, "to": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4", "token0": "ss2fx-dyaaa-aaaar-qacoq-cai", "token1": "cngnf-vqaaa-aaaar-qag4q-cai" } } } } } } }'
test "👁︎ block_swap_get" "$(dfx --identity default canister call core block_swap_get "(3:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "pair": { "swap_v2": { "cumulative_price": { "block_timestamp": "' '", "pa": { "amm": { "swap_v2_0.3%": null }, "pair": { "token0": "ss2fx-dyaaa-aaaar-qacoq-cai", "token1": "cngnf-vqaaa-aaaar-qag4q-cai" } }, "price0_cumulative": "' '", "price1_cumulative": "' '", "price_cumulative_exponent": 64 } } } } } } }'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(7:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "withdraw": { "amount": "447_213_595_499_958", "from": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "to": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4" } } } } }'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(8:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "transfer": { "amount": "1_000_000_000_000_001_118", "fee": [], "from": { "owner": "piwiu-wiaaa-aaaaj-azzka-cai", "subaccount": [ [ 17, 223, 250, 53, 253, 66, 129, 15, 155, 36, 156, 57, 116, 157, 74, 220, 115, 163, 151, 247, 153, 249, 7, 3, 191, 109, 143, 204, 30, 247, 217, 44 ] ] }, "to": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "ss2fx-dyaaa-aaaar-qacoq-cai" } } } } }'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(9:nat64)" --output json 2>&1)" '{ "block": { "parent_hash": [ ' ' ], "timestamp": "' '", "transaction": { "created": [], "memo": [], "operation": { "transfer": { "amount": "200_000_000_000", "fee": [], "from": { "owner": "piwiu-wiaaa-aaaaj-azzka-cai", "subaccount": [ [ 17, 223, 250, 53, 253, 66, 129, 15, 155, 36, 156, 57, 116, 157, 74, 220, 115, 163, 151, 247, 153, 249, 7, 3, 191, 109, 143, 204, 30, 247, 217, 44 ] ] }, "to": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "cngnf-vqaaa-aaaar-qag4q-cai" } } } } }'
test "👁︎ block_token_get" "$(dfx --identity default canister call core block_token_get "(10:nat64)" --output json 2>&1)" 'invalid block height'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(6:nat64)" --output json 2>&1)" '[ { "args": { "pair_liquidity_remove": { "arg": { "amount_a_min": "1", "amount_b_min": "1", "from": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "liquidity": "447_213_595_499_958", "pa": { "amm": { "swap_v2_0.3%": null }, "pair": { "token0": "ss2fx-dyaaa-aaaar-qacoq-cai", "token1": "cngnf-vqaaa-aaaar-qag4q-cai" } }, "self_canister": "piwiu-wiaaa-aaaaj-azzka-cai", "to": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token_a": "ss2fx-dyaaa-aaaar-qacoq-cai", "token_b": "cngnf-vqaaa-aaaar-qag4q-cai" }, "caller": "'"$DEFAULT"'", "created": [], "memo": [], "now": "' '" } }, "done": [ { "0": "' '", "1": { "Ok": "{\"amount\":[\"1_000_000_000_000_001_118\",\"200_000_000_000\"]}" } } ], "index": "6", "locks": { "balances": [ [ { "account": { "owner": "piwiu-wiaaa-aaaaj-azzka-cai", "subaccount": [ [ 17, 223, 250, 53, 253, 66, 129, 15, 155, 36, 156, 57, 116, 157, 74, 220, 115, 163, 151, 247, 153, 249, 7, 3, 191, 109, 143, 204, 30, 247, 217, 44 ] ] }, "token": "ss2fx-dyaaa-aaaar-qacoq-cai" }, { "account": { "owner": "piwiu-wiaaa-aaaaj-azzka-cai", "subaccount": [ [ 17, 223, 250, 53, 253, 66, 129, 15, 155, 36, 156, 57, 116, 157, 74, 220, 115, 163, 151, 247, 153, 249, 7, 3, 191, 109, 143, 204, 30, 247, 217, 44 ] ] }, "token": "cngnf-vqaaa-aaaar-qag4q-cai" }, { "account": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4" }, { "account": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "ss2fx-dyaaa-aaaar-qacoq-cai" }, { "account": { "owner": "'"$DEFAULT"'", "subaccount": [] }, "token": "cngnf-vqaaa-aaaar-qag4q-cai" } ] ], "swap": [ true ], "token": [ true ] }, "traces": [ { "0": "' '", "1": "*Remove Liquidity* `tokenA:[ss2fx-dyaaa-aaaar-qacoq-cai], tokenB:[cngnf-vqaaa-aaaar-qag4q-cai], amm:swap_v2_0.3%, liquidity:447_213_595_499_958, required: 1 <= amount_a && 1 <= amount_b`" }, { "0": "' '", "1": "*Burn Liquidity (Withdraw)*. `token:[vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4], from<pay liquidity>:('"$DEFAULT"'.), to<got 2 tokens>:('"$DEFAULT"'.) amount:447_213_595_499_958`" }, { "0": "' '", "1": "*Burn Liquidity* `token:[vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4], from:('"$DEFAULT"'.), amount:447_213_595_499_958`" }, { "0": "' '", "1": "*Transfer Token* `token:[ss2fx-dyaaa-aaaar-qacoq-cai], from:(piwiu-wiaaa-aaaaj-azzka-cai.11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c), to:('"$DEFAULT"'.), amount:1_000_000_000_000_001_118, done:1_000_000_000_000_001_118`" }, { "0": "' '", "1": "*Transfer Token* `token:[cngnf-vqaaa-aaaar-qag4q-cai], from:(piwiu-wiaaa-aaaaj-azzka-cai.11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c), to:('"$DEFAULT"'.), amount:200_000_000_000, done:200_000_000_000`" }, { "0": "' '", "1": "*Pair Cumulative Price* `pa:(ss2fx-dyaaa-aaaar-qacoq-cai,cngnf-vqaaa-aaaar-qag4q-cai,swap_v2_0.3%), timestamp:' ', exponent:64, price0:' ', price1:' '`" }, { "0": "' '", "1": "Token Pair Remove liquidity Done." } ] } ]'
test "👁︎ request_trace_get" "$(dfx --identity default canister call core request_trace_get "(7:nat64)" 2>&1)" '(null)'
test "pair_query" "$(dfx --identity alice canister call core pair_query "(record { token0 = principal \"$token_ckETH\"; token1 = principal \"$token_ckUSDT\"; amm = \"swap_v2_0.3%\"; })" >&1)" '( opt variant { swap_v2 = record { lp = variant { inner = record { fee = 100_000_000 : nat; decimals = 12 : nat8; dummy_canister_id = principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; minimum_liquidity = 100_000_000_000 : nat; total_supply = 447_213_595_499_957 : nat; } }; price_cumulative_exponent = 64 : nat8; block_timestamp_last = ' ' : nat64; reserve0 = "999_999_999_999_998_882"; reserve1 = "200_000_000_000"; subaccount = "11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c"; price1_cumulative_last = "' '"; token0 = "ss2fx-dyaaa-aaaar-qacoq-cai"; token1 = "cngnf-vqaaa-aaaar-qag4q-cai"; fee_rate = "3/1000"; k_last = "0"; protocol_fee = opt "1/6"; price0_cumulative_last = "' '"; } }, )'
test "tokens_balance_of user default" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\";                                    subaccount=null})" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 2_000_000_000_000_001_118 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 600_000_000_000 : nat;}' 'record { principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; 447_213_595_499_957 : nat;}'
test "tokens_balance_of pool" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$core\"; subaccount=opt blob \"$token_ckETH_token_ckUSDT_subaccount\" })" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 999_999_999_999_998_882 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 200_000_000_000 : nat;}' 'record { principal "vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4"; 0 : nat;}'

blue "\n🚩 2.3 business pair swap extra tokens for tokens"
test "pair_swap_exact_tokens_for_tokens" "$(dfx --identity default canister call core pair_swap_exact_tokens_for_tokens "( record {from=record{owner=principal \"$DEFAULT\"}; amount_in=100_000_000:nat; amount_out_min=100_000_000_000_000:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$DEFAULT\"}; deadline=null} , null)" 2>&1)" '( variant { Err = variant { TokenPairAmmNotExist = record { record { token0 = principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; token1 = principal "cngnf-vqaaa-aaaar-qag4q-cai"; }; "swap_v2_0.3%"; } } }, )'
test "pair_create" "$(dfx --identity default canister call core pair_create "(record { pair=record{ principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\" })" 2>&1)" '(variant { Ok })'
test "pair_query" "$(dfx --identity alice canister call core pair_query "(record { token0 = principal \"$token_ICP\"; token1 = principal \"$token_ckUSDT\"; amm = \"swap_v2_0.3%\"; })" >&1)" '( opt variant { SwapV2 = record { lp = variant { InnerLP = record { fee = 10_000 : nat; decimals = 7 : nat8; dummy_canister_id = principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; minimum_liquidity = 10_000_000 : nat; total_supply = 0 : nat; } }; block_timestamp_last = 0 : nat64; price_cumulative_unit = 18_446_744_073_709_551_615 : nat; reserve0 = 0 : nat; reserve1 = 0 : nat; subaccount = "81c09f0abbdbab8ad406107db3d18588b667eb94f3be6a556ce36b8875cb8996"; price1_cumulative_last = 0 : nat; token0 = "ryjl3-tyaaa-aaaaa-aaaba-cai"; token1 = "cngnf-vqaaa-aaaar-qag4q-cai"; fee_rate = "3/1000"; k_last = 0 : nat; protocol_fee = opt "1/6"; price0_cumulative_last = 0 : nat; } }, )'
token_ICP_token_ckUSDT_subaccount="\81\c0\9f\0a\bb\db\ab\8a\d4\06\10\7d\b3\d1\85\88\b6\67\eb\94\f3\be\6a\55\6c\e3\6b\88\75\cb\89\96"
test "pair_swap_exact_tokens_for_tokens" "$(dfx --identity default canister call core pair_swap_exact_tokens_for_tokens "( record {from=record{owner=principal \"$DEFAULT\"}; amount_in=100_000_000:nat; amount_out_min=100_000_000_000_000:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$DEFAULT\"}; deadline=null} , null)" 2>&1)" '( variant { Err = variant { InsufficientBalance = record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 0 : nat; } } }, )'
test "icrc2_approve token_ICP/default" "$(dfx --identity default canister call token_ICP icrc2_approve "(record { spender=record{owner=principal \"$core\";}; amount=100_000_000_000:nat })" 2>&1)" '(variant { Ok = 1 : nat })'
test "token_deposit" "$(dfx --identity default canister call core token_deposit "(record { token=principal \"$token_ICP\"; from=record{owner=principal \"$DEFAULT\"; subaccount=null}; deposit_amount_without_fee=20_000_000_000: nat; to=record{owner=principal \"$DEFAULT\"; subaccount=null}; }, null)" 2>&1)" '(variant { Ok = 2 : nat })'
test "pair_swap_exact_tokens_for_tokens" "$(dfx --identity default canister call core pair_swap_exact_tokens_for_tokens "( record {from=record{owner=principal \"$DEFAULT\"}; amount_in=100_000_000:nat; amount_out_min=100_000_000_000_000:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$DEFAULT\"}; deadline=null} , null)" 2>&1)" '(variant { Err = variant { Swap = "INSUFFICIENT_LIQUIDITY" } })'
test "tokens_balance_of user default" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\";                                  subaccount=null})" 2>&1)" 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 20_000_000_000 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 600_000_000_000 : nat;}' 'record { principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; 0 : nat;}'
test "tokens_balance_of pool" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$core\"; subaccount=opt blob \"$token_ICP_token_ckUSDT_subaccount\" })" 2>&1)" 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 0 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 0 : nat;}' 'record { principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; 0 : nat;}'
test "pair_liquidity_add" "$(dfx --identity default canister call core pair_liquidity_add "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { pair=record{ principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\" }; amount_desired=record{10_000_000_000:nat;200_000_000_000:nat}; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '( variant { Ok = record { liquidity = 44_721_359_549 : nat; amount = record { 10_000_000_000 : nat; 200_000_000_000 : nat }; } }, )'
test "tokens_balance_of user default" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\";                                   subaccount=null})" 2>&1)" 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 10_000_000_000 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 400_000_000_000 : nat;}' 'record { principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; 44_721_359_549 : nat;}'
test "tokens_balance_of pool" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$core\"; subaccount=opt blob \"$token_ICP_token_ckUSDT_subaccount\" })" 2>&1)" 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 10_000_000_000 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 200_000_000_000 : nat;}' 'record { principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; 0 : nat;}'
test "pair_swap_exact_tokens_for_tokens" "$(dfx --identity default canister call core pair_swap_exact_tokens_for_tokens "( record {from=record{owner=principal \"$DEFAULT\"}; amount_in=100_000_000:nat; amount_out_min=100_000_000_000_000:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$DEFAULT\"}; deadline=null} , null)" 2>&1)" '( variant { Err = variant { Swap = "INSUFFICIENT_OUTPUT_AMOUNT: 1_974_316_068" } }, )'
start_time_s0=$(date +%s)
test "pair_swap_exact_tokens_for_tokens" "$(dfx --identity default canister call core pair_swap_exact_tokens_for_tokens "( record {from=record{owner=principal \"$DEFAULT\"}; amount_in=100_000_000:nat; amount_out_min=1_000_000_000:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$DEFAULT\"}; deadline=null} , null)" 2>&1)" '( variant { Ok = record { amounts = vec { 100_000_000 : nat; 1_974_316_068 : nat } } }, )'
end_time_s0=$(date +%s)
spend=$(($end_time_s0 - $start_time_s0))
echo "pair_swap_exact_tokens_for_tokens Total: $spend seconds"
test "tokens_balance_of user default" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\";                                  subaccount=null})" 2>&1)" 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 9_900_000_000 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 401_974_316_068 : nat;}' 'record { principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; 44_721_359_549 : nat;}'
test "tokens_balance_of pool" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$core\"; subaccount=opt blob \"$token_ICP_token_ckUSDT_subaccount\" })" 2>&1)" 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 10_100_000_000 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 198_025_683_932 : nat;}' 'record { principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; 0 : nat;}'
test "pair_query" "$(dfx --identity alice canister call core pair_query "(record { token0 = principal \"$token_ICP\"; token1 = principal \"$token_ckUSDT\"; amm = \"swap_v2_0.3%\"; })" >&1)" '( opt variant { SwapV2 = record { lp = variant { InnerLP = record { fee = 10_000 : nat; decimals = 7 : nat8; dummy_canister_id = principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; minimum_liquidity = 10_000_000 : nat; total_supply = 44_721_359_549 : nat; } }; block_timestamp_last = ' ' : nat64; price_cumulative_unit = 18_446_744_073_709_551_615 : nat; reserve0 = 10_100_000_000 : nat; reserve1 = 198_025_683_932 : nat; subaccount = "81c09f0abbdbab8ad406107db3d18588b667eb94f3be6a556ce36b8875cb8996"; price1_cumulative_last = ' ' : nat; token0 = "ryjl3-tyaaa-aaaaa-aaaba-cai"; token1 = "cngnf-vqaaa-aaaar-qag4q-cai"; fee_rate = "3/1000"; k_last = 0 : nat; protocol_fee = opt "1/6"; price0_cumulative_last = ' ' : nat; } }, )'

blue "\n🚩 2.4 business pair swap tokens for extra tokens"
test "pair_swap_tokens_for_exact_tokens" "$(dfx --identity default canister call core pair_swap_tokens_for_exact_tokens "( record {from=record{owner=principal \"$DEFAULT\"}; amount_out=25_683_932:nat; amount_in_max=1_000:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$DEFAULT\"}; deadline=null} , null)" 2>&1)" '(variant { Err = variant { Swap = "EXCESSIVE_INPUT_AMOUNT: 1_314_083" } })'
test "pair_swap_tokens_for_exact_tokens" "$(dfx --identity default canister call core pair_swap_tokens_for_exact_tokens "( record {from=record{owner=principal \"$DEFAULT\"}; amount_out=25_683_932:nat; amount_in_max=1_314_082:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$DEFAULT\"}; deadline=null} , null)" 2>&1)" '(variant { Err = variant { Swap = "EXCESSIVE_INPUT_AMOUNT: 1_314_083" } })'
start_time_s0=$(date +%s)
test "pair_swap_tokens_for_exact_tokens" "$(dfx --identity default canister call core pair_swap_tokens_for_exact_tokens "( record {from=record{owner=principal \"$DEFAULT\"}; amount_out=25_683_932:nat; amount_in_max=1_314_083:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$DEFAULT\"}; deadline=null} , null)" 2>&1)" '( variant { Ok = record { amounts = vec { 1_314_083 : nat; 25_683_932 : nat } } }, )'
end_time_s0=$(date +%s)
spend=$(($end_time_s0 - $start_time_s0))
echo "pair_swap_tokens_for_exact_tokens Total: $spend seconds"
test "tokens_balance_of user default" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\";                                  subaccount=null})" 2>&1)" 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 9_898_685_917 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 402_000_000_000 : nat;}' 'record { principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; 44_721_359_549 : nat;}'
test "tokens_balance_of pool" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$core\"; subaccount=opt blob \"$token_ICP_token_ckUSDT_subaccount\" })" 2>&1)" 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 10_101_314_083 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 198_000_000_000 : nat;}' 'record { principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; 0 : nat;}'
test "pair_query" "$(dfx --identity alice canister call core pair_query "(record { token0 = principal \"$token_ICP\"; token1 = principal \"$token_ckUSDT\"; amm = \"swap_v2_0.3%\"; })" >&1)" '( opt variant { SwapV2 = record { lp = variant { InnerLP = record { fee = 10_000 : nat; decimals = 7 : nat8; dummy_canister_id = principal "kvdti-w4byc-pqvo6-3vofn-ibqqp-wz5db-miwzt-6xfht-xzvfk-3hdno-ehk"; minimum_liquidity = 10_000_000 : nat; total_supply = 44_721_359_549 : nat; } }; block_timestamp_last = ' ' : nat64; price_cumulative_unit = 18_446_744_073_709_551_615 : nat; reserve0 = 10_101_314_083 : nat; reserve1 = 198_000_000_000 : nat; subaccount = "81c09f0abbdbab8ad406107db3d18588b667eb94f3be6a556ce36b8875cb8996"; price1_cumulative_last = ' ' : nat; token0 = "ryjl3-tyaaa-aaaaa-aaaba-cai"; token1 = "cngnf-vqaaa-aaaar-qag4q-cai"; fee_rate = "3/1000"; k_last = 0 : nat; protocol_fee = opt "1/6"; price0_cumulative_last = ' ' : nat; } }, )'

blue "\n🚩 2.5 business pair swap by loan"
test "pairs_query" "$(dfx --identity alice canister call core pairs_query >&1)" 'reserve0 = 10_101_314_083 : nat; reserve1 = 198_000_000_000 : nat; subaccount = "81c09f0abbdbab8ad406107db3d18588b667eb94f3be6a556ce36b8875cb8996"' 'reserve0 = 999_999_999_999_998_882 : nat; reserve1 = 200_000_000_000 : nat; subaccount = "11dffa35fd42810f9b249c39749d4adc73a397f799f90703bf6d8fcc1ef7d92c"'
# ICP -> USDT 10_101_314_083 -> 198_000_000_000
# USDT -> ETH 200_000_000_000 -> 999_999_999_999_998_882
test "pair_create" "$(dfx --identity default canister call core pair_create "(record { pair=record{ principal \"$token_ICP\"; principal \"$token_ckETH\"}; amm=\"swap_v2_0.3%\" })" 2>&1)" '(variant { Ok })'
test "tokens_balance_of user default" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 2_000_000_000_000_001_118 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 402_000_000_000 : nat;}' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 9_898_685_917 : nat;}'
test "pair_liquidity_add" "$(dfx --identity default canister call core pair_liquidity_add "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { pair=record{ principal \"$token_ICP\"; principal \"$token_ckETH\"}; amm=\"swap_v2_0.3%\" }; amount_desired=record{2_000_000_000:nat;100_000_000_000_000_000:nat}; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '( variant { Ok = record { liquidity = 14_142_135_623_730 : nat; amount = record { 2_000_000_000 : nat; 100_000_000_000_000_000 : nat }; } }, )'
# ETH -> ICP 100_000_000_000_000_000 -> 20_000_000_000
test "tokens_balance_of user default" "$(dfx --identity default canister call core tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 1_900_000_000_000_001_118 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 402_000_000_000 : nat;}' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 7_898_685_917 : nat;}'
test "tokens_balance_of user bob    " "$(dfx --identity bob canister call core tokens_balance_of "(record { owner=principal \"$BOB\";         subaccount=null})" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 0 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 0 : nat;}' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 0 : nat;}'
test "pair_swap_by_loan" "$(dfx --identity default canister call core pair_swap_by_loan "( record {from=record{owner=principal \"$DEFAULT\"}; loan=2_000_000_000:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"}; record {pair=record{principal \"$token_ckUSDT\"; principal \"$token_ckETH\"}; amm=\"swap_v2_0.3%\"}; record {pair=record{principal \"$token_ckETH\"; principal \"$token_ICP\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$BOB\"}; deadline=null} , null)" 2>&1)" '( variant { Err = variant { Swap = "INSUFFICIENT_OUTPUT_AMOUNT: 1_165_021_595" } }, )'
start_time_s0=$(date +%s)
test "pair_swap_by_loan" "$(dfx --identity default canister call core pair_swap_by_loan "( record {from=record{owner=principal \"$DEFAULT\"}; loan=20_000:nat; path=vec { record {pair=record{principal \"$token_ICP\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"}; record {pair=record{principal \"$token_ckUSDT\"; principal \"$token_ckETH\"}; amm=\"swap_v2_0.3%\"}; record {pair=record{principal \"$token_ckETH\"; principal \"$token_ICP\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$BOB\"}; deadline=null} , null)" 2>&1)" '( variant { Ok = record { amounts = vec { 20_000 : nat; 390_851 : nat; 1_948_388_438_775 : nat; 38_850 : nat; }; } }, )'
end_time_s0=$(date +%s)
spend=$(($end_time_s0 - $start_time_s0))
echo "pair_swap_by_loan Total: $spend seconds"
test "tokens_balance_of user bob    " "$(dfx --identity bob canister call core tokens_balance_of "(record { owner=principal \"$BOB\";         subaccount=null})" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 0 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 0 : nat;}' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 18_850 : nat;}'

blue "\n🚩 3 business config fee to"
blue "\n🚩 3.1 business config fee to update"
test "🙈 config_fee_to_query" "$(dfx --identity bob canister call core config_fee_to_query 2>&1)" "Permission 'BusinessConfigFeeTo' is required"
test "config_fee_to_query" "$(dfx --identity default canister call core config_fee_to_query 2>&1)" '(null)'
test "❎ config_fee_to_replace" "$(dfx --identity bob canister call core config_fee_to_replace "(opt record {owner=principal \"$BOB\"})" 2>&1)" "Permission 'BusinessConfigFeeTo' is required"
test "config_fee_to_replace" "$(dfx --identity default canister call core config_fee_to_replace "(opt record {owner=principal \"$BOB\"})" 2>&1)" '(null)'
test "config_fee_to_query" "$(dfx --identity default canister call core config_fee_to_query 2>&1)" "( opt record { owner = principal \"$BOB\"; subaccount = null; }, )"

blue "\n🚩 3.2 business config fee to effect"
test "token_balance_of user default" "$(dfx --identity default canister call core token_balance_of "(principal \"vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4\", record { owner=principal \"$DEFAULT\"})" 2>&1)" '(447_213_595_499_957 : nat)'
test "token_balance_of user bob" "$(dfx --identity bob canister call core token_balance_of "(principal \"vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4\", record { owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
# liquidity changed after set fee_to
test "pair_liquidity_remove" "$(dfx --identity default canister call core pair_liquidity_remove "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { token = record { principal \"$token_ckETH\"; principal \"$token_ckUSDT\" }; amm=\"swap_v2_0.3%\"; }; liquidity=1_000_000:nat; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '(variant { Ok = record { amount = record { 2_236_063_620 : nat; 447 : nat } } })'
test "token_balance_of user default" "$(dfx --identity default canister call core token_balance_of "(principal \"vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4\", record { owner=principal \"$DEFAULT\"})" 2>&1)" '(447_213_594_499_957 : nat)'
test "token_balance_of user bob" "$(dfx --identity bob canister call core token_balance_of "(principal \"vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4\", record { owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
# do swap
test "pair_swap_exact_tokens_for_tokens" "$(dfx --identity default canister call core pair_swap_exact_tokens_for_tokens "( record {from=record{owner=principal \"$DEFAULT\"}; amount_in=100_000_000_000:nat; amount_out_min=1:nat; path=vec { record {pair=record{principal \"$token_ckETH\"; principal \"$token_ckUSDT\"}; amm=\"swap_v2_0.3%\"} }; to=record{owner=principal \"$DEFAULT\"}; deadline=null} , null)" 2>&1)" '( variant { Ok = record { amounts = vec { 100_000_000_000 : nat; 19_940 : nat } } }, )'
test "token_balance_of user default" "$(dfx --identity default canister call core token_balance_of "(principal \"vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4\", record { owner=principal \"$DEFAULT\"})" 2>&1)" '(447_213_594_499_957 : nat)'
test "token_balance_of user bob" "$(dfx --identity bob canister call core token_balance_of "(principal \"vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4\", record { owner=principal \"$BOB\"})" 2>&1)" '(0 : nat)'
# liquidity changed after swap
test "pair_liquidity_remove" "$(dfx --identity default canister call core pair_liquidity_remove "(record { from=record{owner=principal\"$DEFAULT\"}; swap_pair=record { token = record { principal \"$token_ckETH\"; principal \"$token_ckUSDT\" }; amm=\"swap_v2_0.3%\"; }; liquidity=1_000_000:nat; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$DEFAULT\"}; deadline=null } , null)" 2>&1)" '(variant { Ok = record { amount = record { 2_236_063_844 : nat; 447 : nat } } })'
test "token_balance_of user default" "$(dfx --identity default canister call core token_balance_of "(principal \"vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4\", record { owner=principal \"$DEFAULT\"})" 2>&1)" '(447_213_593_499_957 : nat)'
test "token_balance_of user bob" "$(dfx --identity bob canister call core token_balance_of "(principal \"vbofh-iir37-5dl7k-cqehz-wje4h-f2j2s-w4oor-zp54z-7edqh-p3nr7-gb4\", record { owner=principal \"$BOB\"})" 2>&1)" '(11_194 : nat)'
test "tokens_balance_of user bob    " "$(dfx --identity bob canister call core tokens_balance_of "(record { owner=principal \"$BOB\";         subaccount=null})" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 0 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 0 : nat;}' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 18_850 : nat;}'
test "pair_liquidity_remove" "$(dfx --identity bob canister call core pair_liquidity_remove "(record { from=record{owner=principal\"$BOB\"}; swap_pair=record { token = record { principal \"$token_ckETH\"; principal \"$token_ckUSDT\" }; amm=\"swap_v2_0.3%\"; }; liquidity=11_194:nat; amount_min=record{1:nat;1:nat}; to=record{owner=principal\"$BOB\"}; deadline=null } , null)" 2>&1)" '(variant { Ok = record { amount = record { 25_030_498 : nat; 5 : nat } } }'
test "tokens_balance_of user bob    " "$(dfx --identity bob canister call core tokens_balance_of "(record { owner=principal \"$BOB\";         subaccount=null})" 2>&1)" 'record { principal "ss2fx-dyaaa-aaaar-qacoq-cai"; 25_030_498 : nat;}' 'record { principal "cngnf-vqaaa-aaaar-qag4q-cai"; 5 : nat;}' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 18_850 : nat;}'

blue "\n🚩 1.1 permission permission_query"
test "version" "$(dfx --identity alice canister call core version 2>&1)" '(1 : nat32)'
test "permission_all" "$(dfx --identity alice canister call core permission_all 2>&1)" 'vec { variant { Forbidden = "PauseQuery" }; variant { Permitted = "PauseReplace" }'
test "permission_query" "$(dfx --identity alice canister call core permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_query" "$(dfx canister call core permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessConfigFeeTo"; "BusinessTokenBalanceBy"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_update" "$(dfx --identity bob canister call core permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "'PermissionUpdate' is required"
test "permission_update" "$(dfx canister call core permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "()"
test "🙈 permission_query" "$(dfx --identity alice canister call core permission_query 2>&1)" "'PermissionQuery' is required"
test "permission_query" "$(dfx canister call core permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessConfigFeeTo"; "BusinessTokenBalanceBy"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_find_by_user" "$(dfx canister call core permission_find_by_user "(principal \"$ALICE\")" 2>&1)" '( vec { "PauseQuery"; "PermissionUpdate"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_update" "$(dfx --identity alice canister call core permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call core permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_query" "$(dfx canister call core permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessConfigFeeTo"; "BusinessTokenBalanceBy"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'

blue "\n🚩 1.2 permission permission update"
test "permission_query" "$(dfx canister call core permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessConfigFeeTo"; "BusinessTokenBalanceBy"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_query" "$(dfx --identity alice canister call core permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_find_by_user" "$(dfx canister call core permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessConfigFeeTo"; "BusinessTokenBalanceBy"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairCreate"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_find_by_user" "$(dfx canister call core permission_find_by_user "(principal \"$ALICE\")" 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "🙈 permission_find_by_user" "$(dfx --identity alice canister call core permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" "'PermissionFind' is required"
test "🙈 permission_find_by_user" "$(dfx --identity alice canister call core permission_find_by_user "(principal \"$ALICE\")" 2>&1)" "'PermissionFind' is required"

blue "\n🚩 1.3 permission roles"
test "permission_query" "$(dfx --identity alice canister call core permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_update" "$(dfx canister call core permission_update "(vec { variant { UpdateRolePermission=record{\"Admin\"; opt vec {\"PauseReplace\"; \"PauseQuery\"} } } })" 2>&1)" "()"
test "permission_update" "$(dfx canister call core permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; opt vec {\"Admin\"} } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call core permission_query 2>&1)" '( vec { "PauseReplace"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'
test "permission_update" "$(dfx canister call core permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call core permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessTokenTransfer"; "BusinessTokenPairLiquidityAdd"; "BusinessTokenPairLiquidityRemove"; "BusinessTokenPairSwap"; "BusinessExampleQuery"; }, )'

blue "\n🚩 2.1 pause permission"
test "pause_query" "$(dfx canister call core pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call core pause_query_reason 2>&1)" "(null)"
test "pause_replace" "$(dfx canister call core pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call core pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx canister call core pause_query_reason 2>&1)" "message = \"reason\""

blue "\n🚩 2.2 pause permission by alice"
test "pause_query" "$(dfx --identity alice canister call core pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx --identity alice canister call core pause_query_reason 2>&1)" "message = \"reason\""

blue "\n🚩 2.3 pause no permission"
test "🙈 pause_replace" "$(dfx --identity alice canister call core pause_replace "(null)" 2>&1)" "'PauseReplace' is required"
test "permission_update" "$(dfx canister call core permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PauseReplace\";\"PauseQuery\" } } } })" 2>&1)" "()"
test "pause_replace" "$(dfx --identity alice canister call core pause_replace "(null)" 2>&1)" "()"
test "🙈 pause_query" "$(dfx --identity alice canister call core pause_query 2>&1)" "'PauseQuery' is required"
test "🙈 pause_query_reason" "$(dfx --identity alice canister call core pause_query_reason 2>&1)" "'PauseQuery' is required"
test "pause_query" "$(dfx canister call core pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call core pause_query_reason 2>&1)" "(null)"

blue "\n🚩 3 record no permission"
test "🙈 record_topics" "$(dfx --identity alice canister call core record_topics 2>&1)" "'RecordFind' is required"
test "record_topics" "$(dfx canister call core record_topics 2>&1)" '"Example"' '"CyclesCharge"'
test "record_find_by_page" "$(dfx canister call core record_find_by_page "(record{page=1:nat64;size=1:nat32},opt record{topic=opt vec{\"Pause\"}})" 2>&1)" "record { total = "
test "record_migrate" "$(dfx canister call core record_migrate "(1:nat32)" 2>&1)" "removed = 0"

blue "\n🚩 4 schedule"
test "🙈 schedule_find" "$(dfx --identity alice canister call core schedule_find 2>&1)" "'ScheduleFind' is required"
test "schedule_find" "$(dfx canister call core schedule_find 2>&1)" "(null)"
test "🙈 schedule_replace" "$(dfx --identity alice canister call core schedule_replace "(opt (1000000000:nat64))" 2>&1)" "'ScheduleReplace' is required"
test "schedule_replace" "$(dfx canister call core schedule_replace "(opt (1000000000:nat64))" 2>&1)" "()"
sleep 3
test "schedule_replace" "$(dfx canister call core schedule_replace "(null)" 2>&1)" "()"
sleep 2
test "🙈 schedule_trigger" "$(dfx --identity alice canister call core schedule_trigger 2>&1)" "'ScheduleTrigger' is required"
test "schedule_trigger" "$(dfx canister call core schedule_trigger 2>&1)" "()"

blue "\n🚩 5 example business"
test "business_example_query" "$(dfx --identity alice canister call core business_example_query 2>&1)" "\"\""
test "business_example_query" "$(dfx canister call core business_example_query 2>&1)" "\"\""
test "🙈 business_example_set" "$(dfx --identity alice canister call core business_example_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_set" "$(dfx canister call core business_example_set "(\"test string\")" 2>&1)" "()"
test "business_example_query" "$(dfx --identity alice canister call core business_example_query 2>&1)" "test string"
test "business_example_query" "$(dfx canister call core business_example_query 2>&1)" "test string"

blue "\n🚩 6 test core data"
test "pause_replace" "$(dfx canister call core pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call core pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" core
test "pause_replace" "$(dfx canister call core pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call core pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call core business_example_query 2>&1)" "test string"

blue "\n🚩 7 test core cell"
test "business_example_cell_query" "$(dfx --identity alice canister call core business_example_cell_query 2>&1)" "\"\""
test "business_example_cell_query" "$(dfx canister call core business_example_cell_query 2>&1)" "\"\""
test "🙈 business_example_cell_set" "$(dfx --identity alice canister call core business_example_cell_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_cell_set" "$(dfx canister call core business_example_cell_set "(\"test string\")" 2>&1)" "()"
test "business_example_cell_query" "$(dfx --identity alice canister call core business_example_cell_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx canister call core business_example_cell_query 2>&1)" "test string"

blue "\n🚩 8 test core vec"
test "business_example_vec_query" "$(dfx --identity alice canister call core business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call core business_example_vec_query 2>&1)" "(vec {})"
test "🙈 business_example_vec_pop" "$(dfx --identity alice canister call core business_example_vec_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_pop" "$(dfx canister call core business_example_vec_pop "()" 2>&1)" "(null)"
test "business_example_vec_push" "$(dfx --identity alice canister call core business_example_vec_push "(5: nat64)" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_push" "$(dfx canister call core business_example_vec_push "(5: nat64)" 2>&1)" "()"
test "business_example_vec_query" "$(dfx --identity alice canister call core business_example_vec_query 2>&1)" "(vec { record { vec_data = 5 : nat64 } })"
test "business_example_vec_query" "$(dfx canister call core business_example_vec_query 2>&1)" "(vec { record { vec_data = 5 : nat64 } })"
test "🙈 business_example_vec_pop" "$(dfx --identity alice canister call core business_example_vec_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_pop" "$(dfx canister call core business_example_vec_pop "()" 2>&1)" "(opt record { vec_data = 5 : nat64 })"
test "business_example_vec_query" "$(dfx --identity alice canister call core business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call core business_example_vec_query 2>&1)" "(vec {})"

blue "\n🚩 9 test core map"
test "business_example_map_query" "$(dfx --identity alice canister call core business_example_map_query 2>&1)" "(vec {})"
test "business_example_map_query" "$(dfx canister call core business_example_map_query 2>&1)" "(vec {})"
test "business_example_map_update" "$(dfx --identity alice canister call core business_example_map_update "(1:nat64, opt \"111\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_map_update" "$(dfx canister call core business_example_map_update "(1:nat64, opt \"111\")" 2>&1)" "(null)"
test "business_example_map_query" "$(dfx --identity alice canister call core business_example_map_query 2>&1)" '(vec { record { 1 : nat64; "111" } })'
test "business_example_map_query" "$(dfx canister call core business_example_map_query 2>&1)" '(vec { record { 1 : nat64; "111" } })'
test "business_example_map_update" "$(dfx canister call core business_example_map_update "(1:nat64, opt \"123\")" 2>&1)" "(opt \"111\")"
test "business_example_map_update" "$(dfx canister call core business_example_map_update "(1:nat64, null)" 2>&1)" "(opt \"123\")"
test "business_example_map_update" "$(dfx canister call core business_example_map_update "(2:nat64, opt \"222\")" 2>&1)" "(null)"
test "business_example_map_query" "$(dfx --identity alice canister call core business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_map_query" "$(dfx canister call core business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'

blue "\n🚩 10 test core log"
test "business_example_log_query" "$(dfx --identity alice canister call core business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_query" "$(dfx canister call core business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_update" "$(dfx --identity alice canister call core business_example_log_update "(\"111\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_log_update" "$(dfx canister call core business_example_log_update "(\"111\")" 2>&1)" "(0 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call core business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_query" "$(dfx canister call core business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_update" "$(dfx canister call core business_example_log_update "(\"123\")" 2>&1)" "(1 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call core business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_log_query" "$(dfx canister call core business_example_log_query 2>&1)" '(vec { "111"; "123" })'

blue "\n🚩 11 test core priority queue"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call core business_example_priority_queue_query 2>&1)" "(vec {})"
test "business_example_priority_queue_query" "$(dfx canister call core business_example_priority_queue_query 2>&1)" "(vec {})"
test "business_example_priority_queue_pop" "$(dfx --identity alice canister call core business_example_priority_queue_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_pop" "$(dfx canister call core business_example_priority_queue_pop "()" 2>&1)" "(null)"
test "business_example_priority_queue_push" "$(dfx --identity alice canister call core business_example_priority_queue_push "(5: nat64)" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_push" "$(dfx canister call core business_example_priority_queue_push "(5: nat64)" 2>&1)" "()"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call core business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call core business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_push" "$(dfx canister call core business_example_priority_queue_push "(2: nat64)" 2>&1)" "()"
test "business_example_priority_queue_query" "$(dfx canister call core business_example_priority_queue_query 2>&1)" "(vec { 2 : nat64; 5 : nat64 })"
test "business_example_priority_queue_pop" "$(dfx --identity alice canister call core business_example_priority_queue_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_pop" "$(dfx canister call core business_example_priority_queue_pop "()" 2>&1)" "(opt (2 : nat64))"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call core business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call core business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"

blue "\n🚩 12 test core priority queue"
test "pause_replace" "$(dfx canister call core pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call core pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" core
test "pause_replace" "$(dfx canister call core pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call core pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call core business_example_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx --identity alice canister call core business_example_cell_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx canister call core business_example_cell_query 2>&1)" "test string"
test "business_example_vec_query" "$(dfx --identity alice canister call core business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call core business_example_vec_query 2>&1)" "(vec {})"
test "business_example_map_query" "$(dfx --identity alice canister call core business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_map_query" "$(dfx canister call core business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_log_query" "$(dfx --identity alice canister call core business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_log_query" "$(dfx canister call core business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_priority_queue_query" "$(dfx --identity alice canister call core business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call core business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"

# test completed

green "\n=================== TEST COMPLETED AND SUCCESSFUL ===================\n"

end_time=$(date +%H:%M:%S)
end_time_s=$(date +%s)
spend=$(($end_time_s - $start_time_s))
spend_minutes=$(($spend / 60))
echo "✅ $start_time -> $end_time" "Total: $spend seconds ($spend_minutes mins) 🎉🎉🎉\n"

say test successful

# sleep 10000
# read -s -n1 -p "按任意键结束..."
