FROM rust as base-image

RUN apt-get update -y && apt-get upgrade -y

FROM base-image as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
VOLUME /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --color ${CARGO_TERM_COLOR} --release --verbose --workspace

FROM scratch as appstore

RUN mkdir /space
VOLUME /space
WORKDIR /space

COPY --from=builder /workspace/target/release/proton-backend /proton-backend

FROM photon:latest as runner-base

RUN yum update -y && yum upgrade -y
COPY --from=appstore  /bin/proton-backend /bin/proton-backend

ENV PORT=8000

EXPOSE 80
EXPOSE ${PORT}

CMD ["proton-backend"]