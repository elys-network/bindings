#!/bin/bash

# set -x
extract_txhash() { awk -F 'txhash: ' '/txhash:/{print $2; exit}'; }
extract_code_id() { awk -F 'key: code_id|value: ' '/key: code_id/ { getline; gsub(/"/, "", $2); print $2; exit }'; }
extract_contract_address() { awk -F 'key: _contract_address|value: ' '/key: _contract_address/ { getline; gsub(/"/, "", $2); print $2; exit }'; }

OPTIONS="--from validator --gas auto --gas-adjustment=1.3 --fees 100000uelys -b sync -y --keyring-backend=test --chain-id=elystestnet-1"

docker run --rm -v "$(pwd)":/code \
          --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
          --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
          cosmwasm/workspace-optimizer:0.14.0

# store and init trade shield contract
txhash=$(elysd tx wasm store artifacts/trade_shield_contract.wasm $OPTIONS | extract_txhash)
sleep 10
codeid=$(elysd q tx $txhash | extract_code_id)
txhash=$(elysd tx wasm init $codeid '{}' $OPTIONS  --label "Contract" --admin validator | extract_txhash)
sleep 10
addr=$(elysd q tx $txhash | extract_contract_address)

echo tradeshield : $addr


# store and init account history contract
txhash=$(elysd tx wasm store artifacts/account_history_contract.wasm $OPTIONS | extract_txhash)
sleep 10
codeid=$(elysd q tx $txhash | extract_code_id)
msg=$(echo '{"limit" : 300, "expiration": {"at_time":"604800000000000"}, "trade_shield_address" :"'$addr'"}')
txhash=$(elysd tx wasm init $codeid "$msg" $OPTIONS --label "Contract" --admin validator | extract_txhash)
sleep 10
addr=$(elysd q tx $txhash | extract_contract_address)
echo history : $addr
elysd tx wasm exec $addr '{}' --from validator --gas-prices 0.25uelys --gas auto --gas-adjustment 1.3 -b sync -y  --keyring-backend=test --chain-id=elystestnet-1
elysd q wasm contract-state smart $addr '{"all" : {}}'
# elysd q wasm contract-state smart $addr2 '{"get_liquid_assets" : {"user_address" : "WRITE THE USER ADDRESS"}}'