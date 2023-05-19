/*
    Appellation: sidebar <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use crate::ApplicationScope;
use dioxus::prelude::*;

/// [Sidebar] is a dynamic component typically leveraged with data-driven views
pub fn Sidebar(cx: Scope<ApplicationScope>) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-900", id: "sidebar",
            div {class: "", id: "sidebar-header",
                h1 { class: "text-4xl text-white font-bold", "Sidebar" }
            }
        }
    })
}
