/*
    Appellation: app <binary>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
use proton_ui::{app, ApplicationScope};

fn main() -> anyhow::Result<()> {
    let scope = ApplicationScope::new().with_name("Proton");
    starter(scope)
}

fn starter(scope: ApplicationScope) -> anyhow::Result<()> {
    #[cfg(feature = "web")]
    dioxus_web::launch_with_props(app, scope, dioxus_web::Config::new().hydrate(true));
    #[cfg(feature = "desktop")]
    dioxus_desktop::launch_with_props(app, scope, dioxus_desktop::Config::new());
    Ok(())
}
