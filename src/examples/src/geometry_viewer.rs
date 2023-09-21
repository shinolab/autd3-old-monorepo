/*
 * File: geometry_viewer.rs
 * Project: src
 * Created Date: 29/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;
use autd3_geometry_viewer::GeometryViewer;

fn main() -> anyhow::Result<()> {
    let autd = Controller::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .add_device(AUTD3::new(
            Vector3::new(0., 0., AUTD3::DEVICE_WIDTH),
            Vector3::new(0., PI / 2., 0.),
        ))
        .add_device(AUTD3::new(
            Vector3::new(AUTD3::DEVICE_WIDTH, 0., AUTD3::DEVICE_WIDTH),
            Vector3::new(0., PI, 0.),
        ))
        .add_device(AUTD3::new(
            Vector3::new(AUTD3::DEVICE_WIDTH, 0., 0.),
            Vector3::new(0., -PI / 2., 0.),
        ))
        .open_with(NullLink {})
        .unwrap();

    let code = GeometryViewer::new().run(autd.geometry())?;

    std::process::exit(code)
}
