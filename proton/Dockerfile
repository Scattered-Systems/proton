FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    apt-utils

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo install wasm-bindgen-cli

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM builder

RUN cargo test --all-features --release --verbose
