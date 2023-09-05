/*
 * File: geometry.rs
 * Project: src
 * Created Date: 05/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::ops::{Deref, DerefMut};

use autd3_driver::{defined::float, geometry::*};

#[derive(Default)]
pub struct Geometry<T: Transducer> {
    pub(crate) devices: Vec<Device<T>>,
}

impl<T: Transducer> Geometry<T> {
    #[doc(hidden)]
    pub fn new(devices: Vec<Device<T>>) -> Geometry<T> {
        Self { devices }
    }

    /// Get the number of devices
    pub fn num_devices(&self) -> usize {
        self.devices.len()
    }

    /// Get the number of total transducers
    pub fn num_transducers(&self) -> usize {
        self.devices.iter().map(|dev| dev.num_transducers()).sum()
    }

    /// Get center position of all devices
    pub fn center(&self) -> Vector3 {
        self.devices.iter().map(|d| d.center()).sum::<Vector3>() / self.devices.len() as float
    }

    /// Translate all devices
    pub fn translate(&mut self, t: Vector3) {
        self.affine(t, UnitQuaternion::identity());
    }

    /// Rorate all devices
    pub fn rotate(&mut self, r: UnitQuaternion) {
        self.affine(Vector3::zeros(), r);
    }

    /// Affine transform all devices
    pub fn affine(&mut self, t: Vector3, r: UnitQuaternion) {
        self.devices.iter_mut().for_each(|dev| dev.affine(t, r));
    }

    /// Set speed of sound of all devices from temperature
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
        self.devices
            .iter_mut()
            .for_each(|dev| dev.set_sound_speed_from_temp_with(temp, k, r, m));
    }
}

impl<T: Transducer> Deref for Geometry<T> {
    type Target = [Device<T>];

    fn deref(&self) -> &Self::Target {
        &self.devices
    }
}

impl<T: Transducer> DerefMut for Geometry<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.devices
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    macro_rules! assert_approx_eq_vec3 {
        ($a:expr, $b:expr) => {
            assert_approx_eq::assert_approx_eq!($a.x, $b.x, 1e-3);
            assert_approx_eq::assert_approx_eq!($a.y, $b.y, 1e-3);
            assert_approx_eq::assert_approx_eq!($a.z, $b.z, 1e-3);
        };
    }

    fn create_device<T: Transducer>(idx: usize, n: usize) -> Device<T> {
        Device::new(
            idx,
            (0..n)
                .map(|i| T::new(i, Vector3::zeros(), UnitQuaternion::identity()))
                .collect(),
        )
    }

    #[test]
    fn geometry_num_devices() {
        let geometry = Geometry::new(vec![create_device::<LegacyTransducer>(0, 249)]);
        assert_eq!(geometry.num_devices(), 1);

        let geometry = Geometry::new(vec![
            create_device::<LegacyTransducer>(0, 249),
            create_device::<LegacyTransducer>(0, 249),
        ]);
        assert_eq!(geometry.num_devices(), 2);
    }

    #[test]
    fn geometry_num_transducers() {
        let geometry = Geometry::new(vec![create_device::<LegacyTransducer>(0, 249)]);
        assert_eq!(geometry.num_transducers(), 249);

        let geometry = Geometry::new(vec![
            create_device::<LegacyTransducer>(0, 249),
            create_device::<LegacyTransducer>(0, 249),
        ]);
        assert_eq!(geometry.num_transducers(), 249 * 2);
    }

    #[test]
    fn center() {
        let transducers = itertools::iproduct!((0..18), (0..14))
            .enumerate()
            .map(|(i, (y, x))| {
                LegacyTransducer::new(
                    i,
                    10.16 * Vector3::new(x as float, y as float, 0.),
                    UnitQuaternion::identity(),
                )
            })
            .collect::<Vec<_>>();
        let device0 = Device::new(0, transducers);

        let transducers = itertools::iproduct!((0..18), (0..14))
            .enumerate()
            .map(|(i, (y, x))| {
                LegacyTransducer::new(
                    i,
                    10.16 * Vector3::new(x as float, y as float, 0.) + Vector3::new(10., 20., 30.),
                    UnitQuaternion::identity(),
                )
            })
            .collect::<Vec<_>>();
        let device1 = Device::new(1, transducers);

        let geometry = Geometry::new(vec![device0, device1]);

        let expect = geometry.iter().map(|dev| dev.center()).sum::<Vector3>() / 2.0;

        assert_approx_eq_vec3!(geometry.center(), expect);
    }
}
