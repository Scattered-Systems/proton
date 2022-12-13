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
pub enum Target {
    #[default]
    Wasm32UnknownUnknown = 0
}

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
        #[clap(value_enum)]
        target: Option<Target>,
    },
}

impl Commands {
    pub fn handler(&self, desktop: bool, release: bool) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Compile { workspace } => {
                tracing::info!("Compiling the codebase...");
                clean_dir(&dist_dir())?;
                // handle_compile(desktop, *workspace)?;
                if desktop {
                    compile_desktop()?;
                } else {
                    handle_wasm_pack()?;
                    compile_js()?
                }
            }
            Self::Create { name } => {
                println!("{:?}", name.clone());
            }
            Self::Setup { extras, linux } => {
                tracing::info!("Setting up the environment...");
                handle_setup(desktop, extras.clone(), linux.clone())?;
            }
            Self::Start { target } => {
                tracing::info!("Initializing the application server...");
                handle_start(desktop, release, target.clone())?;
            }
        };
        Ok(self)
    }
}

pub fn command(program: &str, args: Vec<&str>) -> BoxResult {
    let mut cmd = Command::new(program);
    cmd.current_dir(project_root());
    cmd.args(args.as_slice()).status()?;
    Ok(())
}

pub fn handle_wasm_pack() -> BoxResult {
    command("wasm-pack", vec!["build", "proton"])?;

    copy_dir_all(
        &project_root().join("proton/pkg"), 
        dist_dir().join("proton/")
    )?;

    Ok(())
}

pub fn npm(args: Vec<&str>) -> BoxResult {
    command("npm", args)?;
    Ok(())
}

pub fn compile_js() -> BoxResult {
    npm(vec!["run", "build"])?;
    Ok(())
}

pub fn compile_desktop() -> BoxResult {
    tracing::info!("Building for desktops...");
    let mut cmds = Bundle::<&str>::new();
    cmds.insert(
        "cargo",
        vec![vec!["tauri", "build", "--config", "proton/Trunk.toml"]]
    );
    copy_dir_all(
        &project_root().join("desktop/target/release/bundle/"), 
        dist_dir().join("bundle")
    )?;
    Ok(())
}

pub fn clean_dir(path: &std::path::PathBuf) -> BoxResult {
    tracing::debug!("Clearing out the previous build");
    let _ = std::fs::remove_dir_all(path);
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub fn wasm_handle_trunkrs() -> BoxResult {
    let mut cmds = Bundle::<&str>::new();
    cmds.insert(
        "trunk",
        vec![vec!["--config", "proton/Trunk.toml", "build", "--release"]]
    );
    execute_bundle(cmds)?;

    copy_dir_all(
        &project_root().join("proton/dist/"), 
        dist_dir().join("proton/")
    )?;
    Ok(())
}

pub fn handle_compile(desktop: bool, workspace: bool) -> BoxResult {
    clean_dir(&dist_dir())?;
    let mut cmds = Bundle::<&str>::new();
    if desktop {
        compile_desktop()?;
    } 
    
    

    let tmp = dist_dir().display().to_string();
    // List the contents of the recently created bundle
    Command::new("ls")
        .current_dir(project_root())
        .args(&[tmp.as_str(), "-r"])
        .status()?;

    Ok(())
}

///
pub fn handle_start(desktop: bool, release: bool, target: Option<Target>) -> BoxResult {
    let mut args = Bundle::<&str>::new();
    if desktop {
        args.insert(
            "cargo",
            vec![vec!["tauri", "dev", "--config", "desktop/tauri.conf.json"]]
        );
    } else {
        let mut tmp = vec!["--config", "proton/Trunk.toml", "serve"];
        if release {
            tmp.push("--release");
        }
        args.insert(
            "trunk",
            vec![tmp.clone()],
        );
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
    args.insert(
        "npm",
        vec![vec!["install", "-g", "wasm-pack"]],
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
                                "clang",
                                "cmake",
                                "libgtk-3-dev",
                                "libwebkit2gtk-4.0-dev",
                                "libappindicator3-dev",
                                "librsvg2-dev",
                                "llvm",
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
