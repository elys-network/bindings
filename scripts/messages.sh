#!/bin/bash

# Function to check if a command exists
command_exists() {
    type "$1" &>/dev/null
}

# Ensure jq is installed
if ! command_exists jq; then
    echo "jq is not installed. Please install jq to run this script."
    exit 1
fi

# Ensure awk is installed
if ! command_exists awk; then
    echo "awk is not installed. Please install awk to run this script."
    exit 1
fi

# Ensure elysd is installed
if ! command_exists elysd; then
    echo "elysd is not installed. Please install elysd to run this script."
    exit 1
fi

# Extract txhash from the output of a command
extract_txhash() {
    awk -F 'txhash: ' '/txhash:/{print $2; exit}'
}

# Extract response key value from the output of a command
extract_response_key_value() {
    response_key=$1
    awk '/"type": "'$response_key'"/{print "{"; flag=1;next}/]/{if(flag){print $0 "\n}"; exit}flag=0}flag'
}

# Define a function to query the contract state
query_contract() {
    local contract_address=$1
    local query=$2
    command="elysd q --output json --node \"$NODE\" wasm contract-state smart \"$contract_address\" '$query' | jq"
    echo "$ $command"
    eval $command
}

# Define a function to execute a contract message
execute_message() {
    local start_time=$SECONDS
    local contract_address=$1
    local message=$2
    local response_key=$3
    local amount=$4
    local options="--from \"$NAME\" --keyring-backend test --node \"$NODE\" --chain-id elystestnet-1 --gas auto --gas-adjustment=1.3 --fees 100000uelys -b sync -y"
    # if amount is set, then add the amount as the amount in options
    if [ ! -z "$amount" ]; then
        options="$options --amount $amount"
    fi
    command="elysd tx wasm exec $options \"$contract_address\" '$message'"
    echo "$ $command"
    txhash=$(eval $command | extract_txhash)
    echo "txhash: $txhash"
    # check if txhash is empty
    if [ -z "$txhash" ]; then
        echo "Failed to execute the message. Please check the error message above."
        exit 1
    fi

    # loop until query tx cli does not fail
    while ! elysd q tx $txhash --node "$NODE" &>/dev/null; do
        echo "Waiting for the transaction to be included in a block..."
        sleep 0.5
    done

    local end_time=$SECONDS

    # Query the transaction
    local result=$(elysd q tx $txhash --node "$NODE" --output json)

    # Extract the response key value from the result
    echo $result | jq | extract_response_key_value $response_key | jq

    # Extract the logs from the result
    # echo $result | jq '.logs'

    # Calculate the duration
    local duration=$((end_time - start_time))
    echo "Total execution time: $duration seconds."
}

# Environment variables
if [ -z "$NODE" ]; then
    NODE="https://rpc.testnet.elys.network:443"
fi
if [ -z "$NAME" ]; then
    NAME="contract-initiator"
fi

printf "# Node: %s\n" "$NODE"
printf "# Name: %s\n" "$NAME"

# Ensure that the account has been set using elysd keys show command
if ! elysd keys show $NAME &>/dev/null; then
    echo "$NAME account has not been set. Please set the $NAME account using elysd keys show command."
    exit 1
fi

# Contract addresses
if [ -n "$AH_CONTRACT_ADDRESS" ]; then
    ah_contract_address=$AH_CONTRACT_ADDRESS
else
    ah_contract_address="elys1s37xz7tzrru2cpl96juu9lfqrsd4jh73j9slyv440q5vttx2uyesetjpne"
fi
if [ -n "$FS_CONTRACT_ADDRESS" ]; then
    fs_contract_address=$FS_CONTRACT_ADDRESS
else
    fs_contract_address="elys1g2xwx805epc897rwyrykskjque07yxfmc4qq2p4ef5dwd6znl30qnxje76"
fi
if [ -n "$TS_CONTRACT_ADDRESS" ]; then
    ts_contract_address=$TS_CONTRACT_ADDRESS
else
    ts_contract_address="elys1m3hduhk4uzxn8mxuvpz02ysndxfwgy5mq60h4c34qqn67xud584qeee3m4"
fi

# Print contract addresses
printf "# AH contract address: %s\n" "$ah_contract_address"
printf "# FS contract address: %s\n" "$fs_contract_address"
printf "# TS contract address: %s\n" "$ts_contract_address"

# Denoms
usdc_denom="ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
atom_denom="ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4"

# Print denoms
printf "\n# USDC denom: %s\n" "$usdc_denom"
printf "# ATOM denom: %s\n" "$atom_denom"

# User address
user_address=$(elysd keys show $NAME -a)

# Print user address
printf "\n# User address: %s\n" "$user_address"

# Function definitions for each query

# Create spot order
function create_spot_order() {
    order_type=$1
    source=$2
    target=$3
    order_price=$4
    printf "\n# Create spot order as $1 source=$2 target=$3 order_price=$4\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "create_spot_order": {
                "order_price": {
                    "base_denom": "'"$source"'",
                    "quote_denom": "'"$target"'",
                    "rate": "'"$order_price"'"
                },
                "order_type": "'"$order_type"'",
                "order_source_denom": "'"$source"'",
                "order_target_denom": "'"$target"'"
            }
        }' \
        wasm-create_spot_order \
        "1000000$source"
}

