FROM debian as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    build-essential \
    clang \
    curl \
    git \
    llvm \
    pkg-config \
    protobuf-compiler \
    unzip \
    wget 

FROM builder-base as environment-base

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

FROM environment-base as environment

RUN rustup install nightly && \
    rustup target add wasm32-unknown-unknown

FROM environment as builder

RUN curl -fsSL https://sh.iroh.computer/setup.sh | sh -s -- quickstart