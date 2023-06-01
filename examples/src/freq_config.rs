/*
 * File: freq_config.rs
 * Project: src
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use autd3::link::Debug;
use autd3::prelude::*;

fn main() -> Result<()> {
    let geometry = Geometry::builder()
        .advanced()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .build()?;

    let link = Debug::new().with_log_level(LevelFilter::MoreSevereEqual(Level::Debug));

    let mut autd = Controller::open(geometry, link)?;

    for tr in autd.geometry_mut() {
        tr.set_frequency(70e3)?;
    }

    Ok(())
}
