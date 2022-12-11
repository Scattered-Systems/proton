FROM scratch as cache

RUN mkdir config cache data workspace

VOLUME [ "/config" ]
VOLUME [ "/cache"]
VOLUME [ "/data" ]
VOLUME [ "/workspace" ]

FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

FROM builder-base as trunkrs-base

RUN cargo install trunk wasm-bindgen-cli

FROM trunkrs-base as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN trunk build --release

FROM builder as development

ENV PORT=8000

EXPOSE 80
EXPOSE ${PORT}

ENTRYPOINT [ "trunk" ]
CMD [ "serve", "--release" ]
