#!/bin/bash

# Configuration
OUTPUT_FILE="/tmp/accounts_list.txt"

# Helper functions
extract_txhash() { awk -F 'txhash: ' '/txhash:/{print $2; exit}'; }

command_exists() {
    type "$1" &> /dev/null
}

wait_for_tx() {
    local txhash=$1
    while ! elysd q tx $txhash --node "$NODE" &> /dev/null; do
        echo "Waiting for the transaction $txhash to be included in a block..."
        sleep 0.5
    done
}

# Ensure required commands are available
if ! command_exists jq || ! command_exists elysd; then
    echo "Ensure jq and elysd are installed to run this script."
    exit 1
fi

# Environment variables and defaults
NODE="${NODE:-https://rpc.testnet.elys.network:443}"
NAME="${NAME:-contract-initiator}"
TS_CONTRACT_ADDRESS="${TS_CONTRACT_ADDRESS:-elys1m3hduhk4uzxn8mxuvpz02ysndxfwgy5mq60h4c34qqn67xud584qeee3m4}"
USDC_DENOM="ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
ATOM_DENOM="ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4"

# Ensure that the account has been set using elysd keys show command
if ! elysd keys show $NAME &> /dev/null; then
    echo "$NAME account has not been set. Please set the $NAME account using elysd keys show command."
    exit 1
fi

# User address
user_address=$(elysd keys show $NAME -a)

# Function to generate accounts
generate_accounts() {
    N=$1

    # if $OUTPUT_FILE exists, throw an error
    if [ -f "$OUTPUT_FILE" ]; then
        echo "The file $OUTPUT_FILE already exists. Please remove it before running the script."
        exit 1
    fi

    # if the number of accounts to create is not provided, throw an error
    if [ -z "$N" ]; then
        echo "Please specify the number of accounts to create."
        exit 1
    fi

    echo "Generating $N accounts..."
    for i in $(seq 1 $N); do
        ACCOUNT_NAME="Account_$i"
        elysd keys delete $ACCOUNT_NAME -y -f > /dev/null 2>&1
        OUTPUT=$(elysd keys add $ACCOUNT_NAME --output json)
        ADDRESS=$(echo "$OUTPUT" | jq -r '.address')
        echo "$ACCOUNT_NAME $ADDRESS" >> $OUTPUT_FILE
    done
}

# Function to send uelys to accounts
send_uelys() {
    echo "Sending uelys to generated accounts..."

    while IFS= read -r line; do
        ACCOUNT_NAME=$(echo $line | cut -d ' ' -f 1)
        ACCOUNT_ADDRESS=$(echo $line | cut -d ' ' -f 2)
        txhash=$(elysd tx bank send $user_address $ACCOUNT_ADDRESS 1000000uelys --node $NODE --from=$NAME --fees=100000uelys -y | extract_txhash)
        wait_for_tx $txhash
        echo "Sent 1000000000uelys to $ACCOUNT_NAME ($ACCOUNT_ADDRESS)"
    done < $OUTPUT_FILE
    wait
}

# Function to broadcast transactions
broadcast_transactions() {
    echo "Broadcasting transactions in parallel..."
    while IFS= read -r line; do
        ACCOUNT_NAME=$(echo $line | cut -d ' ' -f 1)
        ACCOUNT_ADDRESS=$(echo $line | cut -d ' ' -f 2)
        elysd tx wasm exec --from=$ACCOUNT_NAME --node $NODE --gas auto --gas-adjustment=1.3 --fees 100000uelys -b async -y --amount 100000uelys "$TS_CONTRACT_ADDRESS" '{
            "create_spot_order": {
                "order_type": "market_buy",
                "order_target_denom": "'$ATOM_DENOM'",
                "order_source_denom": "uelys"
            }
        }' > /dev/null 2>&1
    done < $OUTPUT_FILE
    wait
    echo "All transactions have been broadcasted."
}

# Add command line options to trigger specific functions
COMMAND=$1

# function(s) to run based on the provided argument
case "$COMMAND" in
    "generate")
        generate_accounts $2
        ;;
    "send")
        send_uelys
        ;;
    "broadcast")
        broadcast_transactions
        ;;
    *)
        echo "Please provide a valid command: generate, send, or broadcast."
        ;;
esac