/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{copy_dir_all, dist_dir, execute_bundle, project_root, Bundle};
use clap::{Args, ArgGroup, Subcommand, ValueEnum};
use duct::cmd;
use proton_sdk::prelude::BoxResult;
use std::process::Command;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, ValueEnum)]
pub enum Linux {
    Debian,
    #[default]
    Ubuntu
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, ValueEnum)]
#[repr(i32)]
pub enum OperatingSystem {
    #[default]
    Linux = 0,
    MacOS = 1,
    Windows = 2
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, ValueEnum)]
pub enum Setup {
    Extras,
    #[default]
    Langspace,
}

#[derive(Clone, Debug, Hash, PartialEq, Subcommand)]
pub enum Commands {
    Compile {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        workspace: bool,
    },
    Create {
        #[clap(long, short, value_parser)]
        name: String,
    },
    Setup {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        extras: bool,
        #[clap(value_enum)]
        linux: Option<Linux>,
    },
    Start {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        dev: bool,
    },
}

impl Commands {
    pub fn handler(&self, desktop: bool, release: bool) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Compile { workspace } => {
                tracing::info!("Compiling the codebase...");
                compile(desktop, *workspace)?;
            }
            Self::Create { name } => {
                println!("{:?}", name.clone());
            }
            Self::Setup { extras, linux } => {
                tracing::info!("Setting up the environment...");
                handle_setup(desktop, extras.clone(), linux.clone())?;
            }
            Self::Start { dev } => {
                tracing::info!("Initializing the application server...");
                start(desktop, *dev)?;
            }
        };
        Ok(self)
    }
}

pub fn compile(desktop: bool, workspace: bool) -> BoxResult {
    let _ = std::fs::remove_dir_all(&dist_dir());
    std::fs::create_dir_all(&dist_dir())?;
    let mut cmds = Bundle::<&str>::new();

    if desktop {
        tracing::info!("Building for desktops...");
        cmds.insert(
            "cargo",
            vec![vec!["tauri", "build", "--config", "proton/Trunk.toml"]]
        );
        copy_dir_all(
            &project_root().join("desktop/target/release/bundle/"), 
            dist_dir().join("bundle")
        )?;
    } 
    cmds.insert(
        "trunk",
        vec![vec!["--config", "proton/Trunk.toml", "build", "--release"]]
    );
    execute_bundle(cmds)?;

    copy_dir_all(
        &project_root().join("proton/dist/"), 
        dist_dir().join("proton/")
    )?;
    

    let tmp = dist_dir().display().to_string();
    // List the contents of the recently created bundle
    Command::new("ls")
        .current_dir(project_root())
        .args(&[tmp.as_str(), "-r"])
        .status()?;

    Ok(())
}

///
pub fn start(desktop: bool, dev: bool) -> BoxResult {
    let mut args = Bundle::<&str>::new();
    if desktop {
        args.insert(
            "cargo",
            vec![vec!["tauri", "dev", "--config", "desktop/tauri.conf.json"]]
        );
    } else {
        if dev {
            args.insert(
                "trunk",
                vec![vec!["--config", "proton/Trunk.toml", "serve"]],
            );
        } else {
            args.insert(
                "trunk",
                vec![vec!["--config", "proton/Trunk.toml", "serve", "--release"]],
            );
        };
    };
    
    
    execute_bundle(args)?;
    Ok(())
}
/// Handler for configuring the workspace
pub fn handle_setup(desktop: bool, extras: bool, linux: Option<Linux>) -> BoxResult {
    let mut args = crate::Bundle::new();
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
        ],
    );
    args.insert(
        "cargo",
        vec![vec!["install", "trunk", "wasm-bindgen-cli"]],
    );
    if desktop {
        args.insert(
            "cargo",
            vec![vec!["install", "tauri-cli"]],
        );
        //
        if linux.is_some() {
            match linux.unwrap_or_default().clone() as i32 {
                0 => {},
                1 => {
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
                },
                _ => {} 
            }
        }
    }
    //
    if extras {
        args.insert(
            "rustup",
            vec![
                vec!["default", "nightly"],
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
    }
    //
    execute_bundle(args)?;
    Ok(())
}
