/*
    Appellation: gambit-ui <lib>
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
            Route { to: "/dashboard", self::routes::dash::Dashboard(cx.clone()) },
            Route { to: "/auth/login", self::routes::auth::Login(cx.clone()) },
            Route { to: "/auth/register", self::routes::auth::Register(cx.clone()) },
            Route { to: "/settings", self::routes::Settings(cx.clone()) }
        }
    })
}

