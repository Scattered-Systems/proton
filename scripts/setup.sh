#!/usr/bin/env bash
rustup default nightly
rustup component add clippy rustfmt --toolchain nightly
rustup target add wasm32-wasi wasm32-unknown-unknown --toolchain nightly
sudo apt -y update && sudo apt -y upgrade
sudo apt install -y protobuf-compiler
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | bash