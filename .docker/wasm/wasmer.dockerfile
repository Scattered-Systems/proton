FROM debian:buster-slim as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    build-essential \
    clang \
    cmake \
    llvm \
    pkg-config \
    protobuf-compiler

FROM builder-base as builder

RUN curl https://get.wasmer.io -sSfL | sh

FROM builder

ENTRYPOINT [ "wasmer" ]
CMD [ "-h" ]
