FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y protobuf-compiler

RUN rustup default nightly && \
    rustup target add wasm32-wasi wasm32-unknown-unknown --toolchain nightly

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster-slim as runner-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y protobuf-compiler

RUN mkdir /config
VOLUME /config

FROM runner-base as runner

ENV SERVER_PORT=9000

EXPOSE 80
EXPOSE ${SERVER_PORT}

COPY Proton.toml /config/Proton.toml
COPY --from=builder /app/target/release/proton /bin/proton

ENTRYPOINT [ "proton" ]

CMD [ "system", "on" ]
