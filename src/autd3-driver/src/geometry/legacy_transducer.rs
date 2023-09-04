/*
 * File: legacy_transducer.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use super::{Matrix4, Transducer, UnitQuaternion, Vector3, Vector4};

use crate::defined::float;

pub struct LegacyTransducer {
    local_idx: usize,
    pos: Vector3,
    rot: UnitQuaternion,
    mod_delay: u16,
    amp_filter: float,
    phase_filter: float,
}

impl Transducer for LegacyTransducer {
    fn new(local_idx: usize, pos: Vector3, rot: UnitQuaternion) -> Self {
        Self {
            local_idx,
            pos,
            rot,
            mod_delay: 0,
            amp_filter: 0.,
            phase_filter: 0.,
        }
    }

    fn affine(&mut self, t: Vector3, r: UnitQuaternion) {
        let rot_mat: Matrix4 = From::from(r);
        let trans_mat = rot_mat.append_translation(&t);
        let homo = Vector4::new(self.pos[0], self.pos[1], self.pos[2], 1.0);
        let new_pos = trans_mat * homo;
        self.pos = Vector3::new(new_pos[0], new_pos[1], new_pos[2]);
        self.rot = r * self.rot;
    }

    fn position(&self) -> &Vector3 {
        &self.pos
    }

    fn rotation(&self) -> &UnitQuaternion {
        &self.rot
    }

    fn local_idx(&self) -> usize {
        self.local_idx
    }

    fn frequency(&self) -> float {
        40e3
    }

    fn mod_delay(&self) -> u16 {
        self.mod_delay
    }

    fn set_mod_delay(&mut self, delay: u16) {
        self.mod_delay = delay;
    }

    fn amp_filter(&self) -> float {
        self.amp_filter
    }

    fn set_amp_filter(&mut self, value: float) {
        self.amp_filter = value;
    }

    fn phase_filter(&self) -> float {
        self.phase_filter
    }

    fn set_phase_filter(&mut self, value: float) {
        self.phase_filter = value;
    }

    fn cycle(&self) -> u16 {
        4096
    }
}

// #[cfg(test)]
// mod tests {
//     use assert_approx_eq::assert_approx_eq;
//     use autd3_driver::PI;

//     use super::*;

//     macro_rules! assert_vec3_approx_eq {
//         ($a:expr, $b:expr) => {
//             assert_approx_eq!($a.x, $b.x, 1e-3);
//             assert_approx_eq!($a.y, $b.y, 1e-3);
//             assert_approx_eq!($a.z, $b.z, 1e-3);
//         };
//     }

//     #[test]
//     fn affine() {
//         let mut tr = LegacyTransducer::new(0, Vector3::zeros(), UnitQuaternion::identity());

//         let t = Vector3::new(40., 50., 60.);
//         let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.)
//             * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.)
//             * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.);
//         tr.affine(t, rot);

//         let expect_x = Vector3::new(0., 1., 0.);
//         let expect_y = Vector3::new(-1., 0., 0.);
//         let expect_z = Vector3::new(0., 0., 1.);
//         assert_vec3_approx_eq!(expect_x, tr.x_direction());
//         assert_vec3_approx_eq!(expect_y, tr.y_direction());
//         assert_vec3_approx_eq!(expect_z, tr.z_direction());

//         let expect_pos = Vector3::zeros() + t;
//         assert_vec3_approx_eq!(expect_pos, tr.position());
//     }

//     #[test]
//     fn cycle() {
//         let tr = LegacyTransducer::new(0, Vector3::zeros(), UnitQuaternion::identity());
//         assert_eq!(4096, tr.cycle());
//     }

//     #[test]
//     fn freq() {
//         let tr = LegacyTransducer::new(0, Vector3::zeros(), UnitQuaternion::identity());
//         assert_approx_eq!(40e3, tr.frequency());
//     }
// }
