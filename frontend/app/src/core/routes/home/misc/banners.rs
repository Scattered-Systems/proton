/*
    Appellation: banners <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::hooks::use_user_context;
use serde::{Deserialize, Serialize};
use yew::prelude::*;


#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BannerContent {
    subtitle: String,
    title: String,

}

impl BannerContent {
    pub fn new(subtitle: String, title: String) -> Self {
        Self { subtitle, title }
    }
    pub fn set_subtitle<T: std::string::ToString>(&mut self, data: T) -> &Self {
        self.subtitle = data.to_string();
        self
    }
    pub fn set_title<T: std::string::ToString>(&mut self, data: T) -> &Self {
        self.title = data.to_string();
        self
    }
}

#[function_component(Banner)]
pub fn banner() -> Html {
    let user_ctx = use_user_context();
    let mut content = BannerContent::default();
    content.set_subtitle("A unified interface for your decentralized namespace");
    content.set_title("Curiosity");
    if user_ctx.is_authenticated() {
        html! {}
    } else {
        html! {
            <div class="banner">
                <div class="container">
                    <h1 class="logo-font">
                        { content.title }
                    </h1>
                    <p>{ content.subtitle }</p>
                </div>
            </div>
        }
    }
}
