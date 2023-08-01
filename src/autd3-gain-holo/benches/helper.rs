/*
 * File: helper.rs
 * Project: benches
 * Created Date: 30/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    autd3_device::AUTD3,
    float,
    geometry::{Device, Geometry, Transducer, Vector3},
};

pub fn generate_geometry<T: Transducer>(size: usize) -> Geometry<T> {
    let mut transducers = Vec::new();
    let mut device_map = Vec::new();
    for i in 0..size {
        for j in 0..size {
            let id = transducers.len();
            let mut t = AUTD3::new(
                Vector3::new(
                    i as float * AUTD3::DEVICE_WIDTH,
                    j as float * AUTD3::DEVICE_HEIGHT,
                    0.,
                ),
                Vector3::zeros(),
            )
            .get_transducers(id);
            device_map.push(t.len());
            transducers.append(&mut t);
        }
    }
    Geometry::<T>::new(
        transducers
            .iter()
            .map(|&(id, pos, rot)| T::new(id, pos, rot))
            .collect(),
        device_map,
        340e3,
        0.,
    )
    .unwrap()
}
