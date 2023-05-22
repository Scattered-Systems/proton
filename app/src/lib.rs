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
        div { class: "bg-white text-black dark:text-white dark:bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 flex flex-col items-center justify-center m-0 p-0 z-0 min-h-screen min-w-full",
            Router {
                Route { to: "/", self::routes::Homepage(cx.clone()) },
                Route { to: "/settings", self::routes::Settings(cx.clone()) },
            }
        }
        
    })
}

pub fn App(cx: Scope) -> Element {
    
    cx.render(rsx! {
        div {
            "Hello"
        }
    })
}
