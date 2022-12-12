/*
    Appellation: proton-xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use proton_sdk::prelude::BoxResult;
use proton_xtask::{dist_dir, project_root};
use std::{fs, process::Command};

use duct::cmd;
use man::prelude::*;

fn main() -> BoxResult {
    tracing_subscriber::fmt::init();
    proton_xtask::cli::CommandLineInterface::default().handler()?;
    Ok(())
}

pub trait BuildPipeline {
    fn init() -> Self;
    fn handle(&self) -> BoxResult<&Self>;
    fn run(&mut self) -> BoxResult;
    fn stage(&self) -> String;
}

#[derive(Default)]
pub struct Xtask;

impl Xtask {
    pub fn dev(&self) -> BoxResult {
        let cmd = cmd!("trunk")
            .dir(project_root())
            .pipe(cmd!("--config", "proton/Trunk.toml", "serve"))
            .run()?;
        tracing::info!("{:?}", cmd);

        Ok(())
    }
    pub fn dist(&self) -> BoxResult {
        let _ = fs::remove_dir_all(&dist_dir());
        fs::create_dir_all(&dist_dir())?;

        dist_binary()?;
        // dist_manpage()?;

        Ok(())
    }
    pub fn start(&self) -> BoxResult {
        let mut args = std::collections::HashMap::<&str, Vec<Vec<&str>>>::new();
        args.insert(
            "trunk",
            vec![vec!["--config", "proton/Trunk.toml", "serve"]],
        );
        self.execute_bundle(args)?;
        Ok(())
    }
    pub fn workspace(&self) -> BoxResult {
        let mut args = std::collections::HashMap::<&str, Vec<Vec<&str>>>::new();
        args.insert(
            "sudo",
            vec![
                vec!["apt", "update", "-y"],
                vec!["apt", "upgrade", "-y"],
                vec!["apt", "autoremove", "-y"],
                vec![
                    "apt",
                    "install",
                    "-y",
                    "libgtk-3-dev",
                    "libwebkit2gtk-4.0-dev",
                    "libappindicator3-dev",
                    "librsvg2-dev",
                    "patchelf",
                    "protobuf-compiler",
                ],
            ],
        );
        args.insert(
            "cargo",
            vec![vec!["install", "tauri-cli", "trunk", "wasm-bindgen-cli"]],
        );
        args.insert(
            "rustup",
            vec![
                vec!["default", "nightly"],
                vec![
                    "target",
                    "add",
                    "wasm32-unknown-unknown",
                    "wasm32-wasi",
                    "--toolchain",
                    "nightly",
                ],
                vec![
                    "component",
                    "add",
                    "clippy",
                    "rustfmt",
                    "--toolchain",
                    "nightly",
                ],
            ],
        );
        //
        self.execute_bundle(args)?;
        Ok(())
    }
    pub fn execute_bundle(
        &self,
        args: std::collections::HashMap<&str, Vec<Vec<&str>>>,
    ) -> BoxResult<&Self> {
        for k in args.keys() {
            // Step 1: Rustup
            for i in 0..args[k].len() {
                let mut cmd = Command::new(k);
                cmd.current_dir(project_root());
                cmd.args(args[k][i].clone().as_slice()).status()?;
            }
        }
        Ok(self)
    }
    pub fn handle(&self, task: Option<String>) -> BoxResult<&Self> {
        match task.as_ref().map(|it| it.as_str()) {
            Some("dev") => self.dev()?,
            Some("dist") => self.dist()?,
            Some("setup") => self.workspace()?,
            Some("start") => self.start()?,
            _ => print_help(),
        };
        Ok(self)
    }
    pub fn run(&self) -> BoxResult {
        tracing::info!("{:?}", "Starting the build");
        let task = std::env::args().nth(1);
        self.handle(task)?;
        Ok(())
    }
}

pub enum Stages {
    Before,
    During,
    After,
    Initialization,
    Startup,
    Shutdown,
}

fn print_help() {
    eprintln!(
        "Tasks:
dev             Bootstrap's the application with a development server
dist            builds application
setup           Automated setup procedures for ubuntu*
"
    )
}

fn dist_binary() -> BoxResult {
    tracing::info!("{:?}", "Started building the binaries...");
    let a = cmd!("trunk")
        .dir(project_root())
        .pipe(cmd!("--config", "proton/Trunk.toml", "build", "--release"))
        .run();
    let cmd = "trunk".to_string();
    let status = Command::new(cmd)
        .current_dir(project_root())
        .args(&["--config", "proton/Trunk.toml", "build", "--release"])
        .status()?;

    if !a.is_err() {
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

fn dist_manpage() -> BoxResult {
    let page = Manual::new("proton")
        .about("Proton is a complete, cloud-native computational platform empowered with decentralized technologies.")
        .render();
    fs::write(dist_dir().join("proton.wasm"), &page.to_string())?;
    Ok(())
}
