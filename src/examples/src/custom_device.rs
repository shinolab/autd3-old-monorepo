/*
 * File: custom_device.rs
 * Project: src
 * Created Date: 16/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod test_runner;
mod tests;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_simulator::Simulator;

struct ConcentricArray {}

impl autd3::core::geometry::Device for ConcentricArray {
    fn get_transducers(&self, start_id: usize) -> Vec<(usize, Vector3, UnitQuaternion)> {
        [(start_id, Vector3::zeros(), UnitQuaternion::identity())]
            .into_iter()
            .chain((0..8).flat_map(|layer| {
                (0..6 * layer).map(move |i| {
                    let theta = 2.0 * PI * i as float / (6 * layer) as float;
                    let pos = layer as float * 10.0 * Vector3::new(theta.cos(), theta.sin(), 0.0);
                    (start_id + 1 + i, pos, UnitQuaternion::identity())
                })
            }))
            .collect()
    }
}

fn main() -> Result<()> {
    let autd = Controller::builder()
        .advanced()
        .add_device(ConcentricArray {})
        .open_with(Simulator::new(8080))?;

    test_runner::run(autd)
}
