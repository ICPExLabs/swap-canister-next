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

# ! 1. 测试 swap
red "\n=========== 1. swap ===========\n"
dfx canister create swap # --with-cycles 50T
dfx deploy --mode=reinstall --yes --argument "(null)" swap
swap=$(canister_id "swap")
blue "Swap Canister: $swap"

if [ -z "$swap" ]; then
    say deploy failed
    exit 1
fi

blue "1.1 permission permission_query"
test "version" "$(dfx --identity alice canister call swap version 2>&1)" '(1 : nat32)'
test "permission_all" "$(dfx --identity alice canister call swap permission_all 2>&1)" 'vec { variant { Forbidden = "PauseQuery" }; variant { Permitted = "PauseReplace" }'
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; }, )'
test "permission_query" "$(dfx canister call swap permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_update" "$(dfx --identity bob canister call swap permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "'PermissionUpdate' is required"
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PermissionUpdate\";\"PermissionQuery\" } } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" "'PermissionQuery' is required"
test "permission_query" "$(dfx canister call swap permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_find_by_user" "$(dfx canister call swap permission_find_by_user "(principal \"$ALICE\")" 2>&1)" '( vec { "PauseQuery"; "PermissionUpdate"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; }, )'
test "permission_update" "$(dfx --identity alice canister call swap permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" '( vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; }, )'
test "permission_query" "$(dfx canister call swap permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; "BusinessExampleSet";}'

blue "1.2 permission permission update"
test "permission_query" "$(dfx canister call swap permission_query 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; }'
test "permission_find_by_user" "$(dfx canister call swap permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" 'vec { "PauseQuery"; "PauseReplace"; "PermissionQuery"; "PermissionFind"; "PermissionUpdate"; "RecordFind"; "RecordMigrate"; "ScheduleFind"; "ScheduleReplace"; "ScheduleTrigger"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; "BusinessExampleSet";}'
test "permission_find_by_user" "$(dfx canister call swap permission_find_by_user "(principal \"$ALICE\")" 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; }'
test "permission_find_by_user" "$(dfx --identity alice canister call swap permission_find_by_user "(principal \"$DEFAULT\")" 2>&1)" "'PermissionFind' is required"
test "permission_find_by_user" "$(dfx --identity alice canister call swap permission_find_by_user "(principal \"$ALICE\")" 2>&1)" "'PermissionFind' is required"

blue "1.3 permission roles"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; }'
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateRolePermission=record{\"Admin\"; opt vec {\"PauseReplace\"; \"PauseQuery\"} } } })" 2>&1)" "()"
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; opt vec {\"Admin\"} } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" '( vec { "PauseReplace"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; }, )'
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateUserRole=record{principal \"$ALICE\"; null } } })" 2>&1)" "()"
test "permission_query" "$(dfx --identity alice canister call swap permission_query 2>&1)" 'vec { "PauseQuery"; "PermissionQuery"; "BusinessTokenDeposit"; "BusinessTokenWithdraw"; "BusinessExampleQuery"; }'

blue "2.1 pause permission"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call swap pause_query_reason 2>&1)" "(null)"
test "pause_replace" "$(dfx canister call swap pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx canister call swap pause_query_reason 2>&1)" "message = \"reason\""

blue "2.2 pause permission by alice"
test "pause_query" "$(dfx --identity alice canister call swap pause_query 2>&1)" "(true)"
test "pause_query_reason" "$(dfx --identity alice canister call swap pause_query_reason 2>&1)" "message = \"reason\""

