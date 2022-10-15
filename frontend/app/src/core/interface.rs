/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
//! The root app contains initial authentication and url routes
use crate::{dashboard::Dashboard, docs::Docs};
use yew::prelude::*;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum RootRoute {
    #[at("/")]
    Home,
    #[at("/:s")]
    Route,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Dashboard,
    #[at("/docs")]
    Docs,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(
        move |_| history.push(Route::Dashboard)
    );
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn root_route(routes: &RootRoute) -> Html {
    match routes {
        RootRoute::Home => html! { <p class="text-4xl">{ "Yew Template. " }</p> },
        RootRoute::Route => html! {
            <Switch<Route> render={Switch::render(switch)} />
        },
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Dashboard => html! { <Dashboard/> },
        Route::Docs => html! { <Docs/> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
