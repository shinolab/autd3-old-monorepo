/*
 * File: autd3_device.rs
 * Project: src
 * Created Date: 06/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::geometry::{Device, Matrix4, Transducer, UnitQuaternion, Vector3, Vector4};
use autd3_driver::float;
use num::FromPrimitive;

pub const NUM_TRANS_IN_UNIT: usize = 249;
pub const NUM_TRANS_X: usize = 18;
pub const NUM_TRANS_Y: usize = 14;
pub const TRANS_SPACING_MM: float = 10.16;
pub const DEVICE_WIDTH: float = 192.0;
pub const DEVICE_HEIGHT: float = 151.4;

pub struct AUTD3 {
    position: Vector3,
    rotation: UnitQuaternion,
}

impl AUTD3 {
    /// Create AUTD3 device
    ///
    /// # Arguments
    ///
    /// * `pos` - Global position of AUTD.
    /// * `rot` - ZYZ Euler angles.
    ///
    pub fn new(position: Vector3, euler_angles: Vector3) -> Self {
        let q = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), euler_angles.x)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), euler_angles.y)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), euler_angles.z);
        Self::new_with_quaternion(position, q)
    }

    /// Create AUTD3 device
    ///
    /// # Arguments
    ///
    /// * `pos` - Global position of AUTD.
    /// * `rot` - Rotation quaternion.
    ///
    pub fn new_with_quaternion(position: Vector3, rotation: UnitQuaternion) -> Self {
        Self { position, rotation }
    }

    pub fn is_missing_transducer<T1, T2>(x: T1, y: T2) -> bool
    where
        T1: FromPrimitive + PartialEq<T1>,
        T2: FromPrimitive + PartialEq<T2>,
    {
        y == FromPrimitive::from_u8(1).unwrap()
            && (x == FromPrimitive::from_u8(1).unwrap()
                || x == FromPrimitive::from_u8(2).unwrap()
                || x == FromPrimitive::from_u8(16).unwrap())
    }
}

impl<T: Transducer> Device<T> for AUTD3 {
    fn get_transducers(&self, start_id: usize) -> Vec<T> {
        let rot_mat: Matrix4 = From::from(self.rotation);
        let trans_mat = rot_mat.append_translation(&self.position);
        itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
            .filter(|&(y, x)| !Self::is_missing_transducer(x, y))
            .map(|(y, x)| {
                Vector4::new(
                    x as float * TRANS_SPACING_MM,
                    y as float * TRANS_SPACING_MM,
                    0.,
                    1.,
                )
            })
            .map(|p| trans_mat * p)
            .zip(start_id..)
            .map(|(p, i)| T::new(i, Vector3::new(p.x, p.y, p.z), self.rotation))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::LegacyTransducer;

    use super::*;

    #[test]
    fn autd3_device() {
        let dev = AUTD3::new(Vector3::zeros(), Vector3::zeros());
        assert_eq!(
            Device::<LegacyTransducer>::get_transducers(&dev, 0).len(),
            249
        );
    }
}
