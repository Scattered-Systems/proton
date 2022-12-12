/*
    Appellation: view <home>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub tag: Option<String>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Tab {
    All,
    Feed,
    Tag,
}

/// Main content with tabs of article list for home page
#[function_component(MainView)]
pub fn main_view(_props: &Props) -> Html {
    html! {
        <>
            <div class="flex">
                <span>
                    { "Feed" }
                </span>
            </div>
        </>
    }
}
