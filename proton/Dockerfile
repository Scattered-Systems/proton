FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    apt-utils \
    curl

RUN rustup default nightly && \
    rustup target add wasm32-wasi wasm32-unknown-unknown

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM builder

RUN cargo test --all-features --release --verbose
