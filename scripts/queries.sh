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

# Get AH params
function ah_params() {
    printf "\n# AH Params\n"
    query_contract "$ah_contract_address" '{
        "params": {}
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

function staked_assets_no_user() {
    printf "\n# Staked assets\n"
    query_contract "$ah_contract_address" '{
        "get_staked_assets": {
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

# Get Asset Price
function get_asset_price() {
    asset=$1
    printf "\n# Get Asset Price\n"
    query_contract "$ah_contract_address" '{
        "get_asset_price" : {
            "asset": "'"$asset"'"
        }
    }'
}

# Get Asset Price From Denom In To Denom Out
function get_asset_price_from_denom_in_to_denom_out() {
    denom_in=$1
    denom_out=$2
    printf "\n# Get Asset Price From Denom In To Denom Outs\n"
    query_contract "$ah_contract_address" '{
        "get_asset_price_from_denom_in_to_denom_out": {
            "denom_in": "'"$denom_in"'",
            "denom_out": "'"$denom_out"'"
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
    denom=$1
    printf "\n# Get commitment staked balance of denom\n"
    query_contract "$ah_contract_address" '{
        "commitment_staked_balance_of_denom": {
            "address": "'"$user_address"'",
            "denom": "'"$denom"'"
        }
    }'
}

# get StableStakeBalanceOfBorrow
function get_stable_stake_balance_of_borrow() {
    printf "\n# Get stable stake balance of borrow\n"
    query_contract "$ah_contract_address" '{
        "stable_stake_balance_of_borrow": {}
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

# get AmmPriceByDenom
function get_amm_price_by_denom() {
    denom=$1
    printf "\n# Get amm price by denom\n"
    query_contract "$ah_contract_address" '{
        "amm_price_by_denom": {
            "token_in": {
                "amount": "1000000",
                "denom": "'"$denom"'"
            },
            "discount": "0"
        }
    }'
}

# get user snapshots
function get_user_snapshots() {
    printf "\n# Get user snapshots\n"
    query_contract "$ah_contract_address" '{
        "user_snapshots": {
            "user_address": "'"$user_address"'"
        }
    }'
}

# get user last snapshot
function get_user_last_snapshot() {
    printf "\n# Get user last snapshot\n"
    query_contract "$ah_contract_address" '{
        "last_snapshot": {
            "user_address": "'"$user_address"'"
        }
    }'
}

# get all snapshots
function get_all_snapshots() {
    printf "\n# Get all snapshots\n"
    query_contract "$ah_contract_address" '{
        "all": {}
    }'
}

# get stable stake params
function get_stable_stake_params() {
    printf "\n# Get stable stake params\n"
    query_contract "$ah_contract_address" '{
        "stable_stake_params": {}
    }'
}

# get liquidity pools
function get_liquidity_pools() {
    printf "\n# Get stable stake params\n"
    query_contract "$ah_contract_address" '{
        "get_liquidity_pools": {
            "filter_type": "filter_all"
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

# get EstakingRewards
function get_estaking_rewards() {
    printf "\n# Get estaking rewards\n"
    query_contract "$ah_contract_address" '{
        "get_estaking_rewards": {
             "address": "'"$user_address"'"
        }
    }'
}

function master_chef_params() {
    printf "\n# Get Masterchef Params\n"
    query_contract \
        "$ah_contract_address" \
        '{
        "get_masterchef_params": {}
    }'
}

# get Masterchef Pending Rewards
function get_masterchef_pending_rewards() {
    printf "\n# Get master chef pending rewards\n"
    query_contract "$ah_contract_address" '{
        "get_masterchef_pending_rewards": {
             "address": "'"$user_address"'"
        }
    }'
}

# get Masterchef Pending Rewards
function get_masterchef_stable_stake_apr() {
    denom=$1
    printf "\n# Get Masterchef stable stake apr\n"
    query_contract "$ah_contract_address" '{
        "get_masterchef_stable_stake_apr": {
             "denom": "'"$denom"'"
        }
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

function get_ts_stat {
    printf "\n# Get Spot Order State"
    query_contract "$ts_contract_address" '{
        "get_stat": {}
    }'
}

# function(s) to run based on the provided argument
case "$2" in
"ah_params")
    ah_params
    ;;
"membership_tier")
    membership_tier
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
"staked_assets_no_user")
    staked_assets_no_user
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
    get_commitment_staked_balance_of_denom $3
    ;;
"get_stable_stake_balance_of_borrow")
    get_stable_stake_balance_of_borrow
    ;;
"get_commitment_vesting_info")
    get_commitment_vesting_info
    ;;
"get_amm_price_by_denom")
    get_amm_price_by_denom $3
    ;;
"get_user_snapshots")
    get_user_snapshots
    ;;
"get_user_last_snapshot")
    get_user_last_snapshot
    ;;
"get_all_snapshots")
    get_all_snapshots
    ;;
"get_stable_stake_params")
    get_stable_stake_params
    ;;
"get_liquidity_pools")
    get_liquidity_pools
    ;;
"get_asset_price")
    get_asset_price $3
    ;;
"get_asset_price_from_denom_in_to_denom_out")
    get_asset_price_from_denom_in_to_denom_out $3 $4
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
"get_estaking_rewards")
    get_estaking_rewards
    ;;
"master_chef_params")
    master_chef_params
    ;;
"get_masterchef_pending_rewards")
    get_masterchef_pending_rewards
    ;;
"get_masterchef_stable_stake_apr")
    get_masterchef_stable_stake_apr $3
    ;;
"get_spot_order_states")
    get_spot_order_states $3
    ;;
"get_ts_stat")
    get_ts_stat
    ;;
*)
    # Default case: run all functions
    ah_params
    ts_params
    membership_tier
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
    perpetual_position
    perpetual_order
    perpetual_orders stop_loss
    perpetual_orders limit_open
    perpetual_orders limit_close
    perpetual_orders market_open
    perpetual_orders market_close
    perpetual_open_estimation
    perpetual_get_positions_for_address
    get_commitment_staked_positions
    get_commitment_unstaked_positions
    get_commitment_rewards_sub_bucket_balance_of_denom ueden 2
    get_commitment_staked_balance_of_denom $usdc_denom
    get_stable_stake_balance_of_borrow
    get_commitment_vesting_info
    get_amm_price_by_denom $usdc_denom
    get_amm_price_by_denom $elys_denom
    get_amm_price_by_denom $eden_denom
    get_stable_stake_params
    get_liquidity_pools
    ;;
esac
