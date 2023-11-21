/*
 * File: control_point.rs
 * Project: focus
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{common::EmitIntensity, geometry::Vector3};

/// Control point for FocusSTM
#[derive(Clone, Copy)]
pub struct ControlPoint {
    /// Focal point
    point: Vector3,
    /// Emission intensity
    intensity: EmitIntensity,
}

impl ControlPoint {
    /// constructor (shift is 0)
    ///
    /// # Arguments
    ///
    /// * `point` - focal point
    ///
    pub fn new(point: Vector3) -> Self {
        Self {
            point,
            intensity: EmitIntensity::MAX,
        }
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `point` - focal point
    /// * `intensity` - emittion intensity
    ///
    pub fn with_intensity<I: Into<EmitIntensity>>(self, intensity: I) -> Self {
        Self {
            intensity: intensity.into(),
            ..self
        }
    }

    pub fn point(&self) -> &Vector3 {
        &self.point
    }

    pub fn intensity(&self) -> EmitIntensity {
        self.intensity
    }
}

impl From<Vector3> for ControlPoint {
    fn from(point: Vector3) -> Self {
        Self::new(point)
    }
}

impl<I: Into<EmitIntensity>> From<(Vector3, I)> for ControlPoint {
    fn from((point, intensity): (Vector3, I)) -> Self {
        Self::new(point).with_intensity(intensity)
    }
}

impl From<&Vector3> for ControlPoint {
    fn from(point: &Vector3) -> Self {
        Self::new(*point)
    }
}

impl<I: Into<EmitIntensity> + Copy> From<&(Vector3, I)> for ControlPoint {
    fn from((point, intensity): &(Vector3, I)) -> Self {
        Self::new(*point).with_intensity(*intensity)
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
        assert_eq!(c.intensity(), EmitIntensity::MAX);

        let c = ControlPoint::from((Vector3::new(1., 2., 3.), 4));
        assert_eq!(c.point().x, 1.);
        assert_eq!(c.point().y, 2.);
        assert_eq!(c.point().z, 3.);
        assert_eq!(c.intensity(), EmitIntensity::new(4));

        let c = ControlPoint::from(&Vector3::new(1., 2., 3.));
        assert_eq!(c.point().x, 1.);
        assert_eq!(c.point().y, 2.);
        assert_eq!(c.point().z, 3.);
        assert_eq!(c.intensity(), EmitIntensity::MAX);

        let c = ControlPoint::from(&(Vector3::new(1., 2., 3.), EmitIntensity::new(4)));
        assert_eq!(c.point().x, 1.);
        assert_eq!(c.point().y, 2.);
        assert_eq!(c.point().z, 3.);
        assert_eq!(c.intensity(), EmitIntensity::new(4));

        let cc = Clone::clone(&c);
        assert_eq!(cc.point().x, 1.);
        assert_eq!(cc.point().y, 2.);
        assert_eq!(cc.point().z, 3.);
        assert_eq!(c.intensity(), EmitIntensity::new(4));
    }
}
