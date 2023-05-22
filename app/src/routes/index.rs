/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use crate::cmp::{footer::Footer, hero::Hero, nav::navbar::NavBar};
use crate::ApplicationScope;
use dioxus::prelude::*;

/// The base application object to be launched
pub fn Homepage(cx: Scope<ApplicationScope>) -> Element {
    cx.render(rsx!(
        header { class: "flex min-w-full max-w-screen body-font prose dark:prose-invert",
                NavBar { banner: cx.props.name.clone() }
        }
        main { class: "flex flex-col grow items-center justify-center min-h-full max-h-screen min-w-full max-w-screen z-0",
            div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                Hero { class: "",
                    src: "https://images.unsplash.com/photo-1617420207078-f9cae65824d5?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1470&q=80",
                    subtitle: "Gambit empowers gamblers with powerful analytical tools designed to enable users to make more informed decisions when placing bets, visiting casinos, and more.",
                    title: cx.props.name.clone(),
                },
            }
        }
        Footer { }
    ))
}
