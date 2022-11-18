#!/usr/bin/env bash
npm install -g npm@9.1.1
rustup default nightly
rustup component add clippy rustfmt --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
sudo apt -y update && sudo apt -y upgrade && sudo apt -y autoremove
sudo apt install -y protobuf-compiler
