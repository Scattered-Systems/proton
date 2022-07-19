FROM jo3mccain/rusty as builder-base

RUN yum install -y \
    glib2-devel \
    gtk3-devel

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --verbose --color always

FROM photon as application

COPY --from=builder /app/target/release/proton /proton

ENTRYPOINT ["./proton"]