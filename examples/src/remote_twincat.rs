/*
 * File: remote_twincat.rs
 * Project: src
 * Created Date: 22/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/06/2023
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
    let autd = Controller::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .open_with(RemoteTwinCAT::new("0.0.0.0.0.0")?)?;

    test_runner::run(autd)
}
