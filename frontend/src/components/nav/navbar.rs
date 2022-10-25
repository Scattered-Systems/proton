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
        <nav class="absolute bg-transparent text-white flex flex-nowrap items-center justify-between navbar navbar-expand-lg py-3 w-full">
            <div class="inline-flex w-32 px-3">
                <Link<AppRoute> to={AppRoute::Home} classes="inline px-3">
                    { brand }
                </Link<AppRoute>>

            </div>
            <div class="flex grow justify-start hover:text-underline hover:italic">
                {
                    if ctx.is_authenticated() {
                        logged_in_view((*ctx).clone())
                    } else {
                        logged_out_view()
                    }
                }
            </div>
            <div class="mx-3">
                <button class="bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 px-3 py-1 rounded-full">
                    {"Settings"}
                </button>
            </div>
        </nav>
    }
}

fn logged_out_view() -> Html {
    html! {
        <ul class="inline-flex list-none">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link hover:underline">
                    { "Dashboard" }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}

fn logged_in_view(user_info: UserInfo) -> Html {
    html! {
        <ul class="list-none inline-flex">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Dashboard" }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}
