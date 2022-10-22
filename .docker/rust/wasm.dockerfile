FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN rustup update && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

FROM builder-base as wasm-pack
# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get update && apt-get install nodejs

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

ENTRYPOINT [ "wasm-pack" ]

FROM builder-base as trunk

RUN cargo install --locked trunk \
    cargo install wasm-bindgen-cli

ENTRYPOINT [ "trunk" ]
