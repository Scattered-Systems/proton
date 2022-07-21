/*
    Appellation: chaos <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use application::*;
pub use components::*;
pub use controllers::*;
pub use states::*;

mod application;
mod states;

mod components {}

mod controllers {
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct WindowParams {
        pub shape: (f64, f64),
    }

    impl WindowParams {
        pub fn new(shape: (f64, f64)) -> Result<Self, scsys::BoxError> {
            Ok(Self { shape })
        }
        pub fn default() -> Self {
            Self::new((800f64, 600f64)).ok().unwrap()
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Controller {
        pub name: String,
        pub pages: Vec<String>,
        pub window: WindowParams,
    }

    impl Controller {
        pub fn new(
            name: String,
            pages: Vec<String>,
            window: WindowParams,
        ) -> Result<Self, scsys::BoxError> {
            Ok(Self {
                name,
                pages,
                window,
            })
        }
        pub fn default() -> Self {
            let pages: Vec<String> = vec![
                "Dashboard",
                "Artifacts",
                "Connect",
                "Discover",
                "Create",
                "Settings",
            ]
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>();
            Self::new("Proton".to_string(), pages.clone(), WindowParams::default())
                .ok()
                .unwrap()
        }
    }
}
