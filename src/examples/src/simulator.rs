/*
 * File: simulator.rs
 * Project: src
 * Created Date: 27/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod tests;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_simulator::Simulator;

fn main() -> Result<()> {
    let autd = Controller::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .add_device(AUTD3::new(
            Vector3::new(AUTD3::DEVICE_WIDTH, 0.0, 0.0),
            Vector3::zeros(),
        ))
        .open_with(Simulator::builder(8080))?;

    tests::run(autd)
}
