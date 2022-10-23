/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        When using this component, ensure that you place it where you may like the toggle button to appear as most of the rest of the component is, well offcanvas
*/
use crate::{hooks::use_user_context, routes::AppRoute};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let user_ctx = use_user_context();
    let toggle_label = "Control Panel";
    let title = "Sidebar";
    let mut is_open = false;
    html! {
        <>
            <button id="btnSidebarToggler" class="rounded-full bg-indigo-200 px-3 py-2 text-black" type="button">
                <svg id="navClosed" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-8 w-8">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                </svg>
                <svg id="navOpen" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="hidden h-8 w-8">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
            </button>
            
        </>
    }
}
