/*
 * File: build.rs
 * Project: AUTD server
 * Created Date: 07/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    println!(
        "cargo:rerun-if-changed={}/src/main.rs",
        manifest_dir.join("../simulator").display()
    );
    println!(
        "cargo:rerun-if-changed={}/src/main.rs",
        manifest_dir.join("../SOEMAUTDServer").display()
    );

    Command::new("cargo")
        .current_dir(manifest_dir.join("../simulator"))
        .args(["build", "--release"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new("cargo")
        .current_dir(manifest_dir.join("../SOEMAUTDServer"))
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
    std::fs::copy(
        manifest_dir.join(format!(
            "../SOEMAUTDServer/target/release/SOEMAUTDServer{}",
            ext
        )),
        manifest_dir.join(format!(
            "SOEMAUTDServer-{}{}",
            std::env::var("TARGET").unwrap(),
            ext
        )),
    )
    .unwrap();

    tauri_build::build()
}
