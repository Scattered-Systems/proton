/*
    Appellation: stores <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        A face is equivalent to a webpage, route, etc
*/
pub use state::*;

mod state;

/// Outline the standard implementations of a druid widget (W) with data (d)
pub trait StateSpec<Cont: druid::Widget<Self>, Data: Clone + druid::Data>: Sized {
    fn canvas(&self) -> Result<druid::widget::Flex<Self>, scsys::BoxError>
        where
            Self: Sized;
    fn configure(&self) -> Result<Self, config::ConfigError>
        where
            Self: Sized;
    fn display(&self) -> Result<Data, scsys::BoxError>
        where
            Self: Sized;
}
