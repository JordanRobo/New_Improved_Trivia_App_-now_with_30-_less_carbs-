use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=app/src");
    println!("cargo:rerun-if-changed=app/public");

    // Build SvelteKit project
    let output = Command::new("bun")
        .current_dir("app")
        .args(&["run", "build"])
        .output()
        .expect("Failed to build SvelteKit project");

    if !output.status.success() {
        panic!(
            "Vite build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Check if the build directory exists
    let build_path = Path::new("app/dist");
    if !build_path.exists() || !build_path.is_dir() {
        panic!("app/dist directory does not exist after build");
    }
}
