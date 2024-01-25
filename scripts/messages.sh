#!/bin/bash

# Function to check if a command exists
command_exists() {
    type "$1" &> /dev/null
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
    awk -F 'txhash: ' '/txhash:/{print $2; exit}';
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
    sleep 5
    elysd q tx $txhash --node "$NODE" --output json | jq | awk '/"type": "'$response_key'"/{print "{"; flag=1;next}/]/{if(flag){print $0 "\n}"; exit}flag=0}flag' | jq
}

# Environment variables
NODE="https://rpc.testnet.elys.network:443"
NAME=contract-initiator

# Ensure that the account has been set using elysd keys show command
if ! elysd keys show $NAME &> /dev/null; then
    echo "$NAME account has not been set. Please set the $NAME account using elysd keys show command."
    exit 1
fi

# Contract addresses
ah_contract_address="elys1s37xz7tzrru2cpl96juu9lfqrsd4jh73j9slyv440q5vttx2uyesetjpne"
fs_contract_address="elys1g2xwx805epc897rwyrykskjque07yxfmc4qq2p4ef5dwd6znl30qnxje76"
ts_contract_address="elys1m3hduhk4uzxn8mxuvpz02ysndxfwgy5mq60h4c34qqn67xud584qeee3m4"

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
    order_price=$2
    printf "\n# Create spot order as $1\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "create_spot_order": {
                "order_price": {
                    "base_denom": "'"$usdc_denom"'",
                    "quote_denom": "'"$atom_denom"'",
                    "rate": "'"$order_price"'"
                },
                "order_type": "'"$order_type"'",
                "order_target_denom": "'"$atom_denom"'",
                "order_source_denom": "'"$usdc_denom"'"
            }
        }' \
        wasm-create_spot_order \
        "1000000$usdc_denom"
}

# Create spot order as market buy
function create_spot_order_as_market_buy() {
    printf "\n# Create spot order as market buy\n"
    execute_message \
        "$ts_contract_address" \
        '{
            "create_spot_order": {
                "order_price": null,
                "order_type": "market_buy",
                "order_target_denom": "'"$atom_denom"'",
                "order_source_denom": "'"$usdc_denom"'"
            }
        }' \
        wasm-create_spot_order \
        "1000000$usdc_denom"
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

# Get all spot orders
function all_spot_orders() {
    printf "\n# Get all spot orders\n"
    query_contract "$ts_contract_address" '{
        "get_spot_orders": {
            "pagination": null,
            "order_owner": "'"$user_address"'",
            "order_type": null,
            "order_status": null
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

# function(s) to run based on the provided argument
case "$1" in
    "create_spot_order_as_market_buy")
        create_spot_order_as_market_buy
        ;;
    "create_spot_order_as_limit_buy")
        create_spot_order "limit_buy" 0.1
        ;;
    "create_spot_order_as_limit_sell")
        create_spot_order "limit_sell" 100
        ;;
    "create_spot_order_as_stop_loss")
        create_spot_order "stop_loss" 0.1
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

    *)
        # Default case: run all functions
        create_spot_order
        all_spot_orders
        cancel_spot_order
        ;;
esac