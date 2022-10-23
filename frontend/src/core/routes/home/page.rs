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
        <div class="container mx-auto min-h-full relative my-16">
            <Banner />
            <div class="flex py-3">
                <div class="bg-gradient-to-r from-neutral-100 via-neutral-200 to-neutral-100 text-black p-3 min-h-full rounded-lg">
                    <div class="">
                        <p>{ "Popular Tags" }</p>
                        <Tags {callback} />
                    </div>
                </div>
                <div class="container mx-auto">
                    <FileModel/>
                    <MainView tag={(*tag).clone()} />
                </div>
            </div>
        </div>
    }
}
