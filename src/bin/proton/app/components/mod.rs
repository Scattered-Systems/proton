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
    fn constructor() -> Result<Box<dyn druid::widget::Widget<App>>, scsys::BoxError>;
}

pub trait LayoutSpec<App, Cnf> {
    fn canvas(&mut self) -> Result<druid::widget::Flex<App>, scsys::BoxError>
        where
            Self: Sized;
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ComponentStyler<Mode> {
    pub mode: Mode,
}

pub struct ComponentParams {
    pub class: String,
    pub id: String,
    pub key: String,
    pub style: std::collections::HashMap<String, String>,
}

mod utils {}
