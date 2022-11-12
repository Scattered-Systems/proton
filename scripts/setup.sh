#!/usr/bin/env bash
npm install -g npm@8.19.2
rustup default nightly
rustup component add clippy rustfmt --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install create-tauri-app wasm-bindgen-cli wasm-pack
cargo install --locked trunk
sudo apt -y update && sudo apt -y upgrade && sudo apt -y autoremove
