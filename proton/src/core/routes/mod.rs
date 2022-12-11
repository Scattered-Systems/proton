/*
    Appellation: router <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::router::*;

pub mod home;

pub(crate) mod router {
    use super::home::Home;
    use yew::prelude::*;
    use yew_router::prelude::*;

    #[derive(Debug, Clone, PartialEq, Eq, Routable)]
    pub enum Pages {
        #[at("/")]
        Home,
        #[not_found]
        #[at("/404")]
        NotFound,
    }

    pub fn switch(route: Pages) -> Html {
        match route {
            Pages::Home => html! {<Home />},
            Pages::NotFound => html! { "Page not found" },
        }
    }
}
