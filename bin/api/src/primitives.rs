/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

mod constants {

    pub const LOCALHOST: [u8; 4] = [0, 0, 0, 0];
    /// [DEFAULT_SERVER_HOST] is the default host for the server
    pub const DEFAULT_SERVER_HOST: [u8; 4] = [0, 0, 0, 0];
    /// [DEFAULT_SERVER_PORT] is the default port for the server
    pub const DEFAULT_SERVER_PORT: u16 = 8080;
    /// [DEFAULT_SERVER_ADDR] is the default address for the server
    pub const DEFAULT_SERVER_ADDR: &str = "0.0.0.0:8080";
}

mod types {
    use std::sync::{Arc, Mutex};
    use tokio::sync::{broadcast, mpsc, oneshot, watch};

    pub type AsyncResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
    pub type Locked<T = ()> = Arc<Mutex<T>>;
    /// [AsyncMutex] is a type alias for a [tokio::sync::Mutex]
    pub type AsyncMutex<T = ()> = tokio::sync::Mutex<T>;
    /// [BroadcastChannels] is a two-tuple consisting of ([broadcast::Sender], [broadcast::Receiver])
    pub type BroadcastChannels<T = ()> = (broadcast::Sender<T>, broadcast::Receiver<T>);
    /// [OneshotChannels] is a two-tuple consisting of ([oneshot::Sender], [oneshot::Receiver])
    pub type OneshotChannels<T = ()> = (oneshot::Sender<T>, oneshot::Receiver<T>);
    /// [UnboundedMPSC] is a two-tuple consisting of ([mpsc::UnboundedSender], [mpsc::UnboundedReceiver])
    pub type UnboundedMPSC<T = ()> = (mpsc::UnboundedSender<T>, mpsc::UnboundedReceiver<T>);
    /// [WatchChannels] is a two-tuple consisting of ([watch::Sender], [watch::Receiver])
    pub type WatchChannels<T = ()> = (watch::Sender<T>, watch::Receiver<T>);
}
