FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN apt-get -y \
    protobuf-compiler

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
VOLUME /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --color ${CARGO_TERM_COLOR} --release --verbose --workspace

FROM scratch as appstore

COPY --from=builder /workspace/target/release/proton-backend /space/proton-backend
VOLUME /space

FROM photon:latest as runner-base

RUN yum update -y && yum upgrade -y
COPY --from=appstore  /space/proton-backend /bin/proton-backend

ENV PORT=8000

EXPOSE 80
EXPOSE ${PORT}

CMD ["proton-backend"]