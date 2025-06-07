#!/usr/bin/bash

cd ztools-cli && cargo build --release
cd ../target/release/ && sudo cp ztools /usr/bin/ztools