blue "2.3 pause no permission"
test "pause_replace" "$(dfx --identity alice canister call swap pause_replace "(null)" 2>&1)" "'PauseReplace' is required"
test "permission_update" "$(dfx canister call swap permission_update "(vec { variant { UpdateUserPermission=record{principal \"$ALICE\"; opt vec { \"PauseReplace\";\"PauseQuery\" } } } })" 2>&1)" "()"
test "pause_replace" "$(dfx --identity alice canister call swap pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx --identity alice canister call swap pause_query 2>&1)" "'PauseQuery' is required"
test "pause_query_reason" "$(dfx --identity alice canister call swap pause_query_reason 2>&1)" "'PauseQuery' is required"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(false)"
test "pause_query_reason" "$(dfx canister call swap pause_query_reason 2>&1)" "(null)"

blue "3 record no permission"
test "record_topics" "$(dfx --identity alice canister call swap record_topics 2>&1)" "'RecordFind' is required"
test "record_topics" "$(dfx canister call swap record_topics 2>&1)" '"Example"' '"CyclesCharge"'
test "record_find_by_page" "$(dfx canister call swap record_find_by_page "(record{page=1:nat64;size=1:nat32},opt record{topic=opt vec{\"Pause\"}})" 2>&1)" "record { total = "
test "record_migrate" "$(dfx canister call swap record_migrate "(1:nat32)" 2>&1)" "removed = 0"

blue "4 schedule"
test "schedule_find" "$(dfx --identity alice canister call swap schedule_find 2>&1)" "'ScheduleFind' is required"
test "schedule_find" "$(dfx canister call swap schedule_find 2>&1)" "(null)"
test "schedule_replace" "$(dfx --identity alice canister call swap schedule_replace "(opt (1000000000:nat64))" 2>&1)" "'ScheduleReplace' is required"
test "schedule_replace" "$(dfx canister call swap schedule_replace "(opt (1000000000:nat64))" 2>&1)" "()"
sleep 3
test "schedule_replace" "$(dfx canister call swap schedule_replace "(null)" 2>&1)" "()"
sleep 2
test "schedule_trigger" "$(dfx --identity alice canister call swap schedule_trigger 2>&1)" "'ScheduleTrigger' is required"
test "schedule_trigger" "$(dfx canister call swap schedule_trigger 2>&1)" "()"

blue "5 business tokens"
test "tokens_query" "$(dfx --identity alice canister call swap tokens_query 2>&1)" '"ICP"' '"ckUSDT'
test "token_query" "$(dfx --identity alice canister call swap token_query "(principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\")" 2>&1)" '"Internet Computer"'
test "token_balance_of" "$(dfx --identity alice canister call swap token_balance_of "(principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\", record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '(0 : nat)'
test "tokens_balance_of" "$(dfx --identity alice canister call swap tokens_balance_of "(record { owner=principal \"$DEFAULT\"; subaccount=null})" 2>&1)" '( vec { record { principal "' 'record { principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; 0 : nat;}'

blue "5 example business"
test "business_example_query" "$(dfx --identity alice canister call swap business_example_query 2>&1)" "\"\""
test "business_example_query" "$(dfx canister call swap business_example_query 2>&1)" "\"\""
test "business_example_set" "$(dfx --identity alice canister call swap business_example_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_set" "$(dfx canister call swap business_example_set "(\"test string\")" 2>&1)" "()"
test "business_example_query" "$(dfx --identity alice canister call swap business_example_query 2>&1)" "test string"
test "business_example_query" "$(dfx canister call swap business_example_query 2>&1)" "test string"

blue "6 test swap data"
test "pause_replace" "$(dfx canister call swap pause_replace "(opt \"reason\")" 2>&1)" "()"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(true)"
dfx canister install --mode=upgrade --upgrade-unchanged --argument "(null)" swap
test "pause_replace" "$(dfx canister call swap pause_replace "(null)" 2>&1)" "()"
test "pause_query" "$(dfx canister call swap pause_query 2>&1)" "(false)"
test "business_example_query" "$(dfx canister call swap business_example_query 2>&1)" "test string"

blue "7 test swap cell"
test "business_example_cell_query" "$(dfx --identity alice canister call swap business_example_cell_query 2>&1)" "\"\""
test "business_example_cell_query" "$(dfx canister call swap business_example_cell_query 2>&1)" "\"\""
test "business_example_cell_set" "$(dfx --identity alice canister call swap business_example_cell_set "(\"test string\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_cell_set" "$(dfx canister call swap business_example_cell_set "(\"test string\")" 2>&1)" "()"
test "business_example_cell_query" "$(dfx --identity alice canister call swap business_example_cell_query 2>&1)" "test string"
test "business_example_cell_query" "$(dfx canister call swap business_example_cell_query 2>&1)" "test string"

blue "8 test swap vec"
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

blue "9 test swap map"
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

blue "10 test swap log"
test "business_example_log_query" "$(dfx --identity alice canister call swap business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_query" "$(dfx canister call swap business_example_log_query 2>&1)" "(vec {})"
test "business_example_log_update" "$(dfx --identity alice canister call swap business_example_log_update "(\"111\")" 2>&1)" "'BusinessExampleSet' is required"
test "business_example_log_update" "$(dfx canister call swap business_example_log_update "(\"111\")" 2>&1)" "(0 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call swap business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_query" "$(dfx canister call swap business_example_log_query 2>&1)" '(vec { "111" })'
test "business_example_log_update" "$(dfx canister call swap business_example_log_update "(\"123\")" 2>&1)" "(1 : nat64)"
test "business_example_log_query" "$(dfx --identity alice canister call swap business_example_log_query 2>&1)" '(vec { "111"; "123" })'
test "business_example_log_query" "$(dfx canister call swap business_example_log_query 2>&1)" '(vec { "111"; "123" })'

blue "11 test swap priority queue"
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

blue "12 test swap priority queue"
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
