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
                div { class: "flex ml-auto",
                    button {id: "theme-toggle", class: "text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm p-2.5", 
                        svg  {id: "theme-toggle-dark-icon", class: "hidden w-5 h-5", fill: "currentColor", view_box: "0 0 20 20", xmlns: "http://www.w3.org/2000/svg", 
                            path {d: "M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z", }
                        }
                        svg { id: "theme-toggle-light-icon", class: "hidden w-5 h-5", fill: "currentColor", view_box: "0 0 20 20", xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z", 
                                fill_rule: "evenodd",
                                clip_rule: "evenodd"}
                        }
                    }
                    button {
                        class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                        onclick:  move |_| {login(is_auth)},
                        "Button"
                        RightArrowIcon {}
                    }
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
