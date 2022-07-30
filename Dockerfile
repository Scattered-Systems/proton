FROM jo3mccain/rusty:latest as builder-base

RUN yum install -y \
    glib2-devel \
    gtk3-devel

FROM builder-base as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --color always --release --verbose --workspace

FROM photon as application

COPY --from=builder /workspace/target/release/proton /proton

ENTRYPOINT ["./proton"]