# proton

[![Pages](https://github.com/scattered-systems/proton/actions/workflows/pages.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/pages.yml)


[![Clippy](https://github.com/scattered-systems/proton/actions/workflows/clippy.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/clippy.yml)
[![Desktop](https://github.com/scattered-systems/proton/actions/workflows/desktop.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/desktop.yml)
[![Docker](https://github.com/scattered-systems/proton/actions/workflows/docker.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/docker.yml)
[![Rust](https://github.com/scattered-systems/proton/actions/workflows/rust.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/rust.yml)

***

Proton is a cloud-native platform manifesting the active decentralized namespace into a unique experience empowering everyday users to make use of blockchain technologies.

## Getting Started

### Building from the Source

Make sure you have rust installed on your host system

#### *Clone the repository*

```bash
git clone https://github.com/scattered-systems/proton
```

#### *Setup the environment*

```bash
cargo xtask setup
```

#### *Start the application*

```bash
cargo xtask start
```

### Docker

Make sure you have docker installed on the target system

#### *Pull the image*

```bash
docker pull scsys/proton:latest
```

#### *Build the image locally (optional)*

```bash
docker buildx build --tag scsys/proton:latest .
```

#### *Run the image*

```bash
docker run -p 9000:9000 scsys/proton:latest
```

## Usage

### Builder (xtask)

```bash
    cargo xtask -h 
```

## Contributors

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
