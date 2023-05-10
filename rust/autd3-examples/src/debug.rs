/*
 * File: soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod test_runner;
mod tests;

use anyhow::Result;

use autd3::link::Debug;
use autd3::prelude::*;

fn main() -> Result<()> {
    let geometry = Geometry::builder()
        .advanced_phase()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .build()
        .unwrap();

    let link = Debug::new();

    let autd = Controller::open(geometry, link).expect("Failed to open");

    run!(autd);

    Ok(())
}
