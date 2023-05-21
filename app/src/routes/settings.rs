/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use crate::cmp::layouts::dash::DashboardLayout;
use crate::ApplicationScope;
use dioxus::prelude::*;

pub fn Settings(cx: Scope<ApplicationScope>) -> Element {
    cx.render(rsx! {
        DashboardLayout {
            scope: cx.clone(),
            div {
                h1 { class: "prose prose:dark", "Settings" }
            }
        }

    })
}
