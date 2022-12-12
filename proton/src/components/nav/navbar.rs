/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use yew::prelude::*;

#[function_component(NavbarLogo)]
pub fn navbar_logo() -> Html {
    let link = "/";
    let logo = "https://pzzld.eth.limo/_app/immutable/assets/favicon-7286b642.png";
    let title = "Proton";
    html! {
       <a class="flex max-w-sm px-3" href={link} >
           <button class="flex flex-auto nowrap hover:opacity-95 w-full">
               <img alt="#" class="mr-3 h-6 sm:h-9 rounded-full" src={logo} />
               <span class="self-center font-semibold whitespace-nowrap prose prose-invert prose-2xl hover:underline">
                   {title}
               </span>
           </button>
       </a>
    }
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="bg-transparent text-white flex flex-nowrap items-center justify-between min-w-full max-w-screen sticky top-0 m-0 p-3 inset-x-0 z-50">
            <div class="flex flex-auto items-center justify-center">
                <NavbarLogo/>
                <div class="lg:flex grow items-center justify-start sm:hidden xs:hidden" id="main-menu">
                    <ul class="flex-inline list-none">
                        <li><a href="https://pzzld.eth.limo">{"Home"}</a></li>
                    </ul>
                </div>
                <div class="justify-end">
                <slot/>
                </div>
            </div>

        </nav>
    }
}
