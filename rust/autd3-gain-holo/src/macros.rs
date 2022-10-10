/*
 * File: macros.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{Complex, MatrixXc};
use autd3_core::{
    geometry::{Geometry, Transducer, Vector3},
    utils::directivity_t4010a1 as directivity,
    NUM_TRANS_IN_UNIT,
};
#[allow(unused)]
use nalgebra::ComplexField;

pub fn propagate(
    source_pos: &Vector3,
    source_dir: &Vector3,
    atten: f64,
    wavenum: f64,
    target: Vector3,
) -> Complex {
    let diff = target - source_pos;
    let dist = diff.norm();
    let theta = source_dir.angle(&diff);

    let d = directivity(theta);
    let r = d * (-dist * atten).exp() / dist;
    let phi = -wavenum * dist;
    r * Complex::new(0., phi).exp()
}

pub fn generate_propagation_matrix<T: Transducer>(
    geometry: &Geometry<T>,
    foci: &[Vector3],
) -> MatrixXc {
    let m = foci.len();
    let num_device = geometry.num_devices();
    let num_trans = num_device * NUM_TRANS_IN_UNIT;
    let sound_speed = geometry.sound_speed();

    MatrixXc::from_iterator(
        m,
        num_trans,
        geometry.transducers().flat_map(|trans| {
            foci.iter().map(move |&fp| {
                let wavenum = trans.wavenumber(sound_speed);
                propagate(
                    trans.position(),
                    trans.z_direction(),
                    geometry.attenuation,
                    wavenum,
                    fp,
                )
            })
        }),
    )
}