# amm swap exact amount in
function amm_swap_exact_amount_in() {
    printf "\n# AMM swap exact amount in\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "amm_swap_exact_amount_in": {
                "routes": [
                    {
                        "pool_id": 3,
                        "token_out_denom": "'"$usdc_denom"'"
                    }
                ]
            }
        }' \
        wasm-amm_swap_exact_amount_in \
        "1000000uelys"
}

# amm swap exact amount in with multiple routes
function amm_swap_exact_amount_in_with_multiple_routes() {
    printf "\n# AMM swap exact amount in with multiple routes\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "amm_swap_exact_amount_in": {
                "routes": [
                    {
                        "pool_id": 3,
                        "token_out_denom": "'"$usdc_denom"'"
                    },
                    {
                        "pool_id": 2,
                        "token_out_denom": "'"$atom_denom"'"
                    }
                ]
            }
        }' \
        wasm-amm_swap_exact_amount_in \
        "10000000uelys"
}

# Create spot order as market buy
function create_spot_order_as_market_buy() {
    source=$1
    target=$2
    printf "\n# Create spot order as market buy source=$source target=$target\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "create_spot_order": {
                "order_type": "market_buy",
                "order_target_denom": "'"$target"'",
                "order_source_denom": "'"$source"'"
            }
        }' \
        wasm-create_spot_order \
        "10000000$source"
}

# Create perpetual order as market open
function create_perpetual_order_as_market_open() {
    printf "\n# Create perpetual order as market open\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "create_perpetual_order": {
                "position": "short",
                "leverage": "5",
                "trading_asset": "'"$atom_denom"'",
                "take_profit_price": "30",
                "order_type": "market_open"
            }
        }' \
        wasm-create_perpetual_order \
        "101000000$usdc_denom"
}

# Create perpetual order as market close
function create_perpetual_order_as_market_close() {
    position_id=$1
    printf "\n# Create perpetual order as market close\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "create_perpetual_order": {
                "order_type": "market_close",
                "position_id": '"$position_id"'
            }
        }' \
        wasm-create_perpetual_order
}

# Create perpetual order
function create_perpetual_order() {
    order_type=$1
    printf "\n# Create perpetual order as order_type=$order_type\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "create_perpetual_order": {
                "position": "long",
                "leverage": "5",
                "trading_asset": "'"$atom_denom"'",
                "order_type": "'"$order_type"'",
                "trigger_price": {
                    "base_denom": "'"$usdc_denom"'",
                    "quote_denom": "'"$atom_denom"'",
                    "rate": "16"
                }
            }
        }' \
        wasm-create_perpetual_order \
        "120000000$usdc_denom"
}

# Perpetual Close Position
function perpetual_close_position() {
    order_id=$1
    amount=$2
    printf "\n# Perpetual Close Position"
    execute_message \
        "$ts_contract_address" \
        '{
            "close_perpetual_position": {
                "id" : '"$order_id"',
                "amount" : "'"$amount"'"
            }
        }' \
        wasm-close_perpetual_position
}

# Perpetual open estimation
function perpetual_open_estimation() {
    printf "\n# Perpetual open estimation\n"
    query_contract "$ts_contract_address" '{
        "perpetual_open_estimation": {
            "position": "long",
            "leverage": "5",
            "trading_asset": "'"$atom_denom"'",
            "collateral": {"denom": "'"$usdc_denom"'", "amount": "100000000"},
            "user_address": "'"$user_address"'"
        }
    }'
}

# Cancel spot order
function cancel_spot_order() {
    order_id=$1
    printf "\n# Cancel spot order with id $order_id\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "cancel_spot_order": {
                "order_id": '"$order_id"'
            }
        }' \
        wasm-cancel_spot_order
}

# Cancel perpetual order
function cancel_perpetual_order() {
    order_id=$1
    printf "\n# Cancel perpetual order with id $order_id\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "cancel_perpetual_order": {
                "order_id": '"$order_id"'
            }
        }' \
        wasm-cancel_perpetual_order
}

# Open a leverage lp position
function leveragelp_open() {
    execute_message \
        "$ts_contract_address" \
        '{
            "leveragelp_open": {
                "amm_pool_id": 2,
                "collateral_asset": "'"$usdc_denom"'",
                "collateral_amount": "10000000",
                "leverage": "1.10",
                "stop_loss_price": "0.0"
            }
        }' \
        wasm-cancel_perpetual_order
}

# Close a leverage lp position
function leveragelp_close() {
    execute_message \
        "$ts_contract_address" \
        '{
            "leveragelp_close": {
                "position_id" : 2,
                "amount" : "5000000"
            }
        }' \
        wasm-cancel_perpetual_order
}

