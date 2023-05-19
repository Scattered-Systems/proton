/*
   Appellation: server <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

pub trait TryIntoSocketAddr {
    type Error;

    fn try_into_socket_addr(self) -> Result<SocketAddr, Self::Error>;
}

impl<T> TryIntoSocketAddr for T
where
    T: TryInto<SocketAddr>,
{
    type Error = <T as TryInto<SocketAddr>>::Error;

    fn try_into_socket_addr(self) -> Result<SocketAddr, Self::Error> {
        self.try_into()
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
pub enum ServerType {
    #[default]
    TCP,
    UDP,
}

pub enum ServerAddress {
    Address(SocketAddr),
    Pieces { host: IpAddr, port: u16 },
}

impl ServerAddress {
    pub fn new(host: IpAddr, port: u16) -> Self {
        Self::Pieces { host, port }
    }
    pub fn address(&self) -> SocketAddr {
        match self {
            Self::Address(addr) => *addr,
            Self::Pieces { host, port } => SocketAddr::new(*host, *port),
        }
    }
    pub fn host(&self) -> IpAddr {
        match self {
            Self::Address(addr) => addr.ip(),
            Self::Pieces { host, .. } => *host,
        }
    }
    pub fn port(&self) -> u16 {
        match self {
            Self::Address(addr) => addr.port(),
            Self::Pieces { port, .. } => *port,
        }
    }
}

impl Default for ServerAddress {
    fn default() -> Self {
        Self::Address(crate::DEFAULT_SERVER_ADDR.parse().unwrap())
    }
}
impl From<SocketAddr> for ServerAddress {
    fn from(addr: SocketAddr) -> Self {
        Self::Address(addr)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ServerConfig {
    addr: SocketAddr,
    server_type: ServerType,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            addr: crate::DEFAULT_SERVER_ADDR.parse().unwrap(),
            server_type: ServerType::default(),
        }
    }
}

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }
    pub fn address(&self) -> SocketAddr {
        self.addr
    }
    pub fn host(&self) -> IpAddr {
        self.address().ip()
    }
    pub fn port(&self) -> u16 {
        self.address().port()
    }
    pub fn set(&mut self, addr: SocketAddr) {
        self.addr = addr;
    }
}
