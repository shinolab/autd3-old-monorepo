/*
 * File: rotation.rs
 * Project: geometry
 * Created Date: 26/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::defined::float;

use super::{UnitQuaternion, Vector3};

pub struct Deg;
pub struct Rad;

#[derive(Clone, Copy)]
pub enum Angle {
    Deg(float),
    Rad(float),
}

impl Angle {
    fn to_radians(self) -> float {
        match self {
            Self::Deg(a) => a.to_radians(),
            Self::Rad(a) => a,
        }
    }
}

impl std::ops::Mul<Deg> for float {
    type Output = Angle;

    fn mul(self, _rhs: Deg) -> Self::Output {
        Self::Output::Deg(self)
    }
}

impl std::ops::Mul<Rad> for float {
    type Output = Angle;

    fn mul(self, _rhs: Rad) -> Self::Output {
        Self::Output::Rad(self)
    }
}

pub enum EulerAngle {
    ZYZ(Angle, Angle, Angle),
}

impl From<EulerAngle> for UnitQuaternion {
    fn from(angle: EulerAngle) -> Self {
        match angle {
            EulerAngle::ZYZ(z1, y, z2) => {
                UnitQuaternion::from_axis_angle(&Vector3::z_axis(), z1.to_radians())
                    * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), y.to_radians())
                    * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), z2.to_radians())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::defined::PI;

    macro_rules! assert_approx_eq_vec3 {
        ($a:expr, $b:expr $(,)?) => {
            assert_approx_eq::assert_approx_eq!($a.x, $b.x);
            assert_approx_eq::assert_approx_eq!($a.y, $b.y);
            assert_approx_eq::assert_approx_eq!($a.z, $b.z);
        };
    }

    #[test]
    fn angle_clone() {
        let a = 90.0 * Deg;
        let b = a.clone();
        assert_eq!(a.to_radians(), b.to_radians());
    }

    #[test]
    fn to_radians() {
        assert_approx_eq::assert_approx_eq!((90.0 * Deg).to_radians(), PI / 2.0);
        assert_approx_eq::assert_approx_eq!((PI / 2.0 * Rad).to_radians(), PI / 2.0);
    }

    #[test]
    fn test_rotation() {
        let rot: UnitQuaternion = EulerAngle::ZYZ(90.0 * Deg, 0.0 * Deg, 0.0 * Deg).into();
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::x()), Vector3::y());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::y()), -Vector3::x());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::z()), Vector3::z());

        let rot: UnitQuaternion = EulerAngle::ZYZ(0.0 * Deg, 90.0 * Deg, 0.0 * Deg).into();
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::x()), -Vector3::z());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::y()), Vector3::y());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::z()), Vector3::x());

        let rot: UnitQuaternion = EulerAngle::ZYZ(0.0 * Deg, 0.0 * Deg, 90.0 * Deg).into();
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::x()), Vector3::y());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::y()), -Vector3::x());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::z()), Vector3::z());

        let rot: UnitQuaternion = EulerAngle::ZYZ(0.0 * Deg, 90.0 * Deg, 90.0 * Deg).into();
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::x()), Vector3::y());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::y()), Vector3::z());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::z()), Vector3::x());

        let rot: UnitQuaternion = EulerAngle::ZYZ(90.0 * Deg, 90.0 * Deg, 0.0 * Deg).into();
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::x()), -Vector3::z());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::y()), -Vector3::x());
        assert_approx_eq_vec3!(rot.transform_vector(&Vector3::z()), Vector3::y());
    }
}
