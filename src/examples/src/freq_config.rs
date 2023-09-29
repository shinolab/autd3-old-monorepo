/*
 * File: freq_config.rs
 * Project: src
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use autd3::link::Debug;
use autd3::prelude::*;

fn main() -> Result<()> {
    let mut autd = Controller::builder()
        .advanced()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .open_with(Debug::new().with_log_level(LevelFilter::MoreSevereEqual(Level::Debug)))?;

    for dev in autd.geometry_mut() {
        for tr in dev {
            tr.set_frequency(70e3)?;
        }
    }

    autd.send(Synchronize::new())?;

    Ok(())
}
