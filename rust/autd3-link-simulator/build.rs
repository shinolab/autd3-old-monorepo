/*
 * File: build.rs
 * Project: autd3-link-soem
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::env;
use std::path::PathBuf;

fn main() {
    let boost_dir = env::var("BOOST_ROOT")
        .expect("Please install Boost and set BOOST_ROOT environment variable");

    let mut build = cc::Build::new();
    build.cpp(true).warnings(false);
    if cfg!(target_os = "windows") {
        build.flag("/std:c++17");
        build.define("WIN32", None);
    } else {
        build.flag("-std=c++17");
    }
    build
        .include(format!("{}", boost_dir))
        .file("boost_wrap.cpp")
        .opt_level(2)
        .compile("boost");

    let mut builder = bindgen::Builder::default();
    if cfg!(target_os = "windows") {
        builder = builder.clang_arg("-DWIN32");
    }
    let bindings = builder
        .header("boost_wrap.h")
        .allowlist_function("shmem_create")
        .allowlist_function("shmem_copy_to")
        .allowlist_function("shmem_copy_from")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("boost_bindings.rs"))
        .expect("Couldn't write bindings!");
}
