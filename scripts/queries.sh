#!/bin/bash

# check if first arg missing
if [ -z "$1" ]
  then
    echo "No argument supplied"
    exit 1
fi

# environment variables
NODE=https://rpc.testnet.elys.network:443

# contract addresses
AH_CONTRACT_ADDRESS=elys1s37xz7tzrru2cpl96juu9lfqrsd4jh73j9slyv440q5vttx2uyesetjpne
FS_CONTRACT_ADDRESS=elys1g2xwx805epc897rwyrykskjque07yxfmc4qq2p4ef5dwd6znl30qnxje76
TS_CONTRACT_ADDRESS=elys1m3hduhk4uzxn8mxuvpz02ysndxfwgy5mq60h4c34qqn67xud584qeee3m4

# print contract addresses
echo "# AH contract address $AH_CONTRACT_ADDRESS"
echo "# FS contract address $FS_CONTRACT_ADDRESS"
echo "# TS contract address $TS_CONTRACT_ADDRESS"

OPTIONS="--output json --node $NODE"

# set first arg as address
USER_ADDRESS=$1

# print user address
echo
echo "# User address $USER_ADDRESS"

# get total balance
echo
echo "# Total balance"
CMD="elysd q $OPTIONS wasm contract-state smart $AH_CONTRACT_ADDRESS '
{
    \"get_pod_total_balance\": {
        \"user_address\": \"'$USER_ADDRESS'\"
    }
}
' | jq"
echo "$ $CMD"
eval $CMD

# get portfolio balance
echo
echo "# Portfolio balance"
CMD="elysd q $OPTIONS wasm contract-state smart $AH_CONTRACT_ADDRESS '
{
    \"get_pod_portfolio\": {
        \"user_address\": \"'$USER_ADDRESS'\"
    }
}
' | jq"
echo "$ $CMD"
eval $CMD

# get pod rewards
echo
echo "# Pod rewards"
CMD="elysd q $OPTIONS wasm contract-state smart $FS_CONTRACT_ADDRESS '
{
    \"get_pod_rewards\": {
        \"address\": \"'$USER_ADDRESS'\"
    }
}
' | jq"
echo "$ $CMD"
eval $CMD

# get liquid assets
echo
echo "# Liquid assets"
CMD="elysd q $OPTIONS wasm contract-state smart $AH_CONTRACT_ADDRESS '
{
    \"get_liquid_assets\": {
        \"user_address\": \"'$USER_ADDRESS'\"
    }
}
' | jq"
echo "$ $CMD"
eval $CMD

# get staked assets
echo
echo "# Staked assets"
CMD="elysd q $OPTIONS wasm contract-state smart $AH_CONTRACT_ADDRESS '
{
    \"get_staked_assets\": {
        \"user_address\": \"'$USER_ADDRESS'\"
    }
}
' | jq"
echo "$ $CMD"
eval $CMD