/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{
    context::*,
    routes::{switch, AppRoute},
};
use crate::components::nav::*;
use yew::prelude::*;
use yew_router::prelude::*;

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <BrowserRouter>
                <div class="bg-gradient-br from-zinc-900 via-zinc-800 to-zinc-900 flex flex-auto items-center justify-center m-0 min-h-screen p-0 text-white z-0">
                    <Navbar/>
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </div>
            </BrowserRouter>
        </UserContextProvider>
    }
}
