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

# Ensure elysd is installed
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

# Environment variables
if [ -z "$NODE" ]; then
    NODE="https://rpc.testnet.elys.network:443"
fi

printf "# Node: %s\n" "$NODE"

# Contract addresses
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
printf "# FS contract address: %s\n" "$fs_contract_address"
printf "# TS contract address: %s\n" "$ts_contract_address"

# Denoms
elys_denom="uelys"
eden_denom="ueden"
edenb_denom="uedenb"
usdc_denom="ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
atom_denom="ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4"
osmo_denom="ibc/B4314D0E670CB43C88A5DCA09F76E5E812BD831CC2FEC6E434C9E5A9D1F57953"

# Print denoms
printf "\n# ELYS denom: %s\n" "$elys_denom"
printf "# EDEN denom: %s\n" "$eden_denom"
printf "# EDENB denom: %s\n" "$edenb_denom"
printf "# USDC denom: %s\n" "$usdc_denom"
printf "# ATOM denom: %s\n" "$atom_denom"

# User address
user_address="${1:-}"

# Print user address
printf "\n# User address: %s\n" "$user_address"

# Function definitions for each query

# Swap estimation by denom
function swap_estimation_by_denom() {
    amount=$1
    denom=$2
    denom_in=$3
    denom_out=$4
    printf "\n# Swap estimation by denom with amount="$amount""$amount_denom", denom_in="$denom_in", denom_out="$denom_out"\n"
    query_contract "$ts_contract_address" '{
        "swap_estimation_by_denom": {
            "user_address": "'"$user_address"'",
            "amount": {
                "amount": "'"$amount"'",
                "denom": "'"$denom"'"
            },
            "denom_in": "'"$denom_in"'",
            "denom_out": "'"$denom_out"'"
        }
    }'
}

# Get all prices
function all_prices() {
    printf "\n# All prices\n"
    query_contract "$ts_contract_address" '{
        "get_all_prices": {
            "limit": 2
        }
    }'
}

# Asset info
function asset_info() {
    printf "\n# Asset info\n"
    query_contract "$ts_contract_address" '{
        "asset_info": {
            "denom": "uelys"
        }
    }'
}

# Get spot order
function spot_order() {
    order_id=$1
    printf "\n# Spot order order_id=$order_id\n"
    query_contract "$ts_contract_address" '{
        "get_spot_order": {
            "order_id": '"$order_id"'
        }
    }'
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

# Get spot orders
function spot_orders() {
    printf "\n# Get spot orders with $1\n"
    query_contract "$ts_contract_address" '{
        "get_spot_orders": {
            "pagination": null,
            "order_owner": "'"$user_address"'",
            "order_type": "'$1'",
            "order_status": null
        }
    }'
}

# Get perpetual position
function perpetual_position() {
    printf "\n# Get perpetual position\n"
    query_contract "$ts_contract_address" '{
        "get_perpetual_position": {
            "address": "'"$user_address"'",
            "id": 1
        }
    }'
}

# Get perpetual order
function perpetual_order() {
    printf "\n# Perpetual order\n"
    query_contract "$ts_contract_address" '{
        "get_perpetual_order": {
            "id": 1
        }
    }'
}

# Get perpetual orders
function perpetual_orders() {
    printf "\n# Get perpetual orders with $1\n"
    query_contract "$ts_contract_address" '{
        "get_perpetual_orders": {
            "pagination": null,
            "order_owner": "'"$user_address"'",
            "order_type": "'$1'",
            "order_status": null
        }
    }'
}

# Perpetual open estimation
function perpetual_open_estimation() {
    printf "\n# Perpetual open estimation\n"
    query_contract "$ts_contract_address" '{
        "perpetual_open_estimation": {
            "position": "short",
            "leverage": "5",
            "trading_asset": "'"$atom_denom"'",
            "collateral": {"denom": "'"$usdc_denom"'", "amount": "101000000"},
            "take_profit_price": "30",
            "user_address": "'"$user_address"'"
        }
    }'
}

