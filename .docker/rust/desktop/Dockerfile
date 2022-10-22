FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN sudo apt install -y \
          libwebkit2gtk-4.0-dev \
          build-essential \
          curl \
          wget \
          libssl-dev \
          libgtk-3-dev \
          libayatana-appindicator3-dev \
          librsvg2-dev

FROM builder-base as tauri

RUN rustup update && \
    cargo install create-tauri-app

