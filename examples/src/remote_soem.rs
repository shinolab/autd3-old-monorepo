/*
 * File: soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

mod test_runner;
mod tests;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_soem::RemoteSOEM;

fn main() -> Result<()> {
    let geometry = Geometry::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .build()?;

    let server_ip = "";
    let port = 0;

    let link = RemoteSOEM::builder().addr(server_ip).port(port).build();

    let autd = Controller::open(geometry, link)?;

    test_runner::run(autd)
}
