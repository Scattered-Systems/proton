FROM ubuntu as workspace

RUN apt-get install -y \
    build-essential \
    curl \
    cmake

RUN apt-get update -y && apt-get install -y nodejs npm yarn

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM workspace as builderspace

RUN apt-get install -y libcairo2-dev libffi-dev libglib2.0-dev libpcre2-dev

RUN rustup toolchain install nightly && \
    rustup toolchain install stable-gnu && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    apt-get update

FROM builderspace as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --package proton --bin proton

FROM debian:buster-slim

COPY --from=builder /app/target/release/proton /proton

ENV DEV_MODE=false \
    PORT=9999

EXPOSE ${PORT}
CMD ["./proton"]