/*
    Appellation: proton-ui <lib>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
pub use self::{primitives::*, scope::*, utils::*};

mod primitives;
mod scope;
mod utils;

pub mod actors;
pub mod cmp;
pub mod core;
pub mod routes;

use dioxus::prelude::*;
use dioxus_router::{Route, Router};

pub fn app(cx: Scope<ApplicationScope>) -> Element {

    cx.render(rsx! {
        Router {
            Route { to: "/", self::routes::Homepage(cx.clone()) },
        }
    })
}