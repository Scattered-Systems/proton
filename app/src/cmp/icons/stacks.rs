/*
    Appellation: stacks <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
#![allow(non_snake_case)]
use dioxus::prelude::*;

///
pub fn StacksIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 24 24",
            path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"}
        }
    ))
}
