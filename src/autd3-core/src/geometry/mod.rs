/*
 * File: mod.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

mod advanced_phase_transducer;
mod advanced_transducer;
mod device;
mod legacy_transducer;
mod transducer;

use autd3_driver::{float, METER};
use std::marker::PhantomData;

pub type Vector3 = nalgebra::Vector3<float>;
pub type UnitVector3 = nalgebra::UnitVector3<float>;
pub type Vector4 = nalgebra::Vector4<float>;
pub type Quaternion = nalgebra::Quaternion<float>;
pub type UnitQuaternion = nalgebra::UnitQuaternion<float>;
pub type Matrix3 = nalgebra::Matrix3<float>;
pub type Matrix4 = nalgebra::Matrix4<float>;
pub type Affine = nalgebra::Affine3<float>;

pub use advanced_phase_transducer::*;
pub use advanced_transducer::*;
pub use device::*;
pub use legacy_transducer::*;
use std::ops::{Index, IndexMut};
pub use transducer::*;

use crate::error::AUTDInternalError;

#[derive(Default)]
pub struct Geometry<T: Transducer> {
    pub(crate) transducers: Vec<T>,
    device_map: Vec<usize>,
    pub sound_speed: float,
    pub attenuation: float,
}

impl<T: Transducer> Geometry<T> {
    fn new(
        transducers: Vec<T>,
        device_map: Vec<usize>,
        sound_speed: float,
        attenuation: float,
    ) -> Result<Geometry<T>, AUTDInternalError> {
        for &transducers in &device_map {
            if transducers > 256 {
                return Err(AUTDInternalError::TransducersNumInDeviceOutOfRange);
            }
        }

        Ok(Geometry {
            transducers,
            device_map,
            sound_speed,
            attenuation,
        })
    }

    pub fn num_devices(&self) -> usize {
        self.device_map.len()
    }

    pub fn num_transducers(&self) -> usize {
        self.transducers.len()
    }

    pub fn transducers(&self) -> std::slice::Iter<'_, T> {
        self.transducers.iter()
    }

    pub fn transducers_of(
        &self,
        idx: usize,
    ) -> std::iter::Take<std::iter::Skip<std::slice::Iter<'_, T>>> {
        let start_idx: usize = self.device_map.iter().take(idx).sum();
        self.transducers
            .iter()
            .skip(start_idx)
            .take(self.device_map[idx])
    }

    pub fn transducers_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.transducers.iter_mut()
    }

    pub fn transducers_mut_of(
        &mut self,
        idx: usize,
    ) -> std::iter::Take<std::iter::Skip<std::slice::IterMut<'_, T>>> {
        let start_idx: usize = self.device_map.iter().take(idx).sum();
        self.transducers
            .iter_mut()
            .skip(start_idx)
            .take(self.device_map[idx])
    }

    pub fn center(&self) -> Vector3 {
        self.transducers
            .iter()
            .map(|d| d.position())
            .sum::<Vector3>()
            / self.transducers.len() as float
    }

    pub fn center_of(&self, idx: usize) -> Vector3 {
        self.transducers_of(idx)
            .map(|d| d.position())
            .sum::<Vector3>()
            / self.device_map[idx] as float
    }

    pub fn translate(&mut self, t: Vector3) {
        self.affine(t, UnitQuaternion::identity());
    }

    pub fn rotate(&mut self, r: UnitQuaternion) {
        self.affine(Vector3::zeros(), r);
    }

    pub fn affine(&mut self, t: Vector3, r: UnitQuaternion) {
        self.transducers.iter_mut().for_each(|tr| tr.affine(t, r));
    }

    pub fn translate_of(&mut self, idx: usize, t: Vector3) {
        self.affine_of(idx, t, UnitQuaternion::identity());
    }

    pub fn rotate_of(&mut self, idx: usize, r: UnitQuaternion) {
        self.affine_of(idx, Vector3::zeros(), r);
    }

    pub fn affine_of(&mut self, idx: usize, t: Vector3, r: UnitQuaternion) {
        self.transducers_mut_of(idx).for_each(|tr| tr.affine(t, r));
    }

    pub fn device_map(&self) -> &[usize] {
        &self.device_map
    }

    pub fn set_sound_speed_from_temp(&mut self, temp: float) {
        self.set_sound_speed_from_temp_with(temp, 1.4, 8.314_463, 28.9647e-3);
    }

    pub fn set_sound_speed_from_temp_with(&mut self, temp: float, k: float, r: float, m: float) {
        self.sound_speed = (k * r * (273.15 + temp) / m).sqrt() * METER;
    }
}

impl<T: Transducer> Index<usize> for Geometry<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.transducers[idx]
    }
}

impl<T: Transducer> IndexMut<usize> for Geometry<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.transducers[idx]
    }
}

impl<'a, T: Transducer> IntoIterator for &'a Geometry<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> std::slice::Iter<'a, T> {
        self.transducers()
    }
}

impl<'a, T: Transducer> IntoIterator for &'a mut Geometry<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> std::slice::IterMut<'a, T> {
        self.transducers.iter_mut()
    }
}

impl Geometry<LegacyTransducer> {
    pub fn builder() -> GeometryBuilder<LegacyTransducer> {
        GeometryBuilder::<LegacyTransducer>::new()
    }
}

pub struct GeometryBuilder<T: Transducer> {
    attenuation: float,
    sound_speed: float,
    transducers: Vec<(usize, Vector3, UnitQuaternion)>,
    device_map: Vec<usize>,
    phantom: PhantomData<T>,
}

impl<T: Transducer> Default for GeometryBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Transducer> GeometryBuilder<T> {
    pub fn new() -> GeometryBuilder<T> {
        GeometryBuilder::<T> {
            attenuation: 0.0,
            sound_speed: 340.0 * METER,
            transducers: vec![],
            device_map: vec![],
            phantom: PhantomData,
        }
    }

    pub fn attenuation(&mut self, attenuation: float) -> &mut Self {
        self.attenuation = attenuation;
        self
    }

    pub fn sound_speed(&mut self, sound_speed: float) -> &mut Self {
        self.sound_speed = sound_speed;
        self
    }

    pub fn add_device<D: Device>(&mut self, dev: D) -> &mut Self {
        let id = self.transducers.len();
        let mut t = dev.get_transducers(id);
        self.device_map.push(t.len());
        self.transducers.append(&mut t);
        self
    }

    pub fn build(&mut self) -> Result<Geometry<T>, AUTDInternalError> {
        Geometry::<T>::new(
            self.transducers
                .iter()
                .map(|&(id, pos, rot)| T::new(id, pos, rot))
                .collect(),
            self.device_map.clone(),
            self.sound_speed,
            self.attenuation,
        )
    }
}

impl GeometryBuilder<LegacyTransducer> {
    pub fn advanced(&mut self) -> &mut GeometryBuilder<AdvancedTransducer> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn advanced_phase(&mut self) -> &mut GeometryBuilder<AdvancedPhaseTransducer> {
        unsafe { std::mem::transmute(self) }
    }
}

impl GeometryBuilder<AdvancedTransducer> {
    pub fn legacy(&mut self) -> &mut GeometryBuilder<LegacyTransducer> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn advanced_phase(&mut self) -> &mut GeometryBuilder<AdvancedPhaseTransducer> {
        unsafe { std::mem::transmute(self) }
    }
}

impl GeometryBuilder<AdvancedPhaseTransducer> {
    pub fn advanced(&mut self) -> &mut GeometryBuilder<AdvancedTransducer> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn legacy(&mut self) -> &mut GeometryBuilder<LegacyTransducer> {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use autd3_driver::PI;

    use crate::autd3_device::{AUTD3, NUM_TRANS_IN_UNIT};

    use super::*;

    macro_rules! assert_vec3_approx_eq {
        ($a:expr, $b:expr) => {
            assert_approx_eq!($a.x, $b.x, 1e-3);
            assert_approx_eq!($a.y, $b.y, 1e-3);
            assert_approx_eq!($a.z, $b.z, 1e-3);
        };
    }

    #[test]
    fn num_transducers() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();
        assert_eq!(geometry.num_transducers(), 249);

        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();
        assert_eq!(geometry.num_transducers(), 249 * 2);
    }

    #[test]
    fn center() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(10., 20., 30.), Vector3::zeros()))
            .build()
            .unwrap();

        let mut expected = Vector3::zeros();
        for i in 0..18 {
            for j in 0..14 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                expected +=
                    10.16 * Vector3::new(i as float, j as float, 0.) + Vector3::new(10., 20., 30.);
            }
        }
        expected /= 249.;

        assert_vec3_approx_eq!(geometry.center(), expected);
    }

    #[test]
    fn center_of() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(10., 20., 30.), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::new(40., 50., 60.), Vector3::zeros()))
            .build()
            .unwrap();

        let mut expected_0 = Vector3::zeros();
        for i in 0..18 {
            for j in 0..14 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                expected_0 +=
                    10.16 * Vector3::new(i as float, j as float, 0.) + Vector3::new(10., 20., 30.);
            }
        }
        expected_0 /= 249.;
        assert_vec3_approx_eq!(geometry.center_of(0), expected_0);

        let mut expected_1 = Vector3::zeros();
        for i in 0..18 {
            for j in 0..14 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                expected_1 +=
                    10.16 * Vector3::new(i as float, j as float, 0.) + Vector3::new(40., 50., 60.);
            }
        }
        expected_1 /= 249.;
        assert_vec3_approx_eq!(geometry.center_of(1), expected_1);
    }

    #[test]
    fn add_device() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(10., 20., 30.), Vector3::zeros()))
            .add_device(AUTD3::new(
                Vector3::new(0., 0., 0.),
                Vector3::new(PI, PI, 0.),
            ))
            .add_device(AUTD3::new(
                Vector3::new(0., 0., 0.),
                Vector3::new(0., PI, 0.),
            ))
            .add_device(AUTD3::new(
                Vector3::new(0., 0., 0.),
                Vector3::new(PI, 0., 0.),
            ))
            .add_device(AUTD3::new(
                Vector3::new(40., 60., 50.),
                Vector3::new(0., 0., PI / 2.),
            ))
            .build()
            .unwrap();

        let origin = Vector3::new(0., 0., 0.);
        let right_bottom = Vector3::new(10.16 * 17., 0., 0.);
        let left_top = Vector3::new(0., 10.16 * 13., 0.);

        assert_vec3_approx_eq!(geometry[0].position(), Vector3::new(10., 20., 30.) + origin);
        assert_vec3_approx_eq!(
            geometry[17].position(),
            Vector3::new(10., 20., 30.) + right_bottom
        );
        assert_vec3_approx_eq!(
            geometry[231].position(),
            Vector3::new(10., 20., 30.) + left_top
        );
        assert_vec3_approx_eq!(
            geometry[248].position(),
            Vector3::new(10., 20., 30.) + right_bottom + left_top
        );

        assert_vec3_approx_eq!(
            geometry[NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + origin
        );
        assert_vec3_approx_eq!(
            geometry[17 + NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + right_bottom
        );
        assert_vec3_approx_eq!(
            geometry[231 + NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - left_top
        );
        assert_vec3_approx_eq!(
            geometry[248 + NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + right_bottom - left_top
        );

        assert_vec3_approx_eq!(
            geometry[2 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + origin
        );
        assert_vec3_approx_eq!(
            geometry[17 + 2 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - right_bottom
        );
        assert_vec3_approx_eq!(
            geometry[231 + 2 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + left_top
        );
        assert_vec3_approx_eq!(
            geometry[248 + 2 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - right_bottom + left_top
        );

        assert_vec3_approx_eq!(
            geometry[3 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + origin
        );
        assert_vec3_approx_eq!(
            geometry[17 + 3 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - right_bottom
        );
        assert_vec3_approx_eq!(
            geometry[231 + 3 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - left_top
        );
        assert_vec3_approx_eq!(
            geometry[248 + 3 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - right_bottom - left_top
        );

        assert_vec3_approx_eq!(
            geometry[4 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(40., 60., 50.) + origin
        );
        assert_vec3_approx_eq!(
            geometry[17 + 4 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(40., 60., 50.) + Vector3::new(0., 10.16 * 17., 0.)
        );
        assert_vec3_approx_eq!(
            geometry[231 + 4 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(40., 60., 50.) - Vector3::new(10.16 * 13., 0., 0.)
        );
        assert_vec3_approx_eq!(
            geometry[248 + 4 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(40., 60., 50.) + Vector3::new(0., 10.16 * 17., 0.)
                - Vector3::new(10.16 * 13., 0., 0.)
        );
    }

    #[test]
    fn add_device_quaternion() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new_with_quaternion(
                Vector3::new(10., 20., 30.),
                UnitQuaternion::identity(),
            ))
            .add_device(AUTD3::new_with_quaternion(
                Vector3::new(0., 0., 0.),
                UnitQuaternion::identity()
                    * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), PI),
            ))
            .add_device(AUTD3::new_with_quaternion(
                Vector3::new(0., 0., 0.),
                UnitQuaternion::identity()
                    * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), PI),
            ))
            .add_device(AUTD3::new_with_quaternion(
                Vector3::new(0., 0., 0.),
                UnitQuaternion::identity()
                    * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI),
            ))
            .add_device(AUTD3::new_with_quaternion(
                Vector3::new(40., 60., 50.),
                UnitQuaternion::identity()
                    * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.),
            ))
            .build()
            .unwrap();

        let origin = Vector3::new(0., 0., 0.);
        let right_bottom = Vector3::new(10.16 * 17., 0., 0.);
        let left_top = Vector3::new(0., 10.16 * 13., 0.);

        assert_vec3_approx_eq!(geometry[0].position(), Vector3::new(10., 20., 30.) + origin);
        assert_vec3_approx_eq!(
            geometry[17].position(),
            Vector3::new(10., 20., 30.) + right_bottom
        );
        assert_vec3_approx_eq!(
            geometry[231].position(),
            Vector3::new(10., 20., 30.) + left_top
        );
        assert_vec3_approx_eq!(
            geometry[248].position(),
            Vector3::new(10., 20., 30.) + right_bottom + left_top
        );

        assert_vec3_approx_eq!(
            geometry[NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + origin
        );
        assert_vec3_approx_eq!(
            geometry[17 + NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + right_bottom
        );
        assert_vec3_approx_eq!(
            geometry[231 + NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - left_top
        );
        assert_vec3_approx_eq!(
            geometry[248 + NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + right_bottom - left_top
        );

        assert_vec3_approx_eq!(
            geometry[2 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + origin
        );
        assert_vec3_approx_eq!(
            geometry[17 + 2 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - right_bottom
        );
        assert_vec3_approx_eq!(
            geometry[231 + 2 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + left_top
        );
        assert_vec3_approx_eq!(
            geometry[248 + 2 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - right_bottom + left_top
        );

        assert_vec3_approx_eq!(
            geometry[3 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) + origin
        );
        assert_vec3_approx_eq!(
            geometry[17 + 3 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - right_bottom
        );
        assert_vec3_approx_eq!(
            geometry[231 + 3 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - left_top
        );
        assert_vec3_approx_eq!(
            geometry[248 + 3 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(0., 0., 0.) - right_bottom - left_top
        );

        assert_vec3_approx_eq!(
            geometry[4 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(40., 60., 50.) + origin
        );
        assert_vec3_approx_eq!(
            geometry[17 + 4 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(40., 60., 50.) + Vector3::new(0., 10.16 * 17., 0.)
        );
        assert_vec3_approx_eq!(
            geometry[231 + 4 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(40., 60., 50.) - Vector3::new(10.16 * 13., 0., 0.)
        );
        assert_vec3_approx_eq!(
            geometry[248 + 4 * NUM_TRANS_IN_UNIT].position(),
            Vector3::new(40., 60., 50.) + Vector3::new(0., 10.16 * 17., 0.)
                - Vector3::new(10.16 * 13., 0., 0.)
        );
    }

    #[test]
    fn translate() {
        let mut geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::new(10., 20., 30.), Vector3::zeros()))
            .build()
            .unwrap();

        let t = Vector3::new(40., 50., 60.);
        geometry.translate(t);

        let mut idx = 0;
        for j in 0..14 {
            for i in 0..18 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                let expect = 10.16 * Vector3::new(i as float, j as float, 0.) + t;
                assert_vec3_approx_eq!(expect, geometry[idx].position());
                idx += 1;
            }
        }
        for j in 0..14 {
            for i in 0..18 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                let expect = 10.16 * Vector3::new(i as float, j as float, 0.)
                    + Vector3::new(10., 20., 30.)
                    + t;
                assert_vec3_approx_eq!(expect, geometry[idx].position());
                idx += 1;
            }
        }
    }

    #[test]
    fn translate_of() {
        let mut geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::new(10., 20., 30.), Vector3::zeros()))
            .build()
            .unwrap();

        let t = Vector3::new(40., 50., 60.);
        geometry.translate_of(0, t);

        let mut idx = 0;
        for j in 0..14 {
            for i in 0..18 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                let expect = 10.16 * Vector3::new(i as float, j as float, 0.) + t;
                assert_vec3_approx_eq!(expect, geometry[idx].position());
                idx += 1;
            }
        }
        for j in 0..14 {
            for i in 0..18 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                let expect =
                    10.16 * Vector3::new(i as float, j as float, 0.) + Vector3::new(10., 20., 30.);
                assert_vec3_approx_eq!(expect, geometry[idx].position());
                idx += 1;
            }
        }

        geometry.translate_of(1, t);

        let mut idx = 0;
        for j in 0..14 {
            for i in 0..18 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                let expect = 10.16 * Vector3::new(i as float, j as float, 0.) + t;
                assert_vec3_approx_eq!(expect, geometry[idx].position());
                idx += 1;
            }
        }
        for j in 0..14 {
            for i in 0..18 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                let expect = 10.16 * Vector3::new(i as float, j as float, 0.)
                    + Vector3::new(10., 20., 30.)
                    + t;
                assert_vec3_approx_eq!(expect, geometry[idx].position());
                idx += 1;
            }
        }
    }

    #[test]
    fn rotate() {
        let mut geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .build()
            .unwrap();

        let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.);
        geometry.rotate(rot);
        let expect_x = Vector3::new(0., 1., 0.);
        let expect_y = Vector3::new(-1., 0., 0.);
        let expect_z = Vector3::new(0., 0., 1.);
        for tr in &geometry {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        }

        let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), PI / 2.)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 0.);
        geometry.rotate(rot);
        let expect_x = Vector3::new(0., 0., 1.);
        let expect_y = Vector3::new(-1., 0., 0.);
        let expect_z = Vector3::new(0., -1., 0.);
        for tr in &geometry {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        }
    }

    #[test]
    fn rotate_of() {
        let mut geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .build()
            .unwrap();

        let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.);
        geometry.rotate_of(0, rot);
        let expect_x = Vector3::new(0., 1., 0.);
        let expect_y = Vector3::new(-1., 0., 0.);
        let expect_z = Vector3::new(0., 0., 1.);
        for tr in geometry.transducers_of(0) {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        }
        for tr in geometry.transducers_of(1) {
            assert_vec3_approx_eq!(Vector3::x_axis(), tr.x_direction());
            assert_vec3_approx_eq!(Vector3::y_axis(), tr.y_direction());
            assert_vec3_approx_eq!(Vector3::z_axis(), tr.z_direction());
        }

        geometry.rotate_of(1, rot);
        for tr in &geometry {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        }
    }

    #[test]
    fn affine() {
        let mut geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::new(10., 20., 30.), Vector3::zeros()))
            .build()
            .unwrap();

        let t = Vector3::new(40., 50., 60.);
        let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.);
        geometry.affine(t, rot);

        let expect_x = Vector3::new(0., 1., 0.);
        let expect_y = Vector3::new(-1., 0., 0.);
        let expect_z = Vector3::new(0., 0., 1.);
        for tr in &geometry {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        }
        let mut idx = 0;
        for j in 0..14 {
            for i in 0..18 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                let expect = 10.16 * Vector3::new(-j as float, i as float, 0.) + t;
                assert_vec3_approx_eq!(expect, geometry[idx].position());
                idx += 1;
            }
        }
        for j in 0..14 {
            for i in 0..18 {
                if AUTD3::is_missing_transducer(i, j) {
                    continue;
                }
                let expect = 10.16 * Vector3::new(-j as float, i as float, 0.)
                    + Vector3::new(-20., 10., 30.)
                    + t;
                assert_vec3_approx_eq!(expect, geometry[idx].position());
                idx += 1;
            }
        }
    }
}
