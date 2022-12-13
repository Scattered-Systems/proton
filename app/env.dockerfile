FROM node:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as langspace

RUN apt-get install -y \
    clang \
    cmake \
    git \
    llvm \
    wget
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM langspace

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

RUN npm install -g wasm-pack
