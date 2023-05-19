/*
 * File: legacy_transducer.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use super::{Matrix4, Transducer, UnitQuaternion, Vector3, Vector4};

use autd3_driver::float;

pub struct LegacyTransducer {
    idx: usize,
    pos: Vector3,
    rot: UnitQuaternion,
    mod_delay: u16,
}

impl Transducer for LegacyTransducer {
    fn new(idx: usize, pos: Vector3, rot: UnitQuaternion) -> Self {
        Self {
            idx,
            pos,
            rot,
            mod_delay: 0,
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

    fn idx(&self) -> usize {
        self.idx
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

    fn get_direction(dir: Vector3, rotation: &UnitQuaternion) -> Vector3 {
        let dir: UnitQuaternion =
            UnitQuaternion::from_quaternion(super::Quaternion::from_imag(dir));
        (rotation * dir * rotation.conjugate()).imag().normalize()
    }

    fn x_direction(&self) -> Vector3 {
        Self::get_direction(Vector3::x(), self.rotation())
    }

    fn y_direction(&self) -> Vector3 {
        Self::get_direction(Vector3::y(), self.rotation())
    }

    fn z_direction(&self) -> Vector3 {
        Self::get_direction(Vector3::z(), self.rotation())
    }
}
