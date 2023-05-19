/*
    Appellation: login <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
#![allow(non_snake_case)]
use crate::ApplicationScope;
use dioxus::prelude::*;

pub fn Login(cx: Scope<ApplicationScope>) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-900", id: "firebaseui-auth-container",
            LoginForm(cx.clone())
        }
    })
}

async fn sample_login(evt: &FormEvent) -> anyhow::Result<()> {
    let resp = reqwest::Client::new()
        .post("http://localhost:8080/login")
        .form(&[
            ("username", &evt.values["username"]),
            ("password", &evt.values["password"]),
        ])
        .send()
        .await?;

    println!("Response: {:?}", resp);

    Ok(())
}

pub fn LoginForm(cx: Scope<ApplicationScope>) -> Element {
    let onsubmit = move |evt: FormEvent| {
        cx.spawn(async move { sample_login(&evt).await.expect("Login failed") });
    };

    cx.render(rsx! {
        div { class: "w-full max-w-md",
            h1 { class: "", "Login" }
            form { class: "", id: "login-form", onsubmit: onsubmit, prevent_default: "onsubmit", method: "POST",
                input { id: "username", name: "username", r#type: "text" }
                label { "Username" }
                br {}
                input { id: "password", name: "password", r#type: "password" }
                label { "Password" }
                br {}
                button { "Login" }
            }
        }
    })
}
