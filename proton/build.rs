/*
    Appellation: proton-wasm <build>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Create's a compact wasm binary file leveraging substrate_wasm_builder
*/
use substrate_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new()
        // Tell the builder to build the project (crate) this `build.rs` is part of.
        .with_current_project()
        // Make sure to export the `heap_base` global, this is required by Substrate
        .export_heap_base()
        // Build the Wasm file so that it imports the memory (need to be provided by at instantiation)
        .import_memory()
        // Build it.
        .build()
}
