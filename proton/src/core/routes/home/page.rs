/*
    Appellation: page <home>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::view::*;

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex flex-col">
            <MainView/>
        </div>
    }
}
