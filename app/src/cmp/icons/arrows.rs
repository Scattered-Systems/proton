/*
    Appellation: arrows <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
#![allow(non_snake_case)]
use dioxus::prelude::*;

///
pub fn RightArrowIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-4 h-4 ml-1",
            view_box: "0 0 24 24",
            path { d: "M5 12h14M12 5l7 7-7 7"}
        }
    ))
}