# Get all spot orders
function all_spot_orders() {
    printf "\n# Get all spot orders\n"
    query_contract "$ts_contract_address" '{
        "get_spot_orders": {
            "order_owner": "'"$user_address"'"
        }
    }'
}

# Get spot order
function spot_order() {
    order_id=$1
    printf "\n# Spot order for id $order_id\n"
    query_contract "$ts_contract_address" '{
        "get_spot_order": {
            "order_id": '"$order_id"'
        }
    }'
}

# Get all perpetual orders
function all_perpetual_orders() {
    printf "\n# Get all perpetual orders\n"
    query_contract "$ts_contract_address" '{
        "get_perpetual_orders": {
            "order_owner": "'"$user_address"'"
        }
    }'
}

function enable_all_params() {
    printf "\n# Set Params\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "set_params": {
                "market_order_enabled": true,
                "stake_enabled": true,
                "process_order_enabled": true,
                "swap_enabled": true,
                "perpetual_enabled": true,
                "reward_enabled": true,
                "leverage_enabled": true
            }
        }' \
        wasm-set_params
}

function disable_all_params() {
    printf "\n# Set Params\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "set_params": {
                "market_order_enabled": false,
                "stake_enabled": false,
                "process_order_enabled": false,
                "swap_enabled": false,
                "perpetual_enabled": false,
                "reward_enabled": false,
                "leverage_enabled": false
            }
        }' \
        wasm-set_params
}

function set_limit_to_process_orders() {
    #set limit to process orders: set to zero for the limit to be NONE
    limit_number=$1
    printf "\n# Set Params\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "set_params": {
                "limit_process_order": "'"$limit_number"'"
            }
        }' \
        wasm-set_params
}

function eden_claim_vesting_request() {
    #set limit to process orders: set to zero for the limit to be NONE
    limit_number=$1
    printf "\n# Eden Claim Vesting Request\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "eden_claim_vesting_request": {}
        }' \
        wasm-eden_claim_vesting_request
}

function claim_rewards_request() {
    printf "\n# claim_rewards_request\n"
    execute_message \
    "$ts_contract_address" \
    '{
        "claim_rewards_request": {}
    }' \
    "wasm-claim_rewards_request"
}

function masterchef_claim_rewards() {
    pool_ids=$1
    printf "\n# Eden Claim Vesting Request\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "masterchef_claim_rewards": { 
                "pool_ids": "'"$pool_ids"'"
            }
        }' \
        wasm-eden_claim_vesting_request
}

function stake_request() {
    validator_address=$1
    printf "\n# Stake Request\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "stake_request": {
                "amount": 100000,
                "asset": "uelys",
                "validator_address": "'"$validator_address"'"
            }
        }' \
        delegate
}

function update_account() {
    printf "\n# Update Account\n"
    execute_message \
        "$ah_contract_address" \
        '{
            "update_account": {}
        }'
}

# function(s) to run based on the provided argument
case "$1" in
"amm_swap_exact_amount_in")
    amm_swap_exact_amount_in
    ;;
"amm_swap_exact_amount_in_with_multiple_routes")
    amm_swap_exact_amount_in_with_multiple_routes
    ;;
"create_spot_order_as_market_buy")
    create_spot_order_as_market_buy $2 $3
    ;;
"create_spot_order_as_limit_buy")
    create_spot_order "limit_buy" $2 $3 $4
    ;;
"create_spot_order_as_limit_sell")
    create_spot_order "limit_sell" $2 $3 $4
    ;;
"create_spot_order_as_stop_loss")
    create_spot_order "stop_loss" $2 $3 $4
    ;;
"all_spot_orders")
    all_spot_orders
    ;;
"spot_order")
    spot_order $2
    ;;
"cancel_spot_order")
    cancel_spot_order $2
    ;;
"create_perpetual_order_as_market_open")
    create_perpetual_order_as_market_open
    ;;
"create_perpetual_order_as_market_close")
    create_perpetual_order_as_market_close $2
    ;;
"create_perpetual_order_as_limit_open")
    create_perpetual_order "limit_open"
    ;;
"perpetual_close_position")
    perpetual_close_position $2 $3
    ;;
"perpetual_open_estimation")
    perpetual_open_estimation
    ;;
"all_perpetual_orders")
    all_perpetual_orders
    ;;
"cancel_perpetual_order")
    cancel_perpetual_order $2
    ;;
"leveragelp_open")
    leveragelp_open
    ;;
"leveragelp_close")
    leveragelp_close
    ;;
"enable_all_params")
    enable_all_params
    ;;
"disable_all_params")
    disable_all_params
    ;;
"set_limit_to_process_orders")
    set_limit_to_process_orders $2
    ;;
"eden_claim_vesting_request")
    eden_claim_vesting_request
    ;;
"masterchef_claim_rewards")
    masterchef_claim_rewards $2
    ;;
"claim_rewards_request")
    claim_rewards_request
    ;;
"stake_request")
    stake_request $2
    ;;
"update_account")
    update_account
    ;;
*)
    # Default case: run all functions
    all_spot_orders
    all_perpetual_orders
    ;;
esac