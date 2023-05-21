/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{
    constants::*,
    statics::*,
    types::*,
};


mod constants {
    pub const LOCALHOST: &str = "0.0.0.0";
}

mod statics {}

mod types {

    pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
    pub type AsyncResult<T = ()> = std::result::Result<T, AsyncError>;
    /// A type alias for a [std::error::Error].
    pub type BoxError = Box<dyn std::error::Error>;
    /// A type alias for [Result].
    pub type StdResult<T = ()> = std::result::Result<T, BoxError>;

}