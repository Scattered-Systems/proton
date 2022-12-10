FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    libdbus-1-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf \
    protobuf-compiler

FROM builder-base as environment

RUN rustup default nightly \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

RUN cargo install trunk wasm-bindgen-cli tauri-cli

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD .. /app
WORKDIR /app

COPY .. .
RUN cargo tauri build