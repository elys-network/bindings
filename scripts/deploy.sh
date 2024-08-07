#!/bin/bash

# helper functions
extract_txhash() { awk -F 'txhash: ' '/txhash:/{print $2; exit}'; }
extract_code_id() { awk -F 'key: code_id|value: ' '/key: code_id/ { getline; gsub(/"/, "", $2); print $2; exit }'; }
extract_contract_address() { awk -F 'key: _contract_address|value: ' '/key: _contract_address/ { getline; gsub(/"/, "", $2); print $2; exit }'; }
extract_account_number() { awk -F 'account_number: ' '/account_number:/ { gsub(/"/, "", $2); print $2 + 0; exit; }'; }
extract_sequence() { awk -F 'sequence: ' '/sequence:/ { gsub(/"/, "", $2); print $2 + 0; exit; }'; }

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
    TS_CONTRACT_ADDRESS=elys1m3hduhk4uzxn8mxuvpz02ysndxfwgy5mq60h4c34qqn67xud584qeee3m4

    # set elysd path
    ELYSD=/tmp/elysd
    URL=https://github.com/elys-network/elys/releases/download/v0.39.0/elysd-v0.39.0-linux-amd64

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

user_address=$(elysd keys show $NAME -a)

# get account and sequence number
account_number=$(elysd q account $user_address --node $NODE | extract_account_number)
sequence=$(elysd q account $user_address --node $NODE | extract_sequence)
echo "account_number: $account_number"
echo "sequence: $sequence"

# environment variables
OPTIONS="--from $NAME --gas auto --gas-adjustment 1.3 --fees 300000uelys -y --account-number $account_number -b async --log_level trace --trace"

wait_for_tx() {
    local txhash=$1
    # loop until query tx cli does not fail
    while ! elysd q tx $txhash --node "$NODE" &> /dev/null; do
        echo "Waiting for the transaction $txhash to be included in a block..."
        sleep 0.5
    done
}

# store and init/migrate trade shield contract
txhash=$(elysd tx wasm store $OPTIONS --sequence $(($sequence + 3)) artifacts/trade_shield_contract.wasm | extract_txhash)
echo "ts store txhash: $txhash"
wait_for_tx $txhash
codeid=$(elysd q tx $txhash --node $NODE | extract_code_id)
echo "ts code id: $codeid"
if [ -n "$TS_CONTRACT_ADDRESS" ]; then
    txhash=$(elysd tx wasm migrate $OPTIONS --sequence $(($sequence + 4)) $TS_CONTRACT_ADDRESS $codeid '{}' | extract_txhash)
    echo "ts migrate txhash: $txhash"
else
    txhash=$(elysd tx wasm init $OPTIONS --sequence $(($sequence + 4)) --label "ts" --admin $NAME $codeid '{}' | extract_txhash)
    echo "ts init txhash: $txhash"
fi
wait_for_tx $txhash
export ts_contract_address=$(elysd q tx $txhash --node $NODE | extract_contract_address)
echo "ts_contract_address: $ts_contract_address"

# print environment variables to set
printf "\nset those environment variables to use the contracts:\n\n"
printf "export NODE=%s\n" "$NODE"
printf "export NAME=%s\n" "$NAME"
printf "export TS_CONTRACT_ADDRESS=%s\n" "$ts_contract_address"