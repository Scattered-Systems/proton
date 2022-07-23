/*
    Appellation: components <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use navbar::*;
pub use utils::*;

mod navbar;

pub trait LayoutSpec<App, Cnf> {
    fn canvas(&mut self) -> Result<druid::widget::Flex<App>, scsys::BoxError>
        where
            Self: Sized;
}

mod utils {}
