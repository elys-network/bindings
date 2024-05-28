#!/bin/bash

# Check if the CI environment variable is set to true
if [ "${CI:-}" = "true" ]; then
    echo "CI is true, running the block of commands..."
    export VERSION=$(git describe --tags --match 'v*' --abbrev=0 | sed 's/^v//')
    sed -i '' "s/^version = .*/version = \"$VERSION\"/" contracts/account-history-contract/Cargo.toml
    sed -i '' "s/^version = .*/version = \"$VERSION\"/" contracts/trade-shield-contract/Cargo.toml
    sed -i '' "s/^version = .*/version = \"$VERSION\"/" contracts/financial-snapshot-contract/Cargo.toml
fi

docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/workspace-optimizer:0.14.0