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

RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
RUN source /home/gitpod/.wasmedge/env

FROM builder

ENTRYPOINT [ "wasmedge" ]
CMD [ "-h" ]