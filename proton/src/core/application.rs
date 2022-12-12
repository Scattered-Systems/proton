/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::routes::router::*;
use crate::contexts::provider::UserContextProvider;
use crate::nav::{navbar::Navbar, toolbar::Toolbar};
use yew::prelude::{html, Component, Context, Html};
use yew_router::prelude::{BrowserRouter, Switch};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Msg {
    OpenSidebar = 0,
    #[default]
    CloseSidebar = 1,
}

pub struct App {
    sidebar: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { sidebar: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg as i32 {
            0 => {
                self.sidebar = true;
            }
            1 => {
                self.sidebar = false;
            }
            _ => {
                self.sidebar = false;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let _bg = "";
        html! {
            <UserContextProvider>
                <BrowserRouter>
                    <div class={"bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 flex flex-col m-0 py-3 z-0 min-h-screen min-w-full max-w-screen text-white"}>
                        <Navbar/>
                        <div class="flex flex-auto grow nowrap scrollable m-0 p-0 min-h-full max-h-screen min-w-full max-w-screen">
                            <main class="bg-zinc-700 container mx-auto p-3 prose prose-invert rounded min-h-full max-h-screen w-full">
                                <Switch<Pages> render={switch} />
                            </main>
                        </div>
                        <Toolbar/>
                    </div>
                </BrowserRouter>
            </UserContextProvider>
        }
    }
}
