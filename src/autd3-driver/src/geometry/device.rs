/*
 * File: device.rs
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

use std::ops::{Deref, DerefMut};

use crate::defined::{float, METER};

use super::{Transducer, UnitQuaternion, Vector3};

pub struct Device<T: Transducer> {
    idx: usize,
    transducers: Vec<T>,
    pub force_fan: bool,
    pub reads_fpga_info: bool,
    pub sound_speed: float,
    pub attenuation: float,
}

impl<T: Transducer> Device<T> {
    #[doc(hidden)]
    pub fn new(idx: usize, transducers: Vec<T>) -> Self {
        Self {
            idx,
            transducers,
            force_fan: false,
            reads_fpga_info: false,
            sound_speed: 340.0 * METER,
            attenuation: 0.0,
        }
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    /// Get the number of transducers
    pub fn num_transducers(&self) -> usize {
        self.transducers.len()
    }

    /// Get center position
    pub fn center(&self) -> Vector3 {
        self.transducers
            .iter()
            .map(|tr| tr.position())
            .sum::<Vector3>()
            / self.transducers.len() as float
    }

    /// Affine transform
    pub fn affine(&mut self, t: Vector3, r: UnitQuaternion) {
        self.transducers.iter_mut().for_each(|tr| tr.affine(t, r));
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
        self.sound_speed = (k * r * (273.15 + temp) / m).sqrt() * METER;
    }
}

impl<T: Transducer> Deref for Device<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.transducers
    }
}

impl<T: Transducer> DerefMut for Device<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transducers
    }
}

pub trait IntoDevice<T: Transducer> {
    fn into_device(self, dev_idx: usize) -> Device<T>;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn create_device<T: Transducer>(idx: usize, n: usize) -> Device<T> {
        Device::new(
            idx,
            (0..n)
                .map(|i| T::new(i, Vector3::zeros(), UnitQuaternion::identity()))
                .collect(),
        )
    }
}
