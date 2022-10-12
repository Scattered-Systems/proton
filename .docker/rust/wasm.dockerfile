FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y 
RUN rustup update && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

FROM builder-base as builder

RUN cargo install --locked trunk \
    cargo install wasm-bindgen-cli