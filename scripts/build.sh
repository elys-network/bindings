#!/bin/bash

export VERSION=$(git describe --tags --match 'v*' --abbrev=8 | sed 's/-g/-/' | sed 's/-[0-9]*-/-/')

docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/workspace-optimizer:0.14.0 \
    -e VERSION=$VERSION