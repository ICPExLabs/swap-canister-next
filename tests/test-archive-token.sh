#!/usr/bin/env bash
start_time=$(date +%H:%M:%S)
start_time_s=$(date +%s)

# 运行完毕自动停止
dfx stop
trap 'say test over && dfx stop' EXIT

# dfx start --background --clean # 开启新的 dfx 环境
dfx start --artificial-delay 0 --background --clean # 开启新的 dfx 环境
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

# ! 1. 测试 archive_token
red "\n=========== 1. archive_token ===========\n"
dfx canister create archive_token --specified-id "bkyz2-fmaaa-aaaaa-qaaaq-cai" # --with-cycles 50T
dfx deploy --mode=reinstall --yes --argument "(opt variant { V1=record {maintainers=opt vec { principal \"$DEFAULT\" }}})" archive_token
archive_token=$(canister_id "archive_token")
blue "Archive Token Canister: $archive_token_token"

if [ -z "$archive_token" ]; then
    say deploy failed
    exit 1
fi

blue "\n🚩 1 business"
test "🙈 get_block_pb" "$(dfx --identity alice canister call archive_token get_block_pb "(blob \"\")" 2>&1)" 'Only Maintainers are allowed to query data'
test "get_block_pb" "$(dfx canister call archive_token get_block_pb "(blob \"\")" 2>&1)" '(blob "")'
test "remaining_capacity" "$(dfx --identity alice canister call archive_token remaining_capacity 2>&1)" '(10_737_418_240 : nat64)'
test "❎ append_blocks" "$(dfx --identity alice canister call archive_token append_blocks "(vec { vec { 0:nat8 } })" 2>&1)" 'Only Core canister is allowed to append blocks to an Archive Node'
test "append_blocks" "$(dfx canister call archive_token append_blocks "(vec { blob \"\0a\22\0a\20\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\1a\1b\0a\19\0a\17\0a\0c\0a\0a\00\00\00\00\00\00\00\02\01\01\12\00\1a\03\0a\01\64\22\00\" })" 2>&1)" '()'
test "get_block_pb" "$(dfx canister call archive_token get_block_pb "(blob \"\")" --output json 2>&1)" '[ 10, 67, 10, 65, 10, 34, 10, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 10, 25, 10, 23, 10, 12, 10, 10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 18, 0, 26, 3, 10, 1, 100, 34, 0 ]'
test "remaining_capacity" "$(dfx --identity alice canister call archive_token remaining_capacity 2>&1)" '(10_737_418_175 : nat64)'
test "iter_blocks_pb" "$(dfx canister call archive_token iter_blocks_pb "(blob \"\10\64\")" --output json 2>&1)" '[ 10, 67, 10, 65, 10, 34, 10, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 10, 25, 10, 23, 10, 12, 10, 10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 18, 0, 26, 3, 10, 1, 100, 34, 0 ]'
test "get_blocks_pb" "$(dfx canister call archive_token get_blocks_pb "(blob \"\10\64\")" --output json 2>&1)" '[ 18, 104, 82, 101, 113, 117, 101, 115, 116, 101, 100, 32, 98, 108, 111, 99, 107, 115, 32, 111, 117, 116, 115, 105, 100, 101, 32, 116, 104, 101, 32, 114, 97, 110, 103, 101, 32, 115, 116, 111, 114, 101, 100, 32, 105, 110, 32, 116, 104, 101, 32, 97, 114, 99, 104, 105, 118, 101, 32, 110, 111, 100, 101, 46, 32, 82, 101, 113, 117, 101, 115, 116, 101, 100, 32, 91, 48, 32, 46, 46, 32, 49, 48, 48, 93, 46, 32, 65, 118, 97, 105, 108, 97, 98, 108, 101, 32, 91, 48, 32, 46, 46, 32, 49, 93, 46 ]'
test "get_blocks" "$(dfx canister call archive_token get_blocks "(record { start=1:nat64; length=100:nat64})" 2>&1)" '(variant { Ok = record { blocks = vec {} } })'
test "get_blocks" "$(dfx canister call archive_token get_blocks "(record { start=11:nat64; length=100:nat64})" 2>&1)" '(variant { Ok = record { blocks = vec {} } })'
test "get_blocks" "$(dfx canister call archive_token get_blocks "(record { start=0:nat64; length=100:nat64})" 2>&1)" '( variant { Ok = record { blocks = vec { record { transaction = record { created = null; memo = null; operation = variant { deposit = record { to = record { owner = principal "aaaaa-aa"; subaccount = null }; token = principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; from = record { owner = principal "aaaaa-aa"; subaccount = null; }; amount = 100 : nat; } }; }; timestamp = 0 : nat64; parent_hash = blob ""; }; }; } }, )'
test "http /metrics" "$(curl "http://$archive_token.raw.localhost:4943/metrics" 2>&1)" 'archive_node_blocks_bytes 65' 'archive_node_blocks 1' 'archive_node_max_memory_size_bytes 10737418240'
test "get_encoded_blocks" "$(dfx canister call archive_token get_encoded_blocks "(record { start=0:nat64; length=100:nat64})" --output json 2>&1)" '{ "Ok": [ [ 10, 34, 10, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 10, 25, 10, 23, 10, 12, 10, 10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 18, 0, 26, 3, 10, 1, 100, 34, 0 ] ] }'

blue "\n🚩 2 business set_maintainers"
test "set_maintainers" "$(dfx --identity alice canister call archive_token set_maintainers "(null)" 2>&1)" 'Only Core canister is allowed to append blocks to an Archive Node'
test "🙈 get_block_pb" "$(dfx --identity alice canister call archive_token get_block_pb "(blob \"\")" 2>&1)" 'Only Maintainers are allowed to query data'
test "set_maintainers" "$(dfx canister call archive_token set_maintainers "(null)" 2>&1)" '()'
test "🙈 get_block_pb" "$(dfx --identity alice canister call archive_token get_block_pb "(blob \"\")" --output json 2>&1)" '[ 10, 67, 10, 65, 10, 34, 10, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 10, 25, 10, 23, 10, 12, 10, 10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 18, 0, 26, 3, 10, 1, 100, 34, 0 ]'
test "set_maintainers" "$(dfx canister call archive_token set_maintainers "(opt vec {principal\"$DEFAULT\"})" 2>&1)" '()'
test "🙈 get_block_pb" "$(dfx --identity alice canister call archive_token get_block_pb "(blob \"\")" 2>&1)" 'Only Maintainers are allowed to query data'

blue "\n🚩 3 business query"
test "query_latest_block_index" "$(dfx canister call archive_token query_latest_block_index 2>&1)" '(opt (0 : nat64))'
test "query_metrics" "$(dfx canister call archive_token query_metrics 2>&1)" '( record { stable_memory_pages = 1_025 : nat64; stable_memory_bytes = 67_174_400 : nat64; heap_memory_bytes = 1_245_184 : nat64; last_upgrade_time_seconds = 0 : nat64; max_memory_size_bytes = 10_737_418_240 : nat64; blocks = 1 : nat64; blocks_bytes = 65 : nat64; block_height_offset = 0 : nat64; }, )'

blue "\n🚩 4 business set_max_memory_size_bytes"
test "query_metrics" "$(dfx canister call archive_token query_metrics 2>&1)" '( record { stable_memory_pages = 1_025 : nat64; stable_memory_bytes = 67_174_400 : nat64; heap_memory_bytes = 1_245_184 : nat64; last_upgrade_time_seconds = 0 : nat64; max_memory_size_bytes = 10_737_418_240 : nat64; blocks = 1 : nat64; blocks_bytes = 65 : nat64; block_height_offset = 0 : nat64; }, )'
test "set_max_memory_size_bytes" "$(dfx --identity alice canister call archive_token set_max_memory_size_bytes "(10:nat64)" 2>&1)" 'Only Core canister is allowed to append blocks to an Archive Node'
test "set_max_memory_size_bytes" "$(dfx --identity default canister call archive_token set_max_memory_size_bytes "(10:nat64)" 2>&1)" 'Cannot set max_memory_size_bytes to 10, because it is lower than total_block_size 65.'
test "set_max_memory_size_bytes" "$(dfx --identity default canister call archive_token set_max_memory_size_bytes "(100:nat64)" 2>&1)" '()'
test "query_metrics" "$(dfx canister call archive_token query_metrics 2>&1)" '( record { stable_memory_pages = 1_025 : nat64; stable_memory_bytes = 67_174_400 : nat64; heap_memory_bytes = 1_245_184 : nat64; last_upgrade_time_seconds = 0 : nat64; max_memory_size_bytes = 100 : nat64; blocks = 1 : nat64; blocks_bytes = 65 : nat64; block_height_offset = 0 : nat64; }, )'

blue "\n🚩 1.1 permission permission_query"
test "version" "$(dfx --identity alice canister call archive_token version 2>&1)" '(1 : nat32)'
test "permission_all" "$(dfx --identity alice canister call archive_token permission_all 2>&1)" 'vec { variant { Forbidden = "PauseQuery" }; variant { Permitted = "PauseReplace" }'
test "permission_query" "$(dfx --identity alice canister call archive_token permission_query 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_query" "$(dfx canister call archive_token permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_update" "$(dfx --identity bob canister call archive_token permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "'PermissionUpdate' is required"
test "permission_update" "$(dfx canister call archive_token permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "()"
test "🙈 permission_query" "$(dfx --identity alice canister call archive_token permission_query 2>&1)" "'PermissionQuery' is required"
test "permission_query" "$(dfx canister call archive_token permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_find_by_user" "$(dfx canister call archive_token permission_find_by_user "(principal \"$ALICE\")" 2>&1)" '(vec { "PauseQuery"; "PermissionUpdate"; "BusinessExampleQuery" })'
test "permission_update" "$(dfx --identity alice canister call archive_token permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call archive_token permission_query 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_query" "$(dfx canister call archive_token permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'

blue "\n🚩 1.2 permission permission update"
test "permission_query" "$(dfx canister call archive_token permission_query 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_query" "$(dfx --identity alice canister call archive_token permission_query 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_find_by_user" "$(dfx canister call archive_token permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" '( vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessExampleQuery"; "BusinessExampleSet";}, )'
test "permission_find_by_user" "$(dfx canister call archive_token permission_find_by_user "(principal \"$ALICE\")" 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_find_by_user" "$(dfx --identity alice canister call archive_token permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" "'PermissionFind' is required"
test "permission_find_by_user" "$(dfx --identity alice canister call archive_token permission_find_by_user "(principal \"$ALICE\")" 2>&1)" "'PermissionFind' is required"

blue "\n🚩 1.3 permission roles"
test "permission_query" "$(dfx --identity alice canister call archive_token permission_query 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_update" "$(dfx canister call archive_token permission_update "(vec { variant { UpdateRolePermission=record{\"Admin\"; opt vec {\"PauseReplace\"; \"PauseQuery\"} } } })" 2>&1)" "()"
test "permission_update" "$(dfx canister call archive_token permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; opt vec {\"Admin\"} } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call archive_token permission_query 2>&1)" '(vec { "PauseReplace"; "PermissionQuery"; "BusinessExampleQuery" })'
test "permission_update" "$(dfx canister call archive_token permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call archive_token permission_query 2>&1)" '(vec { "PauseQuery"; "PermissionQuery"; "BusinessExampleQuery" })'

blue "\n🚩 2.1 pause permission"
test "pause_query" "$(dfx canister call archive_token pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call archive_token pause_query_reason 2>&1)" "(null)"
test "pause_replace" "$(dfx canister call archive_token pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call archive_token pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx canister call archive_token pause_query_reason 2>&1)" "message = \"reason\""

blue "\n🚩 2.2 pause permission by alice"
test "pause_query" "$(dfx --identity alice canister call archive_token pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx --identity alice canister call archive_token pause_query_reason 2>&1)" "message = \"reason\""

blue "\n🚩 2.3 pause no permission"
test "pause_replace" "$(dfx --identity alice canister call archive_token pause_replace "(null)" 2>&1)" "'PauseReplace' is required"
test "permission_update" "$(dfx canister call archive_token permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PauseReplace\";\"PauseQuery\" } } } })" 2>&1)" "()"
test "pause_replace" "$(dfx --identity alice canister call archive_token pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx --identity alice canister call archive_token pause_query 2>&1)" "'PauseQuery' is required"
test "pause_query_reason" "$(dfx --identity alice canister call archive_token pause_query_reason 2>&1)" "'PauseQuery' is required"
test "pause_query" "$(dfx canister call archive_token pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call archive_token pause_query_reason 2>&1)" "(null)"

blue "\n🚩 3 record no permission"
test "record_topics" "$(dfx --identity alice canister call archive_token record_topics 2>&1)" "'RecordFind' is required"
test "record_topics" "$(dfx canister call archive_token record_topics 2>&1)" '"Example"' '"CyclesCharge"'
test "record_find_by_page" "$(dfx canister call archive_token record_find_by_page "(record{page=1:nat64;size=1:nat32},opt record{topic=opt vec{\"Pause\"}})" 2>&1)" "record { total = "
test "record_migrate" "$(dfx canister call archive_token record_migrate "(1:nat32)" 2>&1)" "removed = 0"

blue "\n🚩 4 schedule"
test "schedule_find" "$(dfx --identity alice canister call archive_token schedule_find 2>&1)" "'ScheduleFind' is required"
test "schedule_find" "$(dfx canister call archive_token schedule_find 2>&1)" "(null)"
test "schedule_replace" "$(dfx --identity alice canister call archive_token schedule_replace "(opt (1000000000:nat64))" 2>&1)" "'ScheduleReplace' is required"
test "schedule_replace" "$(dfx canister call archive_token schedule_replace "(opt (1000000000:nat64))" 2>&1)" "()"
sleep 3
test "schedule_replace" "$(dfx canister call archive_token schedule_replace "(null)" 2>&1)" "()"
sleep 2
test "schedule_trigger" "$(dfx --identity alice canister call archive_token schedule_trigger 2>&1)" "'ScheduleTrigger' is required"
test "schedule_trigger" "$(dfx canister call archive_token schedule_trigger 2>&1)" "()"

blue "\n🚩 5 example business"
test "business_example_query" "$(dfx --identity alice canister call archive_token business_example_query 2>&1)" "\"\""
test "business_example_query" "$(dfx canister call archive_token business_example_query 2>&1)" "\"\""
test "business_example_set" "$(dfx --identity alice canister call archive_token business_example_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_set" "$(dfx canister call archive_token business_example_set "(\"test string\")" 2>&1)" "()"
test "business_example_query" "$(dfx --identity alice canister call archive_token business_example_query 2>&1)" "test string"
test "business_example_query" "$(dfx canister call archive_token business_example_query 2>&1)" "test string"

blue "\n🚩 6 test archive_token data"
test "pause_replace" "$(dfx canister call archive_token pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call archive_token pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" archive_token
test "pause_replace" "$(dfx canister call archive_token pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call archive_token pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call archive_token business_example_query 2>&1)" "test string"

blue "\n🚩 7 test archive_token cell"
test "business_example_cell_query" "$(dfx --identity alice canister call archive_token business_example_cell_query 2>&1)" "\"\""
test "business_example_cell_query" "$(dfx canister call archive_token business_example_cell_query 2>&1)" "\"\""
test "business_example_cell_set" "$(dfx --identity alice canister call archive_token business_example_cell_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_cell_set" "$(dfx canister call archive_token business_example_cell_set "(\"test string\")" 2>&1)" "()"
test "business_example_cell_query" "$(dfx --identity alice canister call archive_token business_example_cell_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx canister call archive_token business_example_cell_query 2>&1)" "test string"

blue "\n🚩 8 test archive_token vec"
test "business_example_vec_query" "$(dfx --identity alice canister call archive_token business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call archive_token business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_pop" "$(dfx --identity alice canister call archive_token business_example_vec_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_pop" "$(dfx canister call archive_token business_example_vec_pop "()" 2>&1)" "(null)"
test "business_example_vec_push" "$(dfx --identity alice canister call archive_token business_example_vec_push "(5: nat64)" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_push" "$(dfx canister call archive_token business_example_vec_push "(5: nat64)" 2>&1)" "()"
test "business_example_vec_query" "$(dfx --identity alice canister call archive_token business_example_vec_query 2>&1)" "(vec { record { vec_data = 5 : nat64 } })"
test "business_example_vec_query" "$(dfx canister call archive_token business_example_vec_query 2>&1)" "(vec { record { vec_data = 5 : nat64 } })"
test "business_example_vec_pop" "$(dfx --identity alice canister call archive_token business_example_vec_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_vec_pop" "$(dfx canister call archive_token business_example_vec_pop "()" 2>&1)" "(opt record { vec_data = 5 : nat64 })"
test "business_example_vec_query" "$(dfx --identity alice canister call archive_token business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call archive_token business_example_vec_query 2>&1)" "(vec {})"

blue "\n🚩 9 test archive_token map"
test "business_example_map_query" "$(dfx --identity alice canister call archive_token business_example_map_query 2>&1)" "(vec {})"
test "business_example_map_query" "$(dfx canister call archive_token business_example_map_query 2>&1)" "(vec {})"
test "business_example_map_update" "$(dfx --identity alice canister call archive_token business_example_map_update "(1:nat64, opt \"111\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_map_update" "$(dfx canister call archive_token business_example_map_update "(1:nat64, opt \"111\")" 2>&1)" "(null)"
test "business_example_map_query" "$(dfx --identity alice canister call archive_token business_example_map_query 2>&1)" '(vec { record { 1 : nat64; "111" } })'
test "business_example_map_query" "$(dfx canister call archive_token business_example_map_query 2>&1)" '(vec { record { 1 : nat64; "111" } })'
test "business_example_map_update" "$(dfx canister call archive_token business_example_map_update "(1:nat64, opt \"123\")" 2>&1)" "(opt \"111\")"
test "business_example_map_update" "$(dfx canister call archive_token business_example_map_update "(1:nat64, null)" 2>&1)" "(opt \"123\")"
test "business_example_map_update" "$(dfx canister call archive_token business_example_map_update "(2:nat64, opt \"222\")" 2>&1)" "(null)"
test "business_example_map_query" "$(dfx --identity alice canister call archive_token business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_map_query" "$(dfx canister call archive_token business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'

blue "\n🚩 10 test archive_token log"
test "business_example_log_query" "$(dfx --identity alice canister call archive_token business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_query" "$(dfx canister call archive_token business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_update" "$(dfx --identity alice canister call archive_token business_example_log_update "(\"111\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_log_update" "$(dfx canister call archive_token business_example_log_update "(\"111\")" 2>&1)" "(0 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call archive_token business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_query" "$(dfx canister call archive_token business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_update" "$(dfx canister call archive_token business_example_log_update "(\"123\")" 2>&1)" "(1 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call archive_token business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_log_query" "$(dfx canister call archive_token business_example_log_query 2>&1)" '(vec { "111"; "123" })'

blue "\n🚩 11 test archive_token priority queue"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call archive_token business_example_priority_queue_query 2>&1)" "(vec {})"
test "business_example_priority_queue_query" "$(dfx canister call archive_token business_example_priority_queue_query 2>&1)" "(vec {})"
test "business_example_priority_queue_pop" "$(dfx --identity alice canister call archive_token business_example_priority_queue_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_pop" "$(dfx canister call archive_token business_example_priority_queue_pop "()" 2>&1)" "(null)"
test "business_example_priority_queue_push" "$(dfx --identity alice canister call archive_token business_example_priority_queue_push "(5: nat64)" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_push" "$(dfx canister call archive_token business_example_priority_queue_push "(5: nat64)" 2>&1)" "()"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call archive_token business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call archive_token business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_push" "$(dfx canister call archive_token business_example_priority_queue_push "(2: nat64)" 2>&1)" "()"
test "business_example_priority_queue_query" "$(dfx canister call archive_token business_example_priority_queue_query 2>&1)" "(vec { 2 : nat64; 5 : nat64 })"
test "business_example_priority_queue_pop" "$(dfx --identity alice canister call archive_token business_example_priority_queue_pop "()" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_priority_queue_pop" "$(dfx canister call archive_token business_example_priority_queue_pop "()" 2>&1)" "(opt (2 : nat64))"
test "business_example_priority_queue_query" "$(dfx --identity alice canister call archive_token business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call archive_token business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"

blue "\n🚩 12 test archive_token priority queue"
test "pause_replace" "$(dfx canister call archive_token pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call archive_token pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" archive_token
test "pause_replace" "$(dfx canister call archive_token pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call archive_token pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call archive_token business_example_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx --identity alice canister call archive_token business_example_cell_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx canister call archive_token business_example_cell_query 2>&1)" "test string"
test "business_example_vec_query" "$(dfx --identity alice canister call archive_token business_example_vec_query 2>&1)" "(vec {})"
test "business_example_vec_query" "$(dfx canister call archive_token business_example_vec_query 2>&1)" "(vec {})"
test "business_example_map_query" "$(dfx --identity alice canister call archive_token business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_map_query" "$(dfx canister call archive_token business_example_map_query 2>&1)" '(vec { record { 2 : nat64; "222" } })'
test "business_example_log_query" "$(dfx --identity alice canister call archive_token business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_log_query" "$(dfx canister call archive_token business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_priority_queue_query" "$(dfx --identity alice canister call archive_token business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"
test "business_example_priority_queue_query" "$(dfx canister call archive_token business_example_priority_queue_query 2>&1)" "(vec { 5 : nat64 })"

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
