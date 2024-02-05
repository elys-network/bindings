#!/bin/bash

# This command is used to automate the store and init process of
# smart contracts. Examples:

# Without params: sh scripts/init_contract.sh ./artifacts/financial_snapshot_contract.wasm
# With init params: sh scripts/init_contract.sh ./artifacts/account_history_contract.wasm '{"limit": 10, "expiration": {"at_time": "604800000000000"}}'
# Common extra aguments: sh scripts/init_contract.sh ./artifacts/financial_snapshot_contract.wasm {} '--node test.com'

init_contract() {
    local contract_wasm_path="$1"
    # Optional object parameter with a default value of '{}'
    local instantiate_msg="$2"

    # shared additional params to be sent
    local additional_params="$3"
    shift 3  # Shift to remove the first three parameters

    # Check if $2 is provided, otherwise initialize to '{}'
    if [ -z "$instantiate_msg" ]; then
        instantiate_msg='{}'
    fi

    local store_command=("elysd" "tx" "wasm" "store" "$contract_wasm_path" "--from" "cw" "--keyring-backend" "test" "--chain-id" "elystestnet-1" "--gas" "auto" "--gas-adjustment=1.3" "--fees" "100000uelys" "-b" "sync" "-y")
    # Append additional params if provided
    if [ -n "$additional_params" ]; then
        store_command+=($additional_params)
    fi

    # Tries to store the contract and confirms the transaction
    echo "Storing the contract..."
    echo "y" | "${store_command[@]}"
 
    # Wait for a few seconds to allow for synchronization
    sleep 1

    # Parse the JSON output to extract the latest code information
    codes=$(elysd q wasm list-code)
    code_id=$(echo "$codes" | awk -F'code_id: "|"' '/code_id/{latest=$2} END{print latest}')

    # Wait for a few seconds before initiating the contract
    sleep 1

    local init_command=("elysd" "tx" "wasm" "init" "$code_id" "$instantiate_msg" "--from" "cw" "--keyring-backend" "test" "--chain-id" "elystestnet-1" "--gas" "auto" "--gas-adjustment=1.3" "--fees" "100000uelys" "-b" "sync" "-y" "--admin" "cw" "--label" "contract") 
    if [ -n "$additional_params" ]; then
        init_command+=($additional_params)
    fi

    # Init the contract
    echo "Initializing the contract..."
    "${init_command[@]}"

    # Wait for a few seconds before initiating the contract
    sleep 1

    # Get contract address based on code ID
    contract_address=$(elysd q wasm contracts $code_id | awk '/^contracts:/{getline; print $2}')

    # Print the contract address
    echo "Code ID: $code_id"
    echo "Contract Address: $contract_address"
    echo "Contract sucessfully initialized"
}

init_contract "$1" "$2" "$3"