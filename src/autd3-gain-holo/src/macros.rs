/*
 * File: macros.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::MatrixXc;
use autd3_core::{
    acoustics::{propagate, Sphere},
    geometry::{Geometry, Transducer, Vector3},
};

pub fn generate_propagation_matrix<T: Transducer>(
    geometry: &Geometry<T>,
    foci: &[Vector3],
) -> MatrixXc {
    let m = foci.len();
    let num_trans = geometry.num_transducers();

    let sound_speed = geometry.sound_speed;
    let attenuation = geometry.attenuation;

    MatrixXc::from_iterator(
        m,
        num_trans,
        geometry.transducers().flat_map(|trans| {
            foci.iter().map(move |fp| {
                propagate::<Sphere>(
                    trans.position(),
                    &trans.z_direction(),
                    attenuation,
                    trans.wavenumber(sound_speed),
                    fp,
                )
            })
        }),
    )
}
