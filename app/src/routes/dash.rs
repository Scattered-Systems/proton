/*
    Appellation: dashboard <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use crate::cmp::layouts::dash::DashboardLayout;
use crate::ApplicationScope;
use dioxus::prelude::*;

pub fn Dashboard(cx: Scope<ApplicationScope>) -> Element {
    cx.render(rsx! {
        DashboardLayout {
            scope: cx.clone(),
            div {
                h1 { "Dashboard" }
                p { "Welcome to the dashboard!" }
            }
        }
    })
}
