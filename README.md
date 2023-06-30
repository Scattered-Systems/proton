# proton

[![Pages](https://github.com/scattered-systems/proton/actions/workflows/pages.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/pages.yml)


[![Clippy](https://github.com/scattered-systems/proton/actions/workflows/clippy.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/clippy.yml)
[![Desktop](https://github.com/scattered-systems/proton/actions/workflows/desktop.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/desktop.yml)
[![Docker](https://github.com/scattered-systems/proton/actions/workflows/docker.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/docker.yml)
[![Rust](https://github.com/scattered-systems/proton/actions/workflows/rust.yml/badge.svg)](https://github.com/scattered-systems/proton/actions/workflows/rust.yml)

***

Proton is a cloud-native platform manifesting the active decentralized namespace into a unique experience empowering everyday users to make use of blockchain technologies.

Proton is desiged to be a multi-platform portal facilitating a myriad of internet-based experiences empowered by a unique blend of technologies.
Proton focuses on providing a seamless experience for users, developers, and content creators alike. Proton is powered by a custom orchestration mechanism,
Flow. Flow is a harmonic orchestration mechanism built to effeciently manage the execution of a myriad of tasks, from simple to complex on any number of surfaces.
This allows users to enjoy the benefits of blockchain and cloud technologies without the hassle of managing them.

## Token

Proton is distributed as an ERC-6551 token, wrapping pre-existing ENS namespaces enabling users to access the Proton portal anywhere in the world.
Owners will enjoy custom wallets, fully-managed application / website hosting furthered with a built-in low-code editor, and a suite of other features.

## About

Proton integrates with the [Scattered Systems](https://github.com/scattered-systems) ecosystem serving as a gateway to the future of internet-based experiences. 

## Features

Proton works closely with the nix ecosystem to provide a consistent and reliable experience across all supported platforms. This enables proton
to be deployed on a wide range of devices and operating systems without the need for additional configuration. Moreover, proton is designed to be
web-accessible, allowing users to access their platform and resources from anywhere in the world.

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
