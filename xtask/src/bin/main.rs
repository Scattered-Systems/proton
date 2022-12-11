/*
    Appellation: proton-xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

use proton_xtask::{project_root, BaseResult};
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use man::prelude::*;

fn main() -> BaseResult {
    tracing_subscriber::fmt::init();
    println!("{:?}", project_root());
    if let Err(e) = Xtask::default().run() {
        eprintln!("{}", e);
        std::process::exit(-1);
    };
    Ok(())
}

#[derive(Default)]
pub struct Xtask;

impl Xtask {
    pub fn run(&self) -> BaseResult {
        tracing::info!("{:?}", "Starting the build");
        let task = std::env::args().nth(1);
        let _res = match task.as_ref().map(|it| it.as_str()) {
            Some("dist") => dist()?,
            _ => print_help(),
        };
        Ok(())
    }
}

fn print_help() {
    eprintln!(
        "Tasks:

dist            builds application
"
    )
}

fn dist() -> BaseResult {
    let _ = fs::remove_dir_all(&dist_dir());
    fs::create_dir_all(&dist_dir())?;

    dist_binary()?;
    // dist_manpage()?;

    Ok(())
}

fn dist_binary() -> BaseResult {
    tracing::info!("{:?}", "Started building the binaries...");
    let cmd = env::var("TRUNK").unwrap_or_else(|_| "trunk".to_string());
    let status = Command::new(cmd)
        .current_dir(project_root())
        .args(&["--config", "proton/Trunk.toml", "build", "--release"])
        .status()?;

    if !status.success() {
        tracing::info!("{:?}", "Failed to complete the build...");
        Err("Build failed")?;
    }

    let dst = project_root().join("proton/dist/");

    proton_xtask::copy_dir_all(&dst, dist_dir().join("proton"))?;

    eprintln!("Cleaning up binaries");
    let tmp = dist_dir().join("proton").display().to_string();
    // List the contents of the recently created bundle
    Command::new("ls")
        .current_dir(project_root())
        .args(&[tmp.as_str(), "-r"])
        .status()?;
    Ok(())
}

fn dist_manpage() -> BaseResult {
    let page = Manual::new("proton")
        .about("Proton is a complete, cloud-native computational platform empowered with decentralized technologies.")
        .render();
    fs::write(dist_dir().join("proton.wasm"), &page.to_string())?;
    Ok(())
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}
