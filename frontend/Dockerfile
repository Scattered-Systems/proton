FROM rust:latest as builder-base

# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get update && apt-get install nodejs

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

FROM builder-base as builder
ENV API_ROOT="http://localhost:8000"

RUN rustup update && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly 

WORKDIR /proton

COPY . .

RUN npm install

EXPOSE 8000

CMD [ "npm", "start" ]
