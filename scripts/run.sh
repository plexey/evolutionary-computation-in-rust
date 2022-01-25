#!/bin/bash

cd "$(dirname "$0")"

pwd 
cd ../algo
cargo build --release
cp target/release/algo .
time ./algo
rm ./algo


cd ../imaging


pwd
yarn build
time node ./dist/index.js