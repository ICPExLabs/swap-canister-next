#!/usr/bin/env bash
start_time=$(date +%H:%M:%S)
start_time_s=$(date +%s)

# Automatically stop after operation
dfx stop
trap 'say test over && dfx stop' EXIT

# dfx start --background --clean                      # Open a new dfx environment
dfx start --artificial-delay 0 --background --clean # Open a new dfx environment
# dfx start --background --clean >/dev/null 2>&1 # Open a new dfx environment

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

# ! 1. Test archive_swap
red "\n=========== 1. archive_swap ===========\n"
dfx canister create archive_swap --specified-id "bkyz2-fmaaa-aaaaa-qaaaq-cai" # --with-cycles 50T
dfx deploy --mode=reinstall --yes --argument "(null)" archive_swap
archive_swap=$(canister_id "archive_swap")
blue "Archive Token Canister: $archive_swap_token"

if [ -z "$archive_swap" ]; then
    say deploy failed
    exit 1
fi

blue "\n🚩 1 business"
test "🙈 get_block_pb" "$(dfx --identity alice canister call archive_swap get_block_pb "(blob \"\")" 2>&1)" '(blob "")'
test "get_block_pb" "$(dfx canister call archive_swap get_block_pb "(blob \"\")" 2>&1)" '(blob "")'
test "remaining_capacity" "$(dfx --identity alice canister call archive_swap remaining_capacity 2>&1)" '(10_737_418_240 : nat64)'
test "❎ append_blocks" "$(dfx --identity alice canister call archive_swap append_blocks "(vec { vec { 0:nat8 } })" 2>&1)" 'Only Core Canister is allowed to append blocks to an Archive Node'
test "append_blocks" "$(dfx canister call archive_swap append_blocks "(vec { blob \"\0a\22\0a\20\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\1a\1b\0a\19\0a\17\0a\0c\0a\0a\00\00\00\00\00\00\00\02\01\01\12\00\1a\03\0a\01\64\22\00\" })" 2>&1)" '()'
test "get_block_pb" "$(dfx canister call archive_swap get_block_pb "(blob \"\")" --output json 2>&1)" '[ 10, 67, 10, 65, 10, 34, 10, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 10, 25, 10, 23, 10, 12, 10, 10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 18, 0, 26, 3, 10, 1, 100, 34, 0 ]'
test "remaining_capacity" "$(dfx --identity alice canister call archive_swap remaining_capacity 2>&1)" '(10_737_418_175 : nat64)'
test "iter_blocks_pb" "$(dfx canister call archive_swap iter_blocks_pb "(blob \"\10\64\")" --output json 2>&1)" '[ 10, 67, 10, 65, 10, 34, 10, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 10, 25, 10, 23, 10, 12, 10, 10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 18, 0, 26, 3, 10, 1, 100, 34, 0 ]'
test "get_blocks_pb" "$(dfx canister call archive_swap get_blocks_pb "(blob \"\10\64\")" --output json 2>&1)" '[ 18, 104, 82, 101, 113, 117, 101, 115, 116, 101, 100, 32, 98, 108, 111, 99, 107, 115, 32, 111, 117, 116, 115, 105, 100, 101, 32, 116, 104, 101, 32, 114, 97, 110, 103, 101, 32, 115, 116, 111, 114, 101, 100, 32, 105, 110, 32, 116, 104, 101, 32, 97, 114, 99, 104, 105, 118, 101, 32, 110, 111, 100, 101, 46, 32, 82, 101, 113, 117, 101, 115, 116, 101, 100, 32, 91, 48, 32, 46, 46, 32, 49, 48, 48, 93, 46, 32, 65, 118, 97, 105, 108, 97, 98, 108, 101, 32, 91, 48, 32, 46, 46, 32, 49, 93, 46 ]'
test "get_blocks" "$(dfx canister call archive_swap get_blocks "(record { start=1:nat64; length=100:nat64})" 2>&1)" '(variant { Ok = record { blocks = vec {} } })'
test "get_blocks" "$(dfx canister call archive_swap get_blocks "(record { start=11:nat64; length=100:nat64})" 2>&1)" '(variant { Ok = record { blocks = vec {} } })'
test "get_blocks" "$(dfx canister call archive_swap get_blocks "(record { start=0:nat64; length=100:nat64})" 2>&1)" '( variant { Ok = record { blocks = vec { record { transaction = record { created = null; memo = null; operation = variant { deposit = record { to = record { owner = principal "aaaaa-aa"; subaccount = null }; token = principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; from = record { owner = principal "aaaaa-aa"; subaccount = null; }; amount = 100 : nat; } }; }; timestamp = 0 : nat64; parent_hash = blob ""; }; }; } }, )'
test "http /metrics" "$(curl "http://$archive_swap.raw.localhost:4943/metrics" 2>&1)" 'archive_node_blocks_bytes 65' 'archive_node_blocks 1' 'archive_node_max_memory_size_bytes 10737418240'
test "get_encoded_blocks" "$(dfx canister call archive_swap get_encoded_blocks "(record { start=0:nat64; length=100:nat64})" --output json 2>&1)" '{ "Ok": [ [ 10, 34, 10, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 10, 25, 10, 23, 10, 12, 10, 10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 18, 0, 26, 3, 10, 1, 100, 34, 0 ] ] }'

blue "\n🚩 2 business set_maintainers"
test "set_maintainers" "$(dfx --identity alice canister call archive_swap set_maintainers "(null)" 2>&1)" 'Only Core Canister is allowed to append blocks to an Archive Node'
test "🙈 get_block_pb" "$(dfx --identity alice canister call archive_swap get_block_pb "(blob \"\")" --output json 2>&1)" '[ 10, 67, 10, 65, 10, 34, 10, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 10, 25, 10, 23, 10, 12, 10, 10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 18, 0, 26, 3, 10, 1, 100, 34, 0 ]'
test "set_maintainers" "$(dfx canister call archive_swap set_maintainers "(opt vec {principal\"$DEFAULT\"})" 2>&1)" '()'
test "🙈 get_block_pb" "$(dfx --identity alice canister call archive_swap get_block_pb "(blob \"\")" 2>&1)" 'Only Maintainers are allowed to query data'
test "set_maintainers" "$(dfx canister call archive_swap set_maintainers "(null)" 2>&1)" '()'
test "🙈 get_block_pb" "$(dfx --identity alice canister call archive_swap get_block_pb "(blob \"\")" --output json 2>&1)" '[ 10, 67, 10, 65, 10, 34, 10, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 10, 25, 10, 23, 10, 12, 10, 10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 18, 0, 26, 3, 10, 1, 100, 34, 0 ]'

blue "\n🚩 3 business query"
test "query_latest_block_index" "$(dfx canister call archive_swap query_latest_block_index 2>&1)" '(opt (0 : nat64))'
test "query_metrics" "$(dfx canister call archive_swap query_metrics 2>&1)" '( record { stable_memory_pages = 257 : nat64; stable_memory_bytes = 16_842_752 : nat64; heap_memory_bytes = 1_245_184 : nat64; last_upgrade_time_seconds = 0 : nat64; max_memory_size_bytes = 10_737_418_240 : nat64; blocks = 1 : nat64; blocks_bytes = 65 : nat64; block_height_offset = 0 : nat64; }, )'

blue "\n🚩 4 business set_max_memory_size_bytes"
test "query_metrics" "$(dfx canister call archive_swap query_metrics 2>&1)" '( record { stable_memory_pages = 257 : nat64; stable_memory_bytes = 16_842_752 : nat64; heap_memory_bytes = 1_245_184 : nat64; last_upgrade_time_seconds = 0 : nat64; max_memory_size_bytes = 10_737_418_240 : nat64; blocks = 1 : nat64; blocks_bytes = 65 : nat64; block_height_offset = 0 : nat64; }, )'
test "set_max_memory_size_bytes" "$(dfx --identity alice canister call archive_swap set_max_memory_size_bytes "(10:nat64)" 2>&1)" 'Only Core Canister is allowed to append blocks to an Archive Node'
test "set_max_memory_size_bytes" "$(dfx --identity default canister call archive_swap set_max_memory_size_bytes "(10:nat64)" 2>&1)" 'Cannot set max_memory_size_bytes to 10, because it is lower than total_block_size 65.'
test "set_max_memory_size_bytes" "$(dfx --identity default canister call archive_swap set_max_memory_size_bytes "(200:nat64)" 2>&1)" '()'
test "query_metrics" "$(dfx canister call archive_swap query_metrics 2>&1)" '( record { stable_memory_pages = 257 : nat64; stable_memory_bytes = 16_842_752 : nat64; heap_memory_bytes = 1_245_184 : nat64; last_upgrade_time_seconds = 0 : nat64; max_memory_size_bytes = 200 : nat64; blocks = 1 : nat64; blocks_bytes = 65 : nat64; block_height_offset = 0 : nat64; }, )'

# test completed

green "\n=================== TEST COMPLETED AND SUCCESSFUL ===================\n"

end_time=$(date +%H:%M:%S)
end_time_s=$(date +%s)
spend=$(($end_time_s - $start_time_s))
spend_minutes=$(($spend / 60))
echo "✅ $start_time -> $end_time" "Total: $spend seconds ($spend_minutes mins) 🎉🎉🎉\n"

say test successful

# sleep 10000
# read -s -n1 -p "Press any key to end..."
