FROM rust as base-image

RUN apt-get update -y && apt-get upgrade -y

FROM base-image as builder

ENV COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --color ${COLOR} --release --verbose --workspace

FROM debian:latest as runner-base

RUN apt-get update -y && apt-get upgrade -y

FROM runner-base as runner

COPY --from=builder /workspace/target/release/proton-backend /bin/proton-backend

FROM runner

ENV PORT=8000

EXPOSE 8000:${PORT}

CMD ["proton-backend"]