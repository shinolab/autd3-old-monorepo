/*
 * File: build.rs
 * Project: AUTD Server
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

    let ext = if cfg!(target_os = "windows") {
        ".exe"
    } else {
        ""
    };

    if cfg!(target_os = "macos") {
        Command::new("cargo")
            .current_dir(manifest_dir.join("../simulator"))
            .args(["build", "--release", "--target=x86_64-apple-darwin"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        Command::new("cargo")
            .current_dir(manifest_dir.join("../simulator"))
            .args(["build", "--release", "--target=aarch64-apple-darwin"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        Command::new("cargo")
            .current_dir(manifest_dir.join("../SOEMAUTDServer"))
            .args(["build", "--release", "--target=x86_64-apple-darwin"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        Command::new("cargo")
            .current_dir(manifest_dir.join("../SOEMAUTDServer"))
            .args(["build", "--release", "--target=aarch64-apple-darwin"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        std::fs::copy(
            manifest_dir.join("../simulator/target/x86_64-apple-darwin/release/simulator"),
            manifest_dir.join("simulator-x86_64-apple-darwin"),
        )
        .unwrap();
        std::fs::copy(
            manifest_dir.join("../simulator/target/aarch64-apple-darwin/release/simulator"),
            manifest_dir.join("simulator-aarch64-apple-darwin"),
        )
        .unwrap();
        Command::new("lipo")
            .current_dir(manifest_dir)
            .args([
                "-create",
                "simulator-x86_64-apple-darwin",
                "simulator-aarch64-apple-darwin",
                "-output",
                "simulator-universal-apple-darwin",
            ])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        std::fs::copy(
            manifest_dir
                .join("../SOEMAUTDServer/target/x86_64-apple-darwin/release/SOEMAUTDServer"),
            manifest_dir.join("SOEMAUTDServer-x86_64-apple-darwin"),
        )
        .unwrap();
        std::fs::copy(
            manifest_dir
                .join("../SOEMAUTDServer/target/aarch64-apple-darwin/release/SOEMAUTDServer"),
            manifest_dir.join("SOEMAUTDServer-aarch64-apple-darwin"),
        )
        .unwrap();
        Command::new("lipo")
            .current_dir(manifest_dir)
            .args([
                "-create",
                "SOEMAUTDServer-x86_64-apple-darwin",
                "SOEMAUTDServer-aarch64-apple-darwin",
                "-output",
                "SOEMAUTDServer-universal-apple-darwin",
            ])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
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
    };

    tauri_build::build()
}
