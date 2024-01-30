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
elys_denom="uelys"
eden_denom="ueden"
edenb_denom="uedenb"
usdc_denom="ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
atom_denom="ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4"

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

# Get AH params
function ah_params() {
    printf "\n# AH Params\n"
    query_contract "$ah_contract_address" '{
        "params": {}
    }'
}

# Get total balance
function total_balance() {
    printf "\n# Total balance\n"
    query_contract "$ah_contract_address" '{
        "get_total_balance": {
            "user_address": "'"$user_address"'"
        }
    }'
}

# Get membership tier
function membership_tier() {
    printf "\n# Membership tier\n"
    query_contract "$ah_contract_address" '{
        "get_membership_tier": {
            "user_address": "'"$user_address"'"
        }
    }'
}

# Get portfolio balance
function portfolio_balance() {
    printf "\n# Portfolio balance\n"
    query_contract "$ah_contract_address" '{
        "get_portfolio": {
            "user_address": "'"$user_address"'"
        }
    }'
}

# Get rewards
function rewards() {
    printf "\n# Rewards\n"
    query_contract "$ah_contract_address" '{
        "get_rewards": {
            "user_address": "'"$user_address"'"
        }
    }'
}

# Get liquid assets
function liquid_assets() {
    printf "\n# Liquid assets\n"
    query_contract "$ah_contract_address" '{
        "get_liquid_assets": {
            "user_address": "'"$user_address"'"
        }
    }'
}

# Get staked assets
function staked_assets() {
    printf "\n# Staked assets\n"
    query_contract "$ah_contract_address" '{
        "get_staked_assets": {
            "user_address": "'"$user_address"'"
        }
    }'
}

# Get perpetual assets
function perpetual_assets() {
    printf "\n# Perpertual assets\n"
    query_contract "$ah_contract_address" '{
        "get_perpetual_assets": {
            "user_address": "'"$user_address"'"
        }
    }'
}

# Get user value
function user_value() {
    printf "\n# User value\n"
    query_contract "$ah_contract_address" '{
        "user_value": {
            "user_address": "'"$user_address"'"
        }
    }'
}

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
    printf "\n# Spot order\n"
    query_contract "$ts_contract_address" '{
        "get_spot_order": {
            "order_id": 1
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

# Get margin position
function margin_position() {
    printf "\n# Get margin position\n"
    query_contract "$ts_contract_address" '{
        "get_margin_position": {
            "address": "'"$user_address"'",
            "id": 1
        }
    }'
}

# Get margin order
function margin_order() {
    printf "\n# Margin order\n"
    query_contract "$ts_contract_address" '{
        "get_margin_order": {
            "id": 1
        }
    }'
}

# Get margin orders
function margin_orders() {
    printf "\n# Get margin orders with $1\n"
    query_contract "$ts_contract_address" '{
        "get_margin_orders": {
            "pagination": null,
            "order_owner": "'"$user_address"'",
            "order_type": "'$1'",
            "order_status": null
        }
    }'
}

# Margin open estimation
function margin_open_estimation() {
    printf "\n# Margin open estimation\n"
    query_contract "$ts_contract_address" '{
        "margin_open_estimation": {
            "position": "long",
            "leverage": "5",
            "trading_asset": "'"$atom_denom"'",
            "collateral": {"denom": "'"$usdc_denom"'", "amount": "100000000"},
            "take_profit_price": "30",
            "user_address": "'"$user_address"'"
        }
    }'
}

# margin get position for address
function margin_get_positions_for_address() {
    printf "\n# Margin get position for address\n"
    query_contract "$ts_contract_address" '{
        "margin_get_positions_for_address": {
            "address": "'"$user_address"'",
            "pagination": null
        }
    }'
}

# get commitment staked positions
function get_commitment_staked_positions() {
    printf "\n# Get commitment staked positions\n"
    query_contract "$ah_contract_address" '{
        "commitment_staked_positions": {
            "delegator_address": "'"$user_address"'"
        }
    }'
}

# get CommitmentUnStakedPositions
function get_commitment_unstaked_positions() {
    printf "\n# Get commitment unstaked positions\n"
    query_contract "$ah_contract_address" '{
        "commitment_un_staked_positions": {
            "delegator_address": "'"$user_address"'"
        }
    }'
}

