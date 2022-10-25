FROM scsys/rust:debian-lts as builder-base

RUN apt-get update -y && spt-get upgrade -y && rustup update

RUN apt-get apt install -y \
        libwebkit2gtk-4.0-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY app .
RUN cargo build --release -v

FROM photon

COPY --from=builder /app/target/release/proton-desktop /bin/proton-desktop

CMD [ "proton-desktop" ]