# perpetual get position for address
function perpetual_get_positions_for_address() {
    printf "\n# Perpetual get position for address\n"
    query_contract "$ts_contract_address" '{
        "perpetual_get_positions_for_address": {
            "address": "'"$user_address"'",
            "pagination": null
        }
    }'
}

# get levarage lp params
function leveragelp_params() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_params": {}
    }'
}

# get leverage lp query positions
function leveragelp_query_positions() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_query_positions": {}
    }'
}

# get leveragelp_get_status
function leveragelp_query_positions_by_pool() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_query_positions_by_pool": {
           "amm_pool_id" : 2
        }
    }'
}

# get leveragelp get status
function leveragelp_get_status() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_get_status": {}
    }'
}

# get leveragelp query positions for address
function leveragelp_query_positions_for_address() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_query_positions_for_address": {
           "address" : "'"$user_address"'"
        }
    }'
}

# get leveragelp get whitelist
function leveragelp_get_whitelist() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_get_whitelist": {}
    }'
}

# get leveragelp is whitelisted
function leveragelp_is_whitelisted() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_is_whitelisted": {
           "address" : "'"$user_address"'"
        }
    }'
}

# get leveragelp pool
function leveragelp_pool() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_pool": {
            "index": 2
        }
    }'
}

# get leveragelp pools
function leveragelp_pools() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_pools": {}
    }'
}

# get leveragelp position
function leveragelp_position() {
    printf "\n# Get stable stake params\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_position": {
            "address" : "'"$user_address"'",
            "id": 2
        }
    }'
}

# leveragelp open estimation
function leveragelp_open_estimation() {
    printf "\n# Leveragelp open estimation\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_open_est": {
            "collateral_asset": "'$usdc_denom'",
            "collateral_amount": "1000000",
            "amm_pool_id": 2,
            "leverage": "5.0",
        }
    }'
}

# leveragelp close estimation
function leveragelp_close_estimation() {
    printf "\n# leveragelp close estimation\n"
    query_contract "$ts_contract_address" '{
        "leveragelp_close_est": {
            "owner" : "'"$user_address"'",
            "id": 2,
            "lp_amount": "1000000"
        }
    }'
}


function ts_params() {
    printf "\n# Get TS Params\n"
    query_contract \
        "$ts_contract_address" \
        '{
        "get_params": {}
    }'
}

function number_of_pending_orders() {
    printf "\n# number_of_pending_orders\n"
    query_contract \
        "$ts_contract_address" \
        '{
        "number_of_pending_order": {}
    }'
}

function get_spot_order_states() {
    order_id=$1
    printf "\n# Get Spot Order State"
    query_contract "$ts_contract_address" '{
        "get_spot_order_states": {
             "order_id": '$order_id'
        }
    }'
}

function get_ts_stat() {
    printf "\n# Get Spot Order State"
    query_contract "$ts_contract_address" '{
        "get_stat": {}
    }'
}

function parameter_params() {
    printf "\n# Get Parameter Params"
query_contract "$ts_contract_address" '{
        "parameter_params": {}
    }'
}

# function(s) to run based on the provided argument
case "$2" in
"swap_estimation_by_denom_elys_usdc_elys")
    swap_estimation_by_denom 1000000 uelys $usdc_denom uelys
    ;;
"swap_estimation_by_denom_elys_elys_usdc")
    swap_estimation_by_denom 1000000 uelys uelys $usdc_denom
    ;;
"swap_estimation_by_denom_usdc_usdc_usdc")
    swap_estimation_by_denom 1000000 $usdc_denom $usdc_denom $usdc_denom
    ;;
"swap_estimation_by_denom_usdc_usdc_atom")
    swap_estimation_by_denom 1000000 $usdc_denom $usdc_denom $atom_denom
    ;;
"swap_estimation_by_denom_atom_atom_usdc")
    swap_estimation_by_denom 1000000 $atom_denom $atom_denom $usdc_denom
    ;;
