/*
    Appellation: register <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use crate::ApplicationScope;
use dioxus::prelude::*;

pub fn Register(cx: Scope<ApplicationScope>) -> Element {
    cx.render(rsx! {
        div {
            h1 { class: "prose prose:dark", "Register" }
            form {

            }
        }
    })
}
