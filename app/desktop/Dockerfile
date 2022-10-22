FROM rust:latest as builder-base

RUN apt-get update -y && spt-get upgrade -y && rustup update

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY app .
RUN cargo build --release -v

FROM photon

COPY --from=builder /app/target/release/proton-desktop /bin/proton-desktop

CMD [ "proton-desktop" ]