/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{hooks::use_user_context, routes::AppRoute, data::meta::UserInfo};
use super::sidebar::*;
use yew::prelude::*;
use yew_router::prelude::*;


#[function_component(Navbar)]
pub fn navbar() -> Html {
    let user_ctx = use_user_context();
    let brand = "Curiosity";
    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                    { brand }
                </Link<AppRoute>>
                {
                    if user_ctx.is_authenticated() {
                        logged_in_view((*user_ctx).clone())
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
        <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Home" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Sign in" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Sign up" }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}

fn logged_in_view(user_info: UserInfo) -> Html {
    html! {
        <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Home" }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}