/*
 * File: simulator_client.rs
 * Project: src
 * Created Date: 10/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod test_runner;
mod tests;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_simulator::Simulator;

fn main() -> Result<()> {
    let geometry = Geometry::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .add_device(AUTD3::new(
            Vector3::new(DEVICE_WIDTH, 0.0, 0.0),
            Vector3::zeros(),
        ))
        .build()?;

    let link = Simulator::builder().port(8080).build();

    let autd = Controller::open(geometry, link)?;

    test_runner::run(autd)
}
