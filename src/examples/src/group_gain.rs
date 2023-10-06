/*
 * File: group.rs
 * Project: src
 * Created Date: 02/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use autd3::prelude::*;

fn main() -> Result<()> {
    let mut autd = Controller::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .open_with(Nop::builder())?;

    let cx = autd.geometry().center().x;
    let g1 = Focus::new(autd.geometry()[0].center() + Vector3::new(0., 0., 150.0 * MILLIMETER));
    let g2 = Null::new();
    let g = Group::new(move |_dev, tr: &LegacyTransducer| {
        if tr.position().x < cx {
            Some("focus")
        } else {
            Some("null")
        }
    })
    .set("focus", g1)
    .set("null", g2);

    let m = Sine::new(150);
    autd.send((m, g))?;

    autd.close()?;

    Ok(())
}
