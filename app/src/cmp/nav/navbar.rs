/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
//! Navbar
//!
//!
#![allow(non_snake_case)]
use super::Navigation;
use crate::cmp::icons::{RightArrowIcon, StacksIcon};
use dioxus::prelude::*;
use dioxus_router::Link;

pub fn NavBar(cx: Scope<Navigation>) -> Element {
    let is_auth = false;
    let pages = ["Dashboard", "Settings"];
    cx.render(
        rsx!(
            nav { class: "flex flex-nowrap grow items-center justify-center bg-transparent prose prose-invert min-w-full max-w-screen my-0 p-3 sticky top-0 z-50",
                a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0", href: "/",
                    StacksIcon {}
                    span { class: "ml-3 text-xl", "{cx.props.banner}"}
                }
                div { class: "flex grow hidden lg:flex items-center justify-start p-3",
                    ul { class: "flex grow items-center justify-start list-none",
                        Link { class: "block px-3 py-2 hover:opacity-75 hover:italic", to: "/dashboard", li { class: "", "Dashboard"}}
                        Link { class: "block px-3 py-2 hover:opacity-75 hover:italic", to: "/settings", li { class: "", "Settings"}}
                    }
                }
                button {
                    class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                    onclick:  move |_| {login(is_auth)},
                    "Button"
                    RightArrowIcon {}
                }
            }
        )
    )
}

fn login(is_auth: bool) {
    if is_auth {
        println!("Logout");
    } else {
        println!("Login");
    }
}
