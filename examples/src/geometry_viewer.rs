/*
 * File: simulator_server copy.rs
 * Project: src
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;
use autd3_geometry_viewer::GeometryViewer;

fn main() {
    let geometry = Geometry::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .add_device(AUTD3::new(
            Vector3::new(0., 0., DEVICE_WIDTH),
            Vector3::new(0., PI / 2., 0.),
        ))
        .add_device(AUTD3::new(
            Vector3::new(DEVICE_WIDTH, 0., DEVICE_WIDTH),
            Vector3::new(0., PI, 0.),
        ))
        .add_device(AUTD3::new(
            Vector3::new(DEVICE_WIDTH, 0., 0.),
            Vector3::new(0., -PI / 2., 0.),
        ))
        .build()
        .unwrap();

    let res = GeometryViewer::new().run(&geometry);
    std::process::exit(res);
}
