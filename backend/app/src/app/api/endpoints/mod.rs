/*
    Appellation: endpoints <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::index::Homepage;

mod index;

pub trait PageRouter {
    fn path(&self) -> String;
    fn router(&self) -> axum::Router;
}
