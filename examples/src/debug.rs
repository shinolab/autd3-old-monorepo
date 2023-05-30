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

use autd3::link::Debug;
use autd3::prelude::*;

fn main() -> Result<()> {
    let geometry = Geometry::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .build()?;

    let link = Debug::builder().build();

    let autd = Controller::open(geometry, link)?;

    test_runner::run(autd)
}
