#!/bin/bash

# Configuration
N=$1                            # Number of accounts to create
OUTPUT=/tmp/accounts_list.txt

# helper functions
extract_txhash() { awk -F 'txhash: ' '/txhash:/{print $2; exit}'; }

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

# Check if the number of accounts to create is provided
if [ -z "$N" ]; then
    echo "Please specify the number of accounts to create."
    exit 1
fi

# Environment variables
if [ -z "$NODE" ]; then
    NODE="https://rpc.testnet.elys.network:443"
fi
if [ -z "$NAME" ]; then
    NAME="contract-initiator"
fi

printf "# Node: %s\n" "$NODE"
printf "# Name: %s\n" "$NAME"

# Ensure that the account has been set using elysd keys show command
if ! elysd keys show $NAME &> /dev/null; then
    echo "$NAME account has not been set. Please set the $NAME account using elysd keys show command."
    exit 1
fi

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
usdc_denom="ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
atom_denom="ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4"

# Print denoms
printf "\n# USDC denom: %s\n" "$usdc_denom"
printf "# ATOM denom: %s\n" "$atom_denom"

# User address
user_address=$(elysd keys show $NAME -a)

# Print user address
printf "\n# User address: %s\n" "$user_address"

wait_for_tx() {
    local txhash=$1
    # loop until query tx cli does not fail
    while ! elysd q tx $txhash --node "$NODE" &> /dev/null; do
        echo "Waiting for the transaction $txhash to be included in a block..."
        sleep 0.5
    done
}

# Step 1: Generate N accounts and store them in a list
echo "Generating $N accounts..."
for i in $(seq 1 $N); do
    ACCOUNT_NAME="Account_$i"
    OUTPUT=$(elysd keys add $ACCOUNT_NAME --output json)
    ADDRESS=$(echo "$OUTPUT" | jq -r '.address')
    echo "$ACCOUNT_NAME $ADDRESS" >> $OUTPUT
done

# Step 2: Send uelys to these accounts
echo "Sending uelys to generated accounts..."
while IFS= read -r line; do
    ACCOUNT_NAME=$(echo $line | cut -d ' ' -f 1)
    ACCOUNT_ADDRESS=$(echo $line | cut -d ' ' -f 2)
    txhash=$(elysd tx bank send $user_address $ACCOUNT_ADDRESS 1000000000uelys --from=$NAME --fees=100000uelys -y | extract_txhash)
    wait_for_tx $txhash
done < $OUTPUT

# Step 3: Broadcast transactions in parallel
echo "Broadcasting transactions in parallel..."
while IFS= read -r line; do
    ACCOUNT_ADDRESS=$(echo $line | cut -d ' ' -f 2)
    (
        elysd tx wasm exec --from "validator" --keyring-backend test --node $NODE --chain-id elystestnet-1 --gas auto --gas-adjustment=1.3 --fees 100000uelys -b sync -y --amount 10000000uelys "$ts_contract_address" '{
            "create_spot_order": {
                "order_type": "market_buy",
                "order_target_denom": "'$atom_denom'",
                "order_source_denom": "uelys"
            }
        }'
    ) &
done < $OUTPUT
wait

echo "All transactions have been broadcasted."
