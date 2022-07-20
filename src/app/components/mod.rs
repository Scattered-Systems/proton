/*
    Appellation: components <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use nav::*;

mod nav;

/// Outlines a standard component model for building Druid applications
pub trait Component<D: druid::Data> {
    /// Each component implements appellation, enforcing a common naming convention
    fn appellation(&self) -> Result<(u64, String, String), scsys::BoxError>
        where
            Self: Sized;
    fn canvas<T: druid::Widget<Self>>() -> T
        where
            Self: Sized;
    fn configure(&self) -> Result<Self, config::ConfigError>
        where
            Self: Sized;
    fn display() -> Result<D, scsys::BoxError>
        where
            Self: Sized;
}
