/*
    Appellation: app <binary>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
extern crate dotenv;
use proton_ui::{app, ApplicationScope};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let scope = ApplicationScope::new().with_name("Gambit");
    starter(scope)
}

fn starter(scope: ApplicationScope) -> anyhow::Result<()> {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch_with_props(app, scope, dioxus_web::Config::new().hydrate(true));
    #[cfg(any(macos, unix, windows))]
    dioxus_desktop::launch_with_props(app, scope, dioxus_desktop::Config::new());
    Ok(())
}
