#!/bin/bash

# helper functions
extract_txhash() { awk -F 'txhash: ' '/txhash:/{print $2; exit}'; }
extract_code_id() { awk -F 'key: code_id|value: ' '/key: code_id/ { getline; gsub(/"/, "", $2); print $2; exit }'; }
extract_contract_address() { awk -F 'key: _contract_address|value: ' '/key: _contract_address/ { getline; gsub(/"/, "", $2); print $2; exit }'; }

# Function to check if a command exists
command_exists() {
    type "$1" &> /dev/null
}

# check if CI variable is set
if [ -n "$CI" ]; then
    # set environment variables
    NODE=https://rpc.testnet.elys.network:443
    NAME=contract-initiator

    # contract addresses
    FS_CONTRACT_ADDRESS=elys1g2xwx805epc897rwyrykskjque07yxfmc4qq2p4ef5dwd6znl30qnxje76
    TS_CONTRACT_ADDRESS=elys1m3hduhk4uzxn8mxuvpz02ysndxfwgy5mq60h4c34qqn67xud584qeee3m4
    AH_CONTRACT_ADDRESS=elys1s37xz7tzrru2cpl96juu9lfqrsd4jh73j9slyv440q5vttx2uyesetjpne

    # set elysd path
    ELYSD=/tmp/elysd
    URL=https://github.com/elys-network/elys/releases/download/v0.26.0/elysd-v0.26.0-linux-amd64

    # download elysd and binary to path
    wget $URL -O $ELYSD
    chmod +x $ELYSD
    export PATH=/tmp:$PATH
else
    # set environment variables
    NODE=tcp://localhost:26657
    NAME=validator
fi

# set elysd config
elysd config keyring-backend test
elysd config node $NODE
elysd config chain-id elystestnet-1
elysd config broadcast-mode sync

# environment variables
OPTIONS="--from $NAME --gas auto --gas-adjustment=1.3 --fees 400000uelys -y"

# Ensure elysd is installed
if ! command_exists elysd; then
    echo "elysd is not installed. Please install elysd to run this script."
    exit 1
fi

# recover key
if [ -n "$CI" ]; then
    if [ -n "$1" ]; then
        echo $1 | elysd keys add $NAME --recover --keyring-backend test
    fi
fi

# Ensure that the account has been set using elysd keys show command
if ! elysd keys show $NAME &> /dev/null; then
    echo "$NAME account has not been set. Please set the $NAME account using elysd keys show command."
    exit 1
fi

# store and init/migrate financial snapshot contract
txhash=$(elysd tx wasm store $OPTIONS artifacts/financial_snapshot_contract.wasm | extract_txhash)
sleep 10
codeid=$(elysd q tx $txhash --node $NODE | extract_code_id)
echo "fs code id: $codeid"
if [ -n "$FS_CONTRACT_ADDRESS" ]; then
    txhash=$(elysd tx wasm migrate $OPTIONS $FS_CONTRACT_ADDRESS $codeid '{}' | extract_txhash)
else
    txhash=$(elysd tx wasm init $OPTIONS --label "fs" --admin $NAME $codeid '{}' | extract_txhash)
fi
sleep 10
export fs_contract_address=$(elysd q tx $txhash --node $NODE | extract_contract_address)
echo "fs_contract_address: $fs_contract_address"

# store and init/migrate trade shield contract
txhash=$(elysd tx wasm store $OPTIONS artifacts/trade_shield_contract.wasm | extract_txhash)
sleep 10
codeid=$(elysd q tx $txhash --node $NODE | extract_code_id)
echo "ts code id: $codeid"
if [ -n "$TS_CONTRACT_ADDRESS" ]; then
    txhash=$(elysd tx wasm migrate $OPTIONS $TS_CONTRACT_ADDRESS $codeid '{
        "account_history_address": "'"$AH_CONTRACT_ADDRESS"'"
    }' | extract_txhash)
else
    # set localnet AH deterministic address as param
    txhash=$(elysd tx wasm init $OPTIONS --label "ts" --admin $NAME $codeid '{
        "account_history_address": "elys17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgs98tvuy"
    }' | extract_txhash)
fi
sleep 10
export ts_contract_address=$(elysd q tx $txhash --node $NODE | extract_contract_address)
echo "ts_contract_address: $ts_contract_address"

# store and init/migrate account history contract
txhash=$(elysd tx wasm store artifacts/account_history_contract.wasm $OPTIONS | extract_txhash)
sleep 10
codeid=$(elysd q tx $txhash --node $NODE | extract_code_id)
echo "ah code id: $codeid"
if [ -n "$AH_CONTRACT_ADDRESS" ]; then
    txhash=$(elysd tx wasm migrate $OPTIONS $AH_CONTRACT_ADDRESS $codeid '{
        "trade_shield_address": "'"$TS_CONTRACT_ADDRESS"'"
    }' | extract_txhash)
else
    txhash=$(elysd tx wasm init $OPTIONS --label "ah" --admin $NAME $codeid '{
        "limit": 300,
        "expiration": {
            "at_time": "604800000000000"
        },
        "trade_shield_address": "'"$ts_contract_address"'"
    }' | extract_txhash)
fi
sleep 10
ah_contract_address=$(elysd q tx $txhash --node $NODE | extract_contract_address)
echo "ah_contract_address: $ah_contract_address"

# print environment variables to set
printf "\nset those environment variables to use the contracts:\n\n"
printf "export NODE=%s\n" "$NODE"
printf "export NAME=%s\n" "$NAME"
printf "export FS_CONTRACT_ADDRESS=%s\n" "$fs_contract_address"
printf "export TS_CONTRACT_ADDRESS=%s\n" "$ts_contract_address"
printf "export AH_CONTRACT_ADDRESS=%s\n" "$ah_contract_address"