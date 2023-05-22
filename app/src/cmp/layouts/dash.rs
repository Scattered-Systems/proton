/*
    Appellation: dashboard <layout>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use crate::cmp::nav::navbar::NavBar;
use crate::ApplicationScope;
use dioxus::prelude::*;

#[derive(Clone, Props)]
pub struct DashboardProps<'a> {
    pub scope: Scope<'a, ApplicationScope>,
    pub children: Element<'a>,
}

pub fn DashboardLayout<'a>(cx: Scope<'a, DashboardProps<'a>>) -> Element {
    cx.render(rsx!(
        div { class: "flex flex-col items-center justify-center m-0 p-0 z-0 min-h-screen min-w-full",
            header { class: "flex min-w-full max-w-screen body-font prose prose-invert",
                NavBar {
                    banner: cx.props.scope.props.name.clone()
                }
            }
            main { class: "flex flex-col grow items-center justify-center min-h-full max-h-screen min-w-full max-w-screen z-0", id: "dashboard-content",
                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center rounded-md", id: "display",
                    &cx.props.children
                }
            }
        }
    ))
}
