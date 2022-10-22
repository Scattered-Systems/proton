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
                <Navbar/>
                <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        </UserContextProvider>
    }
}
