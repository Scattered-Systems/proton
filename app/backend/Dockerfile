FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM photon

ENV POST=8080

COPY --from=builder /app/target/release/proton-api /bin/proton-api

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp

CMD [ "proton-api" ]