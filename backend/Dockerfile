FROM rust:latest as builder-base

RUN rustup update

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
COPY ../.config .
RUN cargo build --color always --release -v

FROM photon:latest

ENV DATABASE_URL="postgresql://postgres:example@localhost:5432/realworld" \
    HMAC_KEY="8JKSy8oPkuQhs25bV0OGfFgngxN42fQouGPQkq2Z4gcxFKd4gY8EDtT9wCCXE2nc" \
    RUST_LOG=realworld_axum_sqlx=debug,tower_http=debug

RUN yum update -y && yum upgrade -y

COPY --from=builder /app/target/release/app /bin/app

EXPOSE 8080

CMD [ "app" ]
