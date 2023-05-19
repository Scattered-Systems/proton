/*
    Appellation: web <binary>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
use proton_ui::{app, ApplicationScope};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let scope = ApplicationScope::new().with_name("Proton");
    dioxus_web::launch_with_props(app, scope, dioxus_web::Config::new().hydrate(true));
    Ok(())
}