/*
 * File: device.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{
    is_missing_transducer, NUM_TRANS_IN_UNIT, NUM_TRANS_X, NUM_TRANS_Y, TRANS_SPACING_MM,
};

use super::{Matrix3, Matrix4, Quaternion, Transducer, UnitQuaternion, Vector3, Vector4};

pub struct Device<T: Transducer> {
    transducers: Vec<T>,
    origin: Vector3,
    trans_inv: Matrix3,
    rotation: UnitQuaternion,
}

impl<T: Transducer> Device<T> {
    fn get_direction(dir: Vector3, rotation: UnitQuaternion) -> Vector3 {
        let dir: UnitQuaternion = UnitQuaternion::from_quaternion(Quaternion::from_imag(dir));
        (rotation * dir * rotation.conjugate()).imag().normalize()
    }

    pub fn local_position(&self, global_position: &Vector3) -> Vector3 {
        self.trans_inv * (global_position - self.origin)
    }

    pub fn transducers(&self) -> &[T] {
        &self.transducers
    }

    pub fn transducers_mut(&mut self) -> &mut [T] {
        &mut self.transducers
    }

    pub fn center(&self) -> Vector3 {
        let sum: Vector3 = self.transducers().iter().map(|t| t.position()).sum();
        sum / self.transducers.len() as f64
    }

    pub fn rotation(&self) -> UnitQuaternion {
        self.rotation
    }
}

impl<T: Transducer> Device<T> {
    pub fn new(id: usize, position: Vector3, rotation: UnitQuaternion) -> Self {
        let rot_mat: Matrix4 = From::from(rotation);
        let trans_mat = rot_mat.append_translation(&position);
        let x_direction = Self::get_direction(Vector3::x(), rotation);
        let y_direction = Self::get_direction(Vector3::y(), rotation);
        let z_direction = Self::get_direction(Vector3::z(), rotation);

        let transducers: Vec<T> = itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
            .filter(|&(y, x)| !is_missing_transducer(x, y))
            .map(|(y, x)| {
                Vector4::new(
                    x as f64 * TRANS_SPACING_MM,
                    y as f64 * TRANS_SPACING_MM,
                    0.,
                    1.,
                )
            })
            .map(|p| trans_mat * p)
            .zip(id * NUM_TRANS_IN_UNIT..)
            .map(|(p, i)| {
                T::new(
                    i,
                    Vector3::new(p.x, p.y, p.z),
                    x_direction,
                    y_direction,
                    z_direction,
                )
            })
            .collect();

        let origin = *transducers[0].position();
        let trans_inv = Matrix3::from_columns(&[x_direction, y_direction, z_direction]).transpose();

        Self {
            transducers,
            origin,
            trans_inv,
            rotation,
        }
    }
}
