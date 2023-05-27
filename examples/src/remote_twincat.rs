/*
 * File: remote_twincat.rs
 * Project: src
 * Created Date: 22/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod test_runner;
mod tests;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_twincat::RemoteTwinCAT;

fn main() -> Result<()> {
    let geometry = Geometry::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .build()?;

    let link = RemoteTwinCAT::builder()
        .server_ams_net_id("0.0.0.0.0.0")
        .build()?;

    let autd = Controller::open(geometry, link)?;

    test_runner::run(autd)
}
