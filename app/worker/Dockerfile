FROM rust as builder-base

RUN apt-get update -y && apt-get upgrade -y && \
    apt-get install -y \
    curl \
    protobuf-compiler \
    wget

RUN rustup update && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get update && apt-get install nodejs


RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && \
    npm install -g npm@8.19.2 wrangler@latest

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN wrangler

FROM builder as dev

EXPOSE 6284
EXPOSE 8787
EXPOSE 8976
EXPOSE 9229

CMD ["wrangler", "dev"]

