FROM scratch as cache

RUN mkdir config cache data workspace

VOLUME [ "/artifacts" ]
VOLUME [ "/cache"]
VOLUME [ "/config" ]
VOLUME [ "/data" ]

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
RUN cargo xtask compile

FROM cache as cache-build

COPY --from=builder /workspace/target/dist /dist

FROM builder as development

ENV PORT=8000

EXPOSE 80
EXPOSE ${PORT}

ENTRYPOINT [ "cargo", "xtask" ]
CMD [ "start" ]
