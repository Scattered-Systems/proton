FROM scsys/rust:debian-lts as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM builder as publisher

ENV CRATE=curiosity \
    CARGO_REGISTRY_TOKEN=""

RUN cargo publish -p ${CRATE} --token ${CARGO_REGISTRY_TOKEN}

FROM builder

RUN cargo test --all-features --release --verbose
