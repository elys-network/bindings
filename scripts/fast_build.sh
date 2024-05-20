#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"

# Iterates over all example projects, builds and tests them.
# This script is for development purposes only. In the CI, each example project
# is configured separately.
BASE_DIR=$(pwd)

mkdir -p artifacts

rm -rf ./target/wasm32-unknown-unknown/

export RUST_BACKTRACE=1

for example in ./contracts/*/; do
  echo "Building and testing $example ..."
  (
    cd "$example"
    cargo wasm --locked
    converted=$(echo "$example" | sed 's/-/_/g')
    filename=$(basename "$converted")
    echo $BASE_DIR
    cp -f $BASE_DIR/target/wasm32-unknown-unknown/release/$filename.wasm  $BASE_DIR/artifacts/
  )
done

(
  cd $BASE_DIR/artifacts
  # create hashes
  shasum -- *.wasm | tee checksums.txt
)