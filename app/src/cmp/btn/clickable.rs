/*
    Appellation: clicker <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Clone, Debug, Props)]
pub struct ClickableProps<'a> {
    href: &'a str,
    children: Element<'a>,
}

pub fn Clickable<'a>(cx: Scope<'a, ClickableProps<'a>>) -> Element {
    cx.render(rsx!(
        a { class: "fancy-button", href: "{cx.props.href}",
            &cx.props.children
        }
    ))
}
