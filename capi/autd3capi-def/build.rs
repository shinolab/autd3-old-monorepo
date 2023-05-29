/*
 * File: build.rs
 * Project: autd3capi-def
 * Created Date: 29/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::env;

use wrapper_generator::generate;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    generate(crate_dir).expect("Failed to generate bindings");
}
