use std::env::var;
use std::path::PathBuf;

fn main() {
    let target = var("TARGET").expect("failed to get target tripple");

    // Search for the workspace manifest
    let this_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut ws_dir: Option<PathBuf> = None;
    for parent in this_dir.ancestors().skip(1) {
        if parent.join("Cargo.toml").exists() {
            ws_dir = Some(parent.to_path_buf());
        }
    }
    let ws_dir = ws_dir
        .map(|p| p.to_string_lossy().to_string())
        .expect(
            "failed to determain cargo workspace directory, \
            make sure this package is under a workspace"
        );

    println!("cargo:rustc-env=TARGET={target}");
    println!("cargo:rustc-env=CARGO_WORKSPACE_DIR={ws_dir}");
}
