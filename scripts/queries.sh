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

# Denoms
usdc_denom="ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
atom_denom="ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4"

# Print denoms
printf "\n# USDC denom: %s\n" "$usdc_denom"
printf "# ATOM denom: %s\n" "$atom_denom"

# User address
user_address="$1"
printf "\n# User address: %s\n" "$user_address"

# Get AH params
printf "\n# AH Params\n"
query_contract "$ah_contract_address" '{
    "params": {}
}'

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

# Swap estimation by denom
printf "\n# Swap estimation by denom\n"
query_contract "$ts_contract_address" '{
    "swap_estimation_by_denom": {
        "user_address": "'"$user_address"'",
        "amount": {
            "amount": "1000000",
            "denom": "uelys"
        },
        "denom_in": "'"$usdc_denom"'",
        "denom_out": "uelys"
    }
}'

# Get all prices
printf "\n# All prices\n"
query_contract "$ts_contract_address" '{
    "get_all_prices": {
        "limit": 2
    }
}'

# Asset info
printf "\n# Asset info\n"
query_contract "$ts_contract_address" '{
    "asset_info": {
        "denom": "uelys"
    }
}'

# Get spot order
printf "\n# Spot order\n"
query_contract "$ts_contract_address" '{
    "get_spot_order": {
        "order_id": 1
    }
}'

# Get spot orders for stop loss
printf "\n# Get stop loss spot orders\n"
query_contract "$ts_contract_address" '{
    "get_spot_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "stop_loss",
        "order_status": null
    }
}'

# Get spot orders for limit sell
printf "\n# Get limit sell spot orders\n"
query_contract "$ts_contract_address" '{
    "get_spot_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "limit_sell",
        "order_status": null
    }
}'

# Get spot orders for limit buy
printf "\n# Get limit buy spot orders\n"
query_contract "$ts_contract_address" '{
    "get_spot_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "limit_buy",
        "order_status": null
    }
}'

# Get spot orders for market buy
printf "\n# Get market buy spot orders\n"
query_contract "$ts_contract_address" '{
    "get_spot_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "market_buy",
        "order_status": null
    }
}'

# Get margin position
printf "\n# Get margin position\n"
query_contract "$ts_contract_address" '{
    "get_margin_position": {
        "address": "'"$user_address"'",
        "id": 1
    }
}'

# Get margin order
printf "\n# Margin order\n"
query_contract "$ts_contract_address" '{
    "get_margin_order": {
        "id": 1
    }
}'

# Get margin orders for stop loss
printf "\n# Get stop loss margin orders\n"
query_contract "$ts_contract_address" '{
    "get_margin_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "stop_loss",
        "order_status": null
    }
}'

# Get margin orders for limit open
printf "\n# Get limit open margin orders\n"
query_contract "$ts_contract_address" '{
    "get_margin_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "limit_open",
        "order_status": null
    }
}'

# Get margin orders for limit close
printf "\n# Get limit close margin orders\n"
query_contract "$ts_contract_address" '{
    "get_margin_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "limit_close",
        "order_status": null
    }
}'

# Get margin orders for market open
printf "\n# Get market open margin orders\n"
query_contract "$ts_contract_address" '{
    "get_margin_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "market_open",
        "order_status": null
    }
}'

# Get margin orders for market close
printf "\n# Get market close margin orders\n"
query_contract "$ts_contract_address" '{
    "get_margin_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "market_close",
        "order_status": null
    }
}'

# Get margin orders for stop loss
printf "\n# Get stop loss margin orders\n"
query_contract "$ts_contract_address" '{
    "get_margin_orders": {
        "pagination": null,
        "order_owner": "'"$user_address"'",
        "order_type": "stop_loss",
        "order_status": null
    }
}'

# Margin open estimation
printf "\n# Margin open estimation\n"
query_contract "$ts_contract_address" '{
    "margin_open_estimation": {
        "position": "long",
        "leverage": "5",
        "trading_asset": "'"$atom_denom"'",
        "collateral": {"denom": "'"$usdc_denom"'", "amount": "100000000"},
        "take_profit_price": "30",
        "user_address": null
    }
}'

# margin get position for address
printf "\n# Margin get position for address\n"
query_contract "$ts_contract_address" '{
    "margin_get_positions_for_address": {
        "address": "'"$user_address"'",
        "pagination": null
    }
}'