# get CommitmentRewardsSubBucketBalanceOfDenom
function get_commitment_rewards_sub_bucket_balance_of_denom() {
    denom=$1
    program=$2
    printf "\n# Get commitment rewards sub bucket balance of denom denom=$1 program=$2\n"
    query_contract "$ah_contract_address" '{
        "commitment_rewards_sub_bucket_balance_of_denom": {
            "address": "'"$user_address"'",
            "denom": "'"$denom"'",
            "program": '"$program"'
        }
    }'
}

# get CommitmentStakedBalanceOfDenom
function get_commitment_staked_balance_of_denom() {
    printf "\n# Get commitment staked balance of denom\n"
    query_contract "$ah_contract_address" '{
        "commitment_staked_balance_of_denom": {
            "address": "'"$user_address"'",
            "denom": "'"$usdc_denom"'"
        }
    }'
}

# get StableStakeBalanceOfBorrow
function get_stable_stake_balance_of_borrow() {
    printf "\n# Get stable stake balance of borrow\n"
    query_contract "$ah_contract_address" '{
        "stable_stake_balance_of_borrow": {
            "address": "'"$user_address"'"
        }
    }'
}

# get CommitmentVestingInfo
function get_commitment_vesting_info() {
    printf "\n# Get commitment vesting info\n"
    query_contract "$ah_contract_address" '{
        "commitment_vesting_info": {
            "address": "'"$user_address"'"
        }
    }'
}

# function(s) to run based on the provided argument
case "$2" in
    "ah_params")
        ah_params
        ;;
    "total_balance")
        total_balance
        ;;
    "membership_tier")
        membership_tier
        ;;
    "portfolio_balance")
        portfolio_balance
        ;;
    "rewards")
        rewards
        ;;
    "liquid_assets")
        liquid_assets
        ;;
    "staked_assets")
        staked_assets
        ;;
    "perpetual_assets")
        perpetual_assets
        ;;
    "user_value")
        user_value
        ;;
    "swap_estimation_by_denom_elys_usdc_elys")
        swap_estimation_by_denom 1000000 uelys $usdc_denom uelys
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
    "all_prices")
        all_prices
        ;;
    "asset_info")
        asset_info
        ;;
    "spot_order")
        spot_order
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
    "margin_position")
        margin_position
        ;;
    "margin_order")
        margin_order
        ;;
    "margin_orders_stop_loss")
        margin_orders stop_loss
        ;;
    "margin_orders_limit_open")
        margin_orders limit_open
        ;;
    "margin_orders_limit_close")
        margin_orders limit_close
        ;;
    "margin_orders_market_open")
        margin_orders market_open
        ;;
    "margin_orders_market_close")
        margin_orders market_close
        ;;
    "margin_open_estimation")
        margin_open_estimation
        ;;
    "margin_get_positions_for_address")
        margin_get_positions_for_address
        ;;
    "get_commitment_staked_positions")
        get_commitment_staked_positions
        ;;
    "get_commitment_unstaked_positions")
        get_commitment_unstaked_positions
        ;;
    "get_commitment_rewards_sub_bucket_balance_of_denom")
        get_commitment_rewards_sub_bucket_balance_of_denom $3 $4
        ;;
    "get_commitment_staked_balance_of_denom")
        get_commitment_staked_balance_of_denom
        ;;
    "get_stable_stake_balance_of_borrow")
        get_stable_stake_balance_of_borrow
        ;;
    "get_commitment_vesting_info")
        get_commitment_vesting_info
        ;;

    *)
        # Default case: run all functions
        ah_params
        total_balance
        membership_tier
        portfolio_balance
        rewards
        liquid_assets
        staked_assets
        perpetual_assets
        user_value
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
        margin_position
        margin_order
        margin_orders stop_loss
        margin_orders limit_open
        margin_orders limit_close
        margin_orders market_open
        margin_orders market_close
        margin_open_estimation
        margin_get_positions_for_address
        get_commitment_staked_positions
        get_commitment_unstaked_positions
        get_commitment_rewards_sub_bucket_balance_of_denom ueden 2
        get_commitment_staked_balance_of_denom
        get_stable_stake_balance_of_borrow
        get_commitment_vesting_info
        ;;
esac