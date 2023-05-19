FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release -v --workspace

FROM debian:buster-slim as runner-base

RUN apt-get update -y && apt-get upgrade -y 

RUN apt-get install -y libssl-dev protobuf-compiler

FROM runner-base

ENV RUST_LOG="info" \
    SERVER_PORT=8080 

COPY --chown=55 .config /config
VOLUME ["/config"]

COPY --from=builder /app/target/release/proton-api /bin/proton-api

EXPOSE ${SERVER_PORT}
EXPOSE 6379

CMD [ "proton-api" ]
