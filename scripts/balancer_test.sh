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

# Ensure curl is installed
if ! command_exists curl; then
    echo "curl is not installed. Please install curl to run this script."
    exit 1
fi

# Define the API URL
api_url="https://api-testnet.elys.network/balancer/"

# Use curl to fetch the data from the API and jq to parse it
# Filtering hosts that are available and not disabled
hosts=$(curl -s "$api_url" | jq -r '.[] | select(.available == true and .disabled == false) | .host')

# Check if hosts are retrieved successfully
if [ -z "$hosts" ]; then
    echo "Failed to retrieve hosts or no suitable hosts available."
    exit 1
fi

# Loop through each host and execute the commands
for host in $hosts; do
    # Check if the host URL contains a port number
    if ! [[ "$host" =~ :[0-9]+ ]]; then
        host="${host}:443"
    fi

    echo "Executing commands on $host"

    export NODE="$host"
    ./scripts/messages.sh $@
done
