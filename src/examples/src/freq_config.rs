/*
 * File: freq_config.rs
 * Project: src
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use autd3::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut autd = Controller::builder()
        .advanced()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .open_with(Nop::builder())
        .await?;

    for dev in &mut autd.geometry {
        for tr in dev {
            tr.set_frequency(70e3)?;
        }
    }

    autd.send(Synchronize::new()).await?;

    Ok(())
}
