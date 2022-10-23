/*
    Appellation: page <home>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{misc::*, view::*};
use crate::actors::files::Model as FileModel;

use yew::prelude::*;

// Home page with an article list and a tag list.
#[function_component(Home)]
pub fn home() -> Html {
    let tag = use_state(|| None);
    let callback = {
        let tag = tag.clone();
        Callback::from(move |t| {
            tag.set(Some(t));
        })
    };

    html! {
        <div class="relative bg-zinc-900 text-white flex min-h-screen m-0 p-0">
            <Banner />
            <div class="flex">
                <div class="relative flex">
                    <div class="sidebar">
                        <p>{ "Popular Tags" }</p>
                        <Tags {callback} />
                    </div>
                </div>
                <div class="container">
                    <FileModel/>
                    <MainView tag={(*tag).clone()} />
                </div>
            </div>
        </div>
    }
}
