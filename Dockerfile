FROM jo3mccain/rusty as builder

ADD bin/proton /app
WORKDIR /app

COPY bin/proton .
RUN cargo build --release --verbose --color always

FROM debian:buster-slim as application

COPY --from=builder /app/target/release/aether /aether

ENTRYPOINT ["./aether"]