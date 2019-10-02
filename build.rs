use std::env;
use std::path::PathBuf;

fn main() {
    add_bass();
}

#[cfg(target_os = "windows")]
fn add_bass() {
    println!("cargo:rustc-link-lib=bass");
}

#[cfg(target_os = "macos")]
fn add_bass() {
    println!("cargo:rustc-link-lib=bass");
    println!("cargo:rustc-link-search=/usr/local/lib");
}
