/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{constants::*, statics::*, types::*};

pub(crate) mod constants {
    pub const DEFAULT_PORT: usize = 8080;
}

pub(crate) mod statics {
    #[global_allocator]
    pub static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
}

pub(crate) mod types {
    pub const DEFAULT_PORT: usize = 8080;
}