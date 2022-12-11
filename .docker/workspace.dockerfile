FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

FROM builder-base as workspace

ENV CARGO_TERM_COLOR=always

ADD . /app
WORKDIR /app

COPY . .

FROM workspace

