#[path = "src/catalog.rs"]
mod catalog;

use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=src/catalog.rs");
    println!("cargo::rerun-if-changed=../granola/src/daisyui");
    println!("cargo::rerun-if-changed=../granola/src/daisyui.rs");

    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("granola-scanner must be in the workspace crates directory");
    if !root.join("crates/granola-scanner").is_dir() {
        return;
    }

    let resources = root.join("resources");
    std::fs::create_dir_all(&resources).expect("could not create the resources directory");
    catalog::write_daisyui_safelist(resources.join("safelist"))
        .expect("could not write resources/safelist");
}
