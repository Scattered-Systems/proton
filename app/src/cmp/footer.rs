/*
    Appellation: footer <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::Link;

/// The base application object to be launched
pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx!(
        footer { class: "flex flex-nowrap grow items-center justify-center bg-transparent prose prose-invert min-w-full max-w-screen my-0 p-3 absolute bottom-0 z-0", id: "footer",
                div { class: "flex flex-col items-center justify-start hidden lg:flex",
                    ul { class: "flex list-none", 
                        Link { to: "/", li { class: "", "Home" } }
                    }
                }
                div { class: "flex grow items-center justify-center",
                    "© 2023 Scattered-Systems, LLC. All rights reserved."
                }
                div { class: "flex items-center justify-end",
                    ul { class: "list-none", 
                        div { id: "sample" }
                    }
                }
            }
    ))
}
