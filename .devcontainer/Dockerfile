FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN apt-get install -y \
    curl \
    libssl-dev \
    protobuf-compiler

RUN rustup install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    rustup default nightly

RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --workspace && npm install && npm run build
