/*
 * File: soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

mod test_runner;
mod tests;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_soem::SOEM;

fn main() -> Result<()> {
    let geometry = Geometry::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .build()?;

    let link = SOEM::new().with_on_lost(|msg| {
        eprintln!("Unrecoverable error occurred: {msg}");
        std::process::exit(-1);
    });

    let autd = Controller::open(geometry, link)?;

    test_runner::run(autd)
}
