/*
    Appellation: actor <home>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use yew::prelude::{Component, Context, Html, html};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Msg {
    Loaded,
    ToggleTheme
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Dashboard {

}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded => {
                
                true
            },
            Msg::ToggleTheme => {

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let banner = "Proton";
        html! {
            <>
                <nav class="navbar navbar-expand-lg  blur blur-rounded top-0 border-bottom z-index-3 shadow w-100 mt-4 d-none d-lg-block my-3 py-2">
                    <div class="container-fluid">
                        <a class="navbar-brand font-weight-bolder ms-3" href="" rel="tooltip" title="Designed and Coded by Creative Tim" data-placement="bottom" target="_blank">
                            {banner}
                        </a>
                        <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navigation" aria-controls="navigation" aria-expanded="false" aria-label="Toggle navigation">
                            <span class="navbar-toggler-icon"></span>
                        </button>
                        <div class="collapse navbar-collapse" id="navigation">
                        <ul class="navbar-nav navbar-nav-hover mx-auto">
                            <li class="nav-item px-3">
                            <a class="nav-link">
                                {"Pages"}
                            </a>
                            </li>

                            <li class="nav-item px-3">
                            <a class="nav-link">
                                {"Utilities"}
                            </a>
                            </li>

                            <li class="nav-item px-3">
                            <a class="nav-link">
                                {"Blocks"}
                            </a>
                            </li>

                            <li class="nav-item px-3">
                            <a class="nav-link">
                                {"Docs"}
                            </a>
                            </li>
                        </ul>

                        <ul class="navbar-nav">
                            <button class="btn btn-sm  bg-gradient-dark  btn-round mb-0 me-1">{"Account"}</button>
                        </ul>
                        </div>
                    </div>
                </nav>
                <div class="container-fluid">
                    <span>
                        { "Dashboard" }
                    </span>
                </div>
            </>
        }
    }
}