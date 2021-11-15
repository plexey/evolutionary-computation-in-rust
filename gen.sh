#!/bin/bash

cd algo
cargo build --release
cp target/release/algo .
time ./algo
rm ./algo

cd ../imaging
yarn build
time node ./dist/index.js