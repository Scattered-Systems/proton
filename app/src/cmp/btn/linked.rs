/*
    Appellation: btn <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
use dioxus::prelude::*;

#[derive(Clone, Props)]
pub struct LinkedProps<'a> {
    href: &'a str,
    children: Element<'a>,
}
