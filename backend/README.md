# template-restful-rs



## Developers

### Getting Started

#### _Building from the source_
    git clone https://gitlab.com/FL03/template-restful-rs

    cargo build --release --workspace
    cargo run --release

#### _Docker_

Build locally with

    docker build . --tag {username}/{project}:{tag}
    docker run {username}/{project}:{tag}

or pull a pre-built image

    docker pull jo3mccain/template-restful-rs:latest
    docker run -d jo3mccain/template-restful-rs

#### _Docker Compose_

    docker compose up -d -f .docker/docker-compose.yml

## Resources

* [Crates.io](https://crates.io)
* [Repository](https://gitlab.com/FL03/template-restful-rs)
* [Sea ORM Docs](https://docs.rs/sea_orm)
