/*
 * File: control_point.rs
 * Project: focus
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::geometry::Vector3;

/// Control point for FocusSTM
#[derive(Clone, Copy)]
pub struct ControlPoint {
    /// Focal point
    point: Vector3,
    /// Duty shift
    /// Duty ratio of ultrasound will be `50% >> shift`.
    /// If `shift` is 0, duty ratio is 50%, which means the amplitude is the maximum.
    shift: u8,
}

impl ControlPoint {
    /// constructor (shift is 0)
    ///
    /// # Arguments
    ///
    /// * `point` - focal point
    ///
    pub fn new(point: Vector3) -> Self {
        Self { point, shift: 0 }
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `point` - focal point
    /// * `shift` - duty shift
    ///
    pub fn with_shift(self, shift: u8) -> Self {
        Self { shift, ..self }
    }

    pub fn point(&self) -> &Vector3 {
        &self.point
    }

    pub fn shift(&self) -> u8 {
        self.shift
    }
}

impl From<Vector3> for ControlPoint {
    fn from(point: Vector3) -> Self {
        Self::new(point)
    }
}

impl From<(Vector3, u8)> for ControlPoint {
    fn from((point, shift): (Vector3, u8)) -> Self {
        Self::new(point).with_shift(shift)
    }
}

impl From<&Vector3> for ControlPoint {
    fn from(point: &Vector3) -> Self {
        Self::new(*point)
    }
}

impl From<&(Vector3, u8)> for ControlPoint {
    fn from((point, shift): &(Vector3, u8)) -> Self {
        Self::new(*point).with_shift(*shift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_point() {
        let c = ControlPoint::from(Vector3::new(1., 2., 3.));
        assert_eq!(c.point().x, 1.);
        assert_eq!(c.point().y, 2.);
        assert_eq!(c.point().z, 3.);
        assert_eq!(c.shift(), 0);

        let c = ControlPoint::from((Vector3::new(1., 2., 3.), 4));
        assert_eq!(c.point().x, 1.);
        assert_eq!(c.point().y, 2.);
        assert_eq!(c.point().z, 3.);
        assert_eq!(c.shift(), 4);

        let c = ControlPoint::from(&Vector3::new(1., 2., 3.));
        assert_eq!(c.point().x, 1.);
        assert_eq!(c.point().y, 2.);
        assert_eq!(c.point().z, 3.);
        assert_eq!(c.shift(), 0);

        let c = ControlPoint::from(&(Vector3::new(1., 2., 3.), 4));
        assert_eq!(c.point().x, 1.);
        assert_eq!(c.point().y, 2.);
        assert_eq!(c.point().z, 3.);
        assert_eq!(c.shift(), 4);

        let cc = Clone::clone(&c);
        assert_eq!(cc.point().x, 1.);
        assert_eq!(cc.point().y, 2.);
        assert_eq!(cc.point().z, 3.);
        assert_eq!(cc.shift(), 4);
    }
}
