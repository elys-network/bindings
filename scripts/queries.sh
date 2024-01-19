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

# Ensure jq is installed
if ! command_exists elysd; then
    echo "elysd is not installed. Please install elysd to run this script."
    exit 1
fi

# Define a function to query the contract state
query_contract() {
    local contract_address=$1
    local query=$2
    command="elysd q --output json --node \"$NODE\" wasm contract-state smart \"$contract_address\" '$query' | jq"
    echo "$ $command"
    eval $command
}

# Check if the first argument is provided
if [ -z "${1:-}" ]; then
    echo "No argument supplied"
    exit 1
fi

# Environment variables
NODE="https://rpc.testnet.elys.network:443"

# Contract addresses
ah_contract_address="elys1s37xz7tzrru2cpl96juu9lfqrsd4jh73j9slyv440q5vttx2uyesetjpne"
fs_contract_address="elys1g2xwx805epc897rwyrykskjque07yxfmc4qq2p4ef5dwd6znl30qnxje76"
ts_contract_address="elys1m3hduhk4uzxn8mxuvpz02ysndxfwgy5mq60h4c34qqn67xud584qeee3m4"

# Print contract addresses
printf "# AH contract address: %s\n" "$ah_contract_address"
printf "# FS contract address: %s\n" "$fs_contract_address"
printf "# TS contract address: %s\n" "$ts_contract_address"

# User address
user_address="$1"
printf "\n# User address: %s\n" "$user_address"

# Get total balance
printf "\n# Total balance\n"
query_contract "$ah_contract_address" '{
    "get_total_balance": {
        "user_address": "'"$user_address"'"
    }
}'

# Get membership tier
printf "\n# Membership tier\n"
query_contract "$ah_contract_address" '{
    "get_membership_tier": {
        "user_address": "'"$user_address"'"
    }
}'

# Get portfolio balance
printf "\n# Portfolio balance\n"
query_contract "$ah_contract_address" '{
    "get_portfolio": {
        "user_address": "'"$user_address"'"
    }
}'

# Get rewards
printf "\n# Rewards\n"
query_contract "$ah_contract_address" '{
    "get_rewards": {
        "user_address": "'"$user_address"'"
    }
}'

# Get liquid assets
printf "\n# Liquid assets\n"
query_contract "$ah_contract_address" '{
    "get_liquid_assets": {
        "user_address": "'"$user_address"'"
    }
}'

# Get staked assets
printf "\n# Staked assets\n"
query_contract "$ah_contract_address" '{
    "get_staked_assets": {
        "user_address": "'"$user_address"'"
    }
}'

# Get user value
printf "\n# User value\n"
query_contract "$ah_contract_address" '{
    "user_value": {
        "user_address": "'"$user_address"'"
    }
}'