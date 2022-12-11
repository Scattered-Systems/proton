/*
    Appellation: workspace <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::types::*;


pub(crate) mod types {
    ///
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
    ///
    pub type AsyncResult<T = ()> = Result<T, AsyncError>;
    ///
    pub type BoxError = Box<dyn std::error::Error>;
    ///
    pub type BoxResult<T = ()> = Result<T, BoxError>;
    ///
    pub type IOError = std::io::Error;
    ///
    pub type IOResult<T = ()> = std::io::Result<T>;
}