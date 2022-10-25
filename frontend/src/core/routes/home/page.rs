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
    // let tag = use_state(|| None);
    // let callback = {
    //     let tag = tag.clone();
    //     Callback::from(move |t| {
    //         tag.set(Some(t));
    //     })
    // };

    html! {
        <div class="container mx-auto">
            <Banner />
            <div class="flex flex-auto flex-nowrap mt-3 min-h-full">
                <div class="flex flex-initial items-center justify-center bg-zinc-800 min-h-full mr-3 p-3 rounded-lg w-xs">
                    <div class="container mx-auto min-h-full">
                        <div class="flex text-base">
                            <h1 class="text-bold ">{"Sidebar"}</h1>
                            // <p>{ "Popular Tags" }</p>
                            // <Tags {callback}/>
                        </div>
                        <div class="flex items-center text-base py-3">
                            <ul class="hover:opacity-75">
                                <li>{"Assets"}</li>
                                <li>{"Create"}</li>
                                <li>{"Control"}</li>
                                <li>{"Documents"}</li>
                            </ul>
                            // <p>{ "Popular Tags" }</p>
                            // <Tags {callback}/>
                        </div>
                    </div>
                </div>
                <div class="bg-zinc-800 flex flex-auto grow items-center justify-center p-3 min-h-full rounded-lg">
                    <FileModel/>
                    // <MainView tag={(*tag).clone()} />
                </div>
            </div>
        </div>
    }
}
