/*
    Appellation: proton-desktop <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    description:
        Proton is a unique runtime environment capable of engaging a myriad of providers

*/
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
