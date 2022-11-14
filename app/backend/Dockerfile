FROM scratch as space

COPY Backend.toml /config/Backend.toml
VOLUME /config

FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN apt-get install -y \
    protobuf-compiler

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
VOLUME /workspace
WORKDIR /workspace

COPY . .
COPY Backend.toml Backend.toml
RUN cargo build --color ${CARGO_TERM_COLOR} --release --verbose --workspace

FROM scratch as cache

COPY --from=builder /workspace/target/release/proton-backend /space/proton/proton-backend
VOLUME /space

FROM photon:latest as runner-base

RUN yum update -y && yum upgrade -y

COPY Backend.toml Backend.toml
RUN chmod -R 777 Backend.toml
COPY --from=cache  /space/proton-backend /bin/proton-backend

ENV PORT=8000

EXPOSE 80
EXPOSE ${PORT}

CMD ["proton-backend"]