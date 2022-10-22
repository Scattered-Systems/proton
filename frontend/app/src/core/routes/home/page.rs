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
        <div class="container-fluid m-0 p-3">
            <Banner />
            <div class="row page">
                <div class="col-md-3 col-xs-12 min-h-screen">
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
