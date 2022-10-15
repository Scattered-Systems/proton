FROM rust:latest as builder-base

# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get update && apt-get install nodejs

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN rustup update && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly 

RUN cargo install --locked trunk && cargo install wasm-bindgen-cli

FROM builder-base as builder

ENV API_ROOT="http://localhost:8000"

ADD app /app
WORKDIR /proton

COPY app .

RUN trunk build

EXPOSE 8000

CMD [ "trunk", "serve" ]
