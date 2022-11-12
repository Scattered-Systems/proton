FROM scsys/rust:debian-lts as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM builder

RUN cargo test --all-features --release --verbose
