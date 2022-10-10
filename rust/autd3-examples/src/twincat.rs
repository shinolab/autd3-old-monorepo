/*
 * File: twincat.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/06/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod test_runner;
mod tests;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_twincat::TwinCAT;

fn main() -> Result<()> {
    let mut geometry = GeometryBuilder::new().legacy_mode().build();
    geometry.add_device(Vector3::zeros(), Vector3::zeros());

    let link = TwinCAT::new();

    let autd = Controller::open(geometry, link).expect("Failed to open");

    run!(autd);

    Ok(())
}
