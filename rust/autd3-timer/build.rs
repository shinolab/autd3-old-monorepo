/*
 * File: build.rs
 * Project: autd3-timer
 * Created Date: 24/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

#[cfg(target_os = "windows")]
fn main() {}
#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rustc-link-lib=rt");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=pthread");
}
