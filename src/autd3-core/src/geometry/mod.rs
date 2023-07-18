/*
 * File: mod.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
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

pub type Vector3 = nalgebra::Vector3<float>;
pub type UnitVector3 = nalgebra::UnitVector3<float>;
pub type Vector4 = nalgebra::Vector4<float>;
pub type Quaternion = nalgebra::Quaternion<float>;
pub type UnitQuaternion = nalgebra::UnitQuaternion<float>;
pub type Matrix3 = nalgebra::Matrix3<float>;
pub type Matrix4 = nalgebra::Matrix4<float>;
pub type Affine = nalgebra::Affine3<float>;

use std::ops::{Deref, DerefMut};

pub use advanced_phase_transducer::*;
pub use advanced_transducer::*;
pub use device::*;
pub use legacy_transducer::*;
pub use transducer::*;

use crate::error::AUTDInternalError;

#[derive(Default)]
pub struct Geometry<T: Transducer> {
    pub(crate) transducers: Vec<T>,
    device_map: Vec<usize>,
    /// Speed of sound
    pub sound_speed: float,
    /// Attenuation coefficient
    pub attenuation: float,
}

impl<T: Transducer> Geometry<T> {
    #[doc(hidden)]
    pub fn new(
        transducers: Vec<T>,
        device_map: Vec<usize>,
        sound_speed: float,
        attenuation: float,
    ) -> Result<Geometry<T>, AUTDInternalError> {
        if device_map.iter().any(|&t| t > 256) {
            return Err(AUTDInternalError::TooManyTransducers);
        }
        Ok(Geometry {
            transducers,
            device_map,
            sound_speed,
            attenuation,
        })
    }

    /// Get the number of devices
    pub fn num_devices(&self) -> usize {
        self.device_map.len()
    }

    /// Get the number of transducers
    pub fn num_transducers(&self) -> usize {
        self.transducers.len()
    }

    /// Get transducers
    pub fn transducers(&self) -> std::slice::Iter<'_, T> {
        self.transducers.iter()
    }

    /// Get transducers of specified device
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

    /// Get transducers mutably
    pub fn transducers_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.transducers.iter_mut()
    }

    /// Get transducers mutably of specified device
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

    /// Get center position of all transducers
    pub fn center(&self) -> Vector3 {
        self.transducers
            .iter()
            .map(|d| d.position())
            .sum::<Vector3>()
            / self.transducers.len() as float
    }

    /// Get center position of transducers in the specified device
    pub fn center_of(&self, idx: usize) -> Vector3 {
        self.transducers_of(idx)
            .map(|d| d.position())
            .sum::<Vector3>()
            / self.device_map[idx] as float
    }

    /// Translate all transducers
    pub fn translate(&mut self, t: Vector3) {
        self.affine(t, UnitQuaternion::identity());
    }

    /// Rorate all transducers
    pub fn rotate(&mut self, r: UnitQuaternion) {
        self.affine(Vector3::zeros(), r);
    }

    /// Affine transform all transducers
    pub fn affine(&mut self, t: Vector3, r: UnitQuaternion) {
        self.transducers.iter_mut().for_each(|tr| tr.affine(t, r));
    }

    /// Translate transducers in the specified device
    pub fn translate_of(&mut self, idx: usize, t: Vector3) {
        self.affine_of(idx, t, UnitQuaternion::identity());
    }

    /// Rotate transducers in the specified device
    pub fn rotate_of(&mut self, idx: usize, r: UnitQuaternion) {
        self.affine_of(idx, Vector3::zeros(), r);
    }

    /// Affine transform transducers in the specified device
    pub fn affine_of(&mut self, idx: usize, t: Vector3, r: UnitQuaternion) {
        self.transducers_mut_of(idx).for_each(|tr| tr.affine(t, r));
    }

    #[doc(hidden)]
    pub fn device_map(&self) -> &[usize] {
        &self.device_map
    }

    /// Set speed of sound from temperature
    /// This is equivalent to `set_sound_speed_from_temp_with(temp, 1.4, 8.314463, 28.9647e-3)`
    ///
    /// # Arguments
    ///
    /// * `temp` - Temperature in Celsius
    ///
    pub fn set_sound_speed_from_temp(&mut self, temp: float) {
        self.set_sound_speed_from_temp_with(temp, 1.4, 8.314_463, 28.9647e-3);
    }

    /// Set speed of sound from temperature with air parameter
    ///
    /// # Arguments
    ///
    /// * `temp` - Temperature in Celsius
    /// * `k` - Ratio of specific heat
    /// * `r` - Gas constant
    /// * `m` - Molar mass
    ///
    pub fn set_sound_speed_from_temp_with(&mut self, temp: float, k: float, r: float, m: float) {
        self.sound_speed = (k * r * (273.15 + temp) / m).sqrt() * METER;
    }
}

impl<T: Transducer> Deref for Geometry<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.transducers
    }
}

impl<T: Transducer> DerefMut for Geometry<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transducers
    }
}

#[cfg(test)]
pub mod tests {
    use std::marker::PhantomData;

    use assert_approx_eq::assert_approx_eq;
    use autd3_driver::PI;

    use crate::autd3_device::{AUTD3, NUM_TRANS_IN_UNIT, NUM_TRANS_X, NUM_TRANS_Y};

    use super::*;

    macro_rules! assert_vec3_approx_eq {
        ($a:expr, $b:expr) => {
            assert_approx_eq!($a.x, $b.x, 1e-3);
            assert_approx_eq!($a.y, $b.y, 1e-3);
            assert_approx_eq!($a.z, $b.z, 1e-3);
        };
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

        let expected = itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
            .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
            .fold(Vector3::zeros(), |acc, (y, x)| {
                acc + 10.16 * Vector3::new(x as float, y as float, 0.) + Vector3::new(10., 20., 30.)
            })
            / 249.;

        assert_vec3_approx_eq!(geometry.center(), expected);
    }

    #[test]
    fn center_of() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(10., 20., 30.), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::new(40., 50., 60.), Vector3::zeros()))
            .build()
            .unwrap();

        let expected_0 = itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
            .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
            .fold(Vector3::zeros(), |acc, (y, x)| {
                acc + 10.16 * Vector3::new(x as float, y as float, 0.) + Vector3::new(10., 20., 30.)
            })
            / 249.;

        assert_vec3_approx_eq!(geometry.center_of(0), expected_0);

        let expected_1 = itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
            .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
            .fold(Vector3::zeros(), |acc, (y, x)| {
                acc + 10.16 * Vector3::new(x as float, y as float, 0.) + Vector3::new(40., 50., 60.)
            })
            / 249.;

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
            .add_device(AUTD3::with_quaternion(
                Vector3::new(10., 20., 30.),
                UnitQuaternion::identity(),
            ))
            .add_device(AUTD3::with_quaternion(
                Vector3::new(0., 0., 0.),
                UnitQuaternion::identity()
                    * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), PI),
            ))
            .add_device(AUTD3::with_quaternion(
                Vector3::new(0., 0., 0.),
                UnitQuaternion::identity()
                    * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), PI),
            ))
            .add_device(AUTD3::with_quaternion(
                Vector3::new(0., 0., 0.),
                UnitQuaternion::identity()
                    * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI),
            ))
            .add_device(AUTD3::with_quaternion(
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

        itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
            .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
            .map(|(y, x)| 10.16 * Vector3::new(x as float, y as float, 0.) + t)
            .chain(
                itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
                    .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
                    .map(|(y, x)| {
                        10.16 * Vector3::new(x as float, y as float, 0.)
                            + Vector3::new(10., 20., 30.)
                            + t
                    }),
            )
            .zip(geometry.iter())
            .for_each(|(expect, tr)| {
                assert_vec3_approx_eq!(expect, tr.position());
            });
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
        itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
            .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
            .map(|(y, x)| 10.16 * Vector3::new(x as float, y as float, 0.) + t)
            .chain(
                itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
                    .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
                    .map(|(y, x)| {
                        10.16 * Vector3::new(x as float, y as float, 0.)
                            + Vector3::new(10., 20., 30.)
                    }),
            )
            .zip(geometry.iter())
            .for_each(|(expect, tr)| {
                assert_vec3_approx_eq!(expect, tr.position());
            });

        geometry.translate_of(1, t);
        itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
            .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
            .map(|(y, x)| 10.16 * Vector3::new(x as float, y as float, 0.) + t)
            .chain(
                itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
                    .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
                    .map(|(y, x)| {
                        10.16 * Vector3::new(x as float, y as float, 0.)
                            + Vector3::new(10., 20., 30.)
                            + t
                    }),
            )
            .zip(geometry.iter())
            .for_each(|(expect, tr)| {
                assert_vec3_approx_eq!(expect, tr.position());
            });
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
        geometry.iter().for_each(|tr| {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        });

        let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), PI / 2.)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 0.);
        geometry.rotate(rot);
        let expect_x = Vector3::new(0., 0., 1.);
        let expect_y = Vector3::new(-1., 0., 0.);
        let expect_z = Vector3::new(0., -1., 0.);
        geometry.iter().for_each(|tr| {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        });
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
        geometry.transducers_of(0).for_each(|tr| {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        });
        geometry.transducers_of(1).for_each(|tr| {
            assert_vec3_approx_eq!(Vector3::x_axis(), tr.x_direction());
            assert_vec3_approx_eq!(Vector3::y_axis(), tr.y_direction());
            assert_vec3_approx_eq!(Vector3::z_axis(), tr.z_direction());
        });

        geometry.rotate_of(1, rot);
        geometry.iter().for_each(|tr| {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        });
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
        geometry.iter().for_each(|tr| {
            assert_vec3_approx_eq!(expect_x, tr.x_direction());
            assert_vec3_approx_eq!(expect_y, tr.y_direction());
            assert_vec3_approx_eq!(expect_z, tr.z_direction());
        });

        itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
            .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
            .map(|(y, x)| 10.16 * Vector3::new(-(y as float), x as float, 0.) + t)
            .chain(
                itertools::iproduct!((0..NUM_TRANS_Y), (0..NUM_TRANS_X))
                    .filter(|&(y, x)| !AUTD3::is_missing_transducer(x, y))
                    .map(|(y, x)| {
                        10.16 * Vector3::new(-(y as float), x as float, 0.)
                            + Vector3::new(-20., 10., 30.)
                            + t
                    }),
            )
            .zip(geometry.iter())
            .for_each(|(expect, tr)| {
                assert_vec3_approx_eq!(expect, tr.position());
            });
    }
}
