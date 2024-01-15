#!/bin/bash

migrate_contract() {
    # Tries to store the contract and confirms the transaction
    echo "Storing the contract..."
    echo "y" | elysd tx wasm store artifacts/financial_snapshot_contract.wasm --from cw --keyring-backend test --chain-id elystestnet-1 --gas auto --gas-adjustment=1.3 --fees 100000uelys -b sync

    # Wait for a few seconds to allow for synchronization
    sleep 1

    # Parse the JSON output to extract the latest code information
    codes=$(elysd q wasm list-code)
    code_id=$(echo "$codes" | awk -F'code_id: "|"' '/code_id/{latest=$2} END{print latest}')

    # Wait for a few seconds before initiating the contract
    sleep 1

    # Init the contract
    echo "Initializing the contract..."
    elysd tx wasm init $code_id '{}' --from cw --keyring-backend test --chain-id elystestnet-1 --gas auto --gas-adjustment=1.3 --fees 100000uelys -b sync -y --admin cw --label contract

    # Wait for a few seconds before initiating the contract
    sleep 1

    # Get contract address based on code ID
    contract_address=$(elysd q wasm contracts $code_id | awk '/^contracts:/{getline; print $2}')

    # Migrate the contract
    echo "Migrating the contract..."
    elysd tx wasm migrate $contract_address $code_id '{}' --from cw --keyring-backend test --chain-id elystestnet-1 --gas auto --gas-adjustment=1.3 --fees 100000uelys -b sync -y

    # Print the contract address (optional)
    echo "Code ID: $code_id."
    echo "Contract Address: $contract_address."
    echo "Contract sucessfully migrated."
}

migrate_contract