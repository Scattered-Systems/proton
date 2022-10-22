FROM rust:latest as builder-base

RUN rustup update && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly 

RUN cargo install --locked trunk && cargo install wasm-bindgen-cli

FROM builder-base as builder

ENV API_ROOT="http://localhost:8000"

ADD . /app
WORKDIR /app

COPY . .

RUN trunk build

EXPOSE 8080

CMD [ "trunk", "serve" ]
