/*
    Appellation: faces <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        A face is equivalent to a webpage, route, etc
*/
pub use home::*;

pub mod calc;
mod home;

pub trait Face<Cmp: druid::Data> {
    fn appellation(&self) -> String
        where
            Self: Sized;
    fn display(&self);
    fn layout(&self);
}
