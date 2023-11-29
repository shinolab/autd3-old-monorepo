/*
 * File: build.rs
 * Project: autd3capi
 * Created Date: 26/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::env;

use autd3capi_wrapper_generator::generate;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if let Err(e) = generate(crate_dir) {
        eprintln!("{}", e);
    }
}
