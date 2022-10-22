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
    html! {
        <>
            <button class="btn btn-outline-primary" type="button" data-bs-toggle="offcanvas" data-bs-target="#sidebar" aria-controls="sidebar">
                {toggle_label}
            </button>
            <div class="offcanvas offcanvas-start" tabindex="-1" id="sidebar" aria-labelledby="sidebarLabel">
                <div class="offcanvas-header">
                    <h5 class="offcanvas-title" id="sidebarLabel">{ title }</h5>
                    <button type="button" class="btn-close" data-bs-dismiss="offcanvas" aria-label="Close"></button>
                </div>
                <div class="offcanvas-body">
                    <div>
                        {"Application"}
                    </div>
                    <div class="vstack gap-3">
                        <div class="bg-light border">
                            {"Automate"}
                        </div>
                        <div class="bg-light border">
                            {"Control"}
                        </div>
                        <div class="bg-light border">
                            {"Discover"}
                        </div>
                    </div>
                    <div class="dropdown mt-3 relative bottom-0">
                        <button class="btn btn-secondary dropdown-toggle" type="button" data-bs-toggle="dropdown">
                            <i class="fa-regular fa-user px-1"></i>
                            { "Account" }
                        </button>
                        <ul class="dropdown-menu">
                            <li><a class="dropdown-item" href="#">{"Actions"}</a></li>
                            <li><a class="dropdown-item" href="#">{"Content"}</a></li>
                            <li><a class="dropdown-item" href="#">{"Schedule"}</a></li>
                        </ul>
                    </div>
                </div>
            </div>
        </>
    }
}
