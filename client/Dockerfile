FROM node:18.12.1 as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as langspace

RUN apt-get install -y \
    clang \
    cmake \
    git \
    llvm \
    wget
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM langspace as builder-base

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

RUN npm install -g wasm-pack

FROM builder-base as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN npm install && npm run build

FROM builder as development

ENV MODE="production"

EXPOSE 3000
CMD [ "npm", "run", "start" ]

FROM node:lts-alpine as runner

RUN apt-get update -y && apt-get upgrade -y

COPY --chown=55 --from=builder /workspace/build ./build

FROM runner

ENV MODE="production"

EXPOSE 3000
CMD ["node", "build/index.js"]