"swap_estimation_by_denom_usdc_usdc_atom_with_large_amount")
    swap_estimation_by_denom 112234000000 $usdc_denom $usdc_denom $atom_denom
    ;;
"swap_estimation_by_denom_atom_atom_usdc_with_small_amount")
    swap_estimation_by_denom 213565 $atom_denom $atom_denom $usdc_denom
    ;;
"swap_estimation_by_denom_osmo_usdc")
    swap_estimation_by_denom 1000000000 $osmo_denom $osmo_denom $usdc_denom
    ;;
"all_prices")
    all_prices
    ;;
"asset_info")
    asset_info
    ;;
"spot_order")
    spot_order $3
    ;;
"all_spot_orders")
    all_spot_orders
    ;;
"spot_orders_stop_loss")
    spot_orders stop_loss
    ;;
"spot_orders_limit_sell")
    spot_orders limit_sell
    ;;
"spot_orders_limit_buy")
    spot_orders limit_buy
    ;;
"spot_orders_market_buy")
    spot_orders market_buy
    ;;
"perpetual_position")
    perpetual_position
    ;;
"perpetual_order")
    perpetual_order
    ;;
"perpetual_orders_stop_loss")
    perpetual_orders stop_loss
    ;;
"perpetual_orders_limit_open")
    perpetual_orders limit_open
    ;;
"perpetual_orders_limit_close")
    perpetual_orders limit_close
    ;;
"perpetual_orders_market_open")
    perpetual_orders market_open
    ;;
"perpetual_orders_market_close")
    perpetual_orders market_close
    ;;
"perpetual_open_estimation")
    perpetual_open_estimation
    ;;
"perpetual_get_positions_for_address")
    perpetual_get_positions_for_address
    ;;
"leveragelp_params")
    leveragelp_params
    ;;
"leveragelp_query_positions")
    leveragelp_query_positions
    ;;
"leveragelp_query_positions_by_pool")
    leveragelp_query_positions_by_pool
    ;;
"leveragelp_get_status")
    leveragelp_get_status
    ;;
"leveragelp_query_positions_for_address")
    leveragelp_query_positions_for_address
    ;;
"leveragelp_get_whitelist")
    leveragelp_get_whitelist
    ;;
"leveragelp_is_whitelisted")
    leveragelp_is_whitelisted
    ;;
"leveragelp_pool")
    leveragelp_pool
    ;;
"leveragelp_pools")
    leveragelp_pools
    ;;
"leveragelp_position")
    leveragelp_position
    ;;
"leveragelp_open_estimation")
    leveragelp_open_estimation
    ;;
"leveragelp_close_estimation")
    leveragelp_close_estimation
    ;;
"ts_params")
    ts_params
    ;;
"number_of_pending_orders")
    number_of_pending_orders
    ;;
"get_spot_order_states")
    get_spot_order_states $3
    ;;
"get_ts_stat")
    get_ts_stat
    ;;
"parameter_params")
    parameter_params
    ;;
*)
    # Default case: run all functions
    ts_params
    perpetual_assets
    swap_estimation_by_denom 1000000 uelys $usdc_denom uelys
    swap_estimation_by_denom 1000000 $usdc_denom $usdc_denom $usdc_denom
    swap_estimation_by_denom 1000000 $usdc_denom $usdc_denom $atom_denom
    swap_estimation_by_denom 1000000 $atom_denom $atom_denom $usdc_denom
    swap_estimation_by_denom 112234000000 $usdc_denom $usdc_denom $atom_denom
    swap_estimation_by_denom 213565 $atom_denom $atom_denom $usdc_denom
    all_prices
    asset_info
    spot_order
    all_spot_orders
    spot_orders stop_loss
    spot_orders limit_sell
    spot_orders limit_buy
    spot_orders market_buy
    perpetual_position
    perpetual_order
    perpetual_orders stop_loss
    perpetual_orders limit_open
    perpetual_orders limit_close
    perpetual_orders market_open
    perpetual_orders market_close
    perpetual_open_estimation
    perpetual_get_positions_for_address
    ;;
esac
