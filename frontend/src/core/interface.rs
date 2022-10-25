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
                <div class="bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 text-white">
                    <div class="flex flex-auto grow min-h-screen m-0 p-0">
                        
                        <div class="flex flex-auto grow pt-16">
                            <div class="flex grow items-center justify-center min-h-full">
                                <Switch<AppRoute> render={Switch::render(switch)} />
                            </div>
                        </div>
                    </div>
                </div>
            </BrowserRouter>
        </UserContextProvider>
    }
}
