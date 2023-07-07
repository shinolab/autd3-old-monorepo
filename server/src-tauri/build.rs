use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    Command::new("cargo")
        .current_dir(manifest_dir.join("../simulator"))
        .args(["build", "--release"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let ext = if cfg!(target_os = "windows") {
        ".exe"
    } else {
        ""
    };

    std::fs::copy(
        manifest_dir.join(format!("../simulator/target/release/simulator{}", ext)),
        manifest_dir.join(format!(
            "simulator-{}{}",
            std::env::var("TARGET").unwrap(),
            ext
        )),
    )
    .unwrap();

    tauri_build::build()
}
