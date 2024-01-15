#!/bin/bash

# This command is used to automate the store and init process of
# smart contracts. Examples:
# `sh scripts/migrate.sh ./artifacts/financial_snapshot_contract.wasm`
# `sh scripts/migrate.sh ./artifacts/account_history_contract.wasm '{"limit": 10, "value_denom": "uusdc", "expiration": {"at_time": "604800000000000"}}'`

init_contract() {
    local contract_wasm_path="$1"
    # Optional object parameter with a default value of '{}'
    local instantiate_msg="$2"

    # Check if $2 is provided, otherwise initialize to '{}'
    if [ -z "$instantiate_msg" ]; then
        instantiate_msg='{}'
    fi

    # Tries to store the contract and confirms the transaction
    echo "Storing the contract..."
    echo "y" | elysd tx wasm store "$1" --from cw --keyring-backend test --chain-id elystestnet-1 --gas auto --gas-adjustment=1.3 --fees 100000uelys -b sync

    # Wait for a few seconds to allow for synchronization
    sleep 1

    # Parse the JSON output to extract the latest code information
    codes=$(elysd q wasm list-code)
    code_id=$(echo "$codes" | awk -F'code_id: "|"' '/code_id/{latest=$2} END{print latest}')

    # Wait for a few seconds before initiating the contract
    sleep 1

    # Init the contract
    echo "Initializing the contract..."
    elysd tx wasm init $code_id "$instantiate_msg" --from cw --keyring-backend test --chain-id elystestnet-1 --gas auto --gas-adjustment=1.3 --fees 100000uelys -b sync -y --admin cw --label contract

    # Wait for a few seconds before initiating the contract
    sleep 1

    # Get contract address based on code ID
    contract_address=$(elysd q wasm contracts $code_id | awk '/^contracts:/{getline; print $2}')

    # Print the contract address
    echo "Code ID: $code_id"
    echo "Contract Address: $contract_address"
    echo "Contract sucessfully initialized"
}

init_contract "$1" "$2"