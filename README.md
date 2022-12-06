# proton

[![Clippy](https://github.com/scattered-systems/proton/actions/workflows/clippy.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/clippy.yml)
[![Docker](https://github.com/scattered-systems/proton/actions/workflows/docker.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/docker.yml)
[![Rust](https://github.com/scattered-systems/proton/actions/workflows/rust.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/rust.yml)

***

Proton is a cloud-native platform manifesting the active decentralized namespace into a unique experience empowering everyday users to make use of blockchain technologies.

## Installation

Make sure you have docker installed on the target system

### *Pull the image*

```bash
docker pull jo3mccain/template-cli-rs:latest
```

### *Build the image locally (optional)*

```bash
docker buildx build --tag proton:latest .
```

### *Run the image*

```bash
docker run -p 9000:9000 proton:latest
```

## Usage

```bash
    cli -h 
```

## Contributors

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

* [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT](https://choosealicense.com/licenses/mit/)
