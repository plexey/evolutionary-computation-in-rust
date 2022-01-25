#!/bin/bash

cd "$(dirname "$0")"

mkdir ../data
mkdir ../data/images

cd ../imaging
yarn install
yarn build

cd ../algo
cargo build --release
echo "Setup complete!"