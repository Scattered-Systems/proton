/*
    Appellation: map <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/

#![allow(non_snake_case)]
use crate::{cmp::layouts::dash::DashboardLayout, ApplicationScope};
use dioxus::prelude::*;

pub fn Mapper(cx: Scope<ApplicationScope>) -> Element {
    cx.render(rsx! {
        DashboardLayout {
            scope: cx.clone(),
            div { class: "",
                div {
                    form { class: "", id: "map-form"
                        }
                }
                div { class: "", id: "map" }
            }
        }
    })
}

#[derive(Clone, Props)]
pub struct MapProps<'a> {
    pub class: &'a str,
}

pub fn EmbeddedMap<'a>(cx: Scope<'a, MapProps<'a>>) -> Element {
    cx.render(rsx! {
        div { class: cx.props.class, id: "map" }
    })
}
