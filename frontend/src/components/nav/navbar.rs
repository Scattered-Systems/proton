/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::sidebar::*;
use crate::{data::meta::UserInfo, hooks::use_user_context, routes::AppRoute};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let ctx = use_user_context();
    let brand = "Proton";
    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                    { brand }
                </Link<AppRoute>>
                {
                    if ctx.is_authenticated() {
                        logged_in_view((*ctx).clone())
                    } else {
                        logged_out_view()
                    }
                }
                <Sidebar/>
            </div>
        </nav>
    }
}

fn logged_out_view() -> Html {
    html! {
        <ul class="nav navbar-nav pull-xs-left">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Home" }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}

fn logged_in_view(user_info: UserInfo) -> Html {
    html! {
        <ul class="nav navbar-nav pull-xs-left">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Home" }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}
