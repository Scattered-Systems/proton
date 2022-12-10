/*
    Appellation: toolbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use yew::prelude::*;

pub struct Component {
    pub bg: Option<String>,
}

#[function_component(Toolbar)]
pub fn toolbar() -> Html {
    let value = "";

    html! {
        <div class="bg-transparent text-white flex flex-nowrap items-center justify-between min-w-full max-w-screen sticky bottom-0 m-0 p-3 inset-x-0 z-50">
            <div class="flex flex-auto items-center justify-center">

                <div class="lg:flex grow items-center justify-start sm:hidden xs:hidden" id="main-menu">
                    <ul></ul>
                </div>

            </div>
            <div class="flex items-center justify-between ml-auto ">
                <input
                    class={"flex max-w-sm prose rounded-full px-3 py-1 mx-2 space-x-2"}
                    value={value}
                />
                <button
                    class={"w-24 block rounded bg-gradient-to-br from-cyan-300 via-cyan-400 to-cyan-500 flex flex-auto items-center justify-center mx-auto px-3 py-1 rounded text-white"}
                >
                    {"Search"}
                </button>
            </div>
        </div>
    }
}
