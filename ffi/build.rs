use std::env;

// Use from hello.rs
fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/target/", project_dir);
}
