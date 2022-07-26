/*
    Appellation: components <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use navbar::*;
pub use utils::*;

mod navbar;

pub trait ComponentSpec<App: druid::Data = crate::ApplicationState, Cnt = crate::Context> {
    fn component() -> Result<Box<dyn druid::widget::Widget<App>>, scsys::BoxError>;
    fn new() -> Result<Box<dyn druid::widget::Widget<App>>, scsys::BoxError>;
}

pub trait LayoutSpec<App: druid::Data> {
    fn header() -> druid::widget::Flex<App> where Self: Sized;
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ComponentStyler<Mode> {
    pub mode: Mode,
}

mod utils {}