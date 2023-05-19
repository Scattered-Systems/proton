/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Clone, Debug, Default, Props)]
pub struct HeroProps<'a> {
    pub class: &'a str,
    pub src: &'a str,
    pub subtitle: &'a str,
    pub title: String,
    pub children: Element<'a>,
}

pub fn Hero<'a>(cx: Scope<'a, HeroProps<'a>>) -> Element {
    cx.render(rsx!(
        div { class: "flex grow {cx.props.class}",
            div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center prose prose-invert",
                h1 { class: "title-font sm:text-4xl text-3xl mb-4 font-medium",
                    br { class: "hidden lg:inline-block" }
                    "{cx.props.title}"
                }
                p { class: "mb-8 leading-relaxed",
                    "{cx.props.subtitle}"
                }
                div { class: "flex justify-center",
                    button {
                        class: "inline-flex text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                        "About"
                    }
                    button {
                        class: "ml-4 inline-flex text-gray-400 bg-gray-800 border-0 py-2 px-6 focus:outline-none hover:bg-gray-700 hover:text-white rounded text-lg",
                        "Build"
                    }
                }
            }
            div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
                img {
                    class: "object-cover object-center rounded",
                    src: "{cx.props.src}",
                    referrerpolicy:"no-referrer",
                    alt: "hero",
                }
            }
        }
        &cx.props.children
    ))
}
