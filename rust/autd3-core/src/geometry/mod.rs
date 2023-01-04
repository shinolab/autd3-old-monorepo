/*
 * File: mod.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod builder;
mod device;
mod legacy_transducer;
mod normal_phase_transducer;
mod normal_transducer;
mod transducer;

pub type Vector3 = nalgebra::Vector3<f64>;
pub type Vector4 = nalgebra::Vector4<f64>;
pub type Quaternion = nalgebra::Quaternion<f64>;
pub type UnitQuaternion = nalgebra::UnitQuaternion<f64>;
pub type Matrix3 = nalgebra::Matrix3<f64>;
pub type Matrix4 = nalgebra::Matrix4<f64>;

use std::ops::{Index, IndexMut};

pub use builder::*;
pub use device::*;
pub use legacy_transducer::*;
pub use normal_phase_transducer::*;
pub use normal_transducer::*;
pub use transducer::*;

use crate::error::AUTDInternalError;

#[derive(Default)]
pub struct Geometry<T: Transducer> {
    transducers: Vec<T>,
    device_map: Vec<usize>,
    pub sound_speed: f64,
    pub attenuation: f64,
}

impl<T: Transducer> Geometry<T> {
    fn new() -> Geometry<T> {
        Geometry {
            transducers: vec![],
            device_map: vec![],
            sound_speed: 340e3,
            attenuation: 0.0,
        }
    }

    pub fn add_device<D: Device<T>>(&mut self, dev: D) -> anyhow::Result<()> {
        let id = self.transducers.len();
        let mut transducers = dev.get_transducers(id);
        if transducers.len() > 256 {
            return Err(AUTDInternalError::TransducersNumInDeviceOutOfRange.into());
        }
        self.device_map.push(transducers.len());
        self.transducers.append(&mut transducers);
        Ok(())
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

    pub fn transducers_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.transducers.iter_mut()
    }

    pub fn center(&self) -> Vector3 {
        self.transducers
            .iter()
            .map(|d| d.position())
            .sum::<Vector3>()
            / self.transducers.len() as f64
    }

    pub fn device_map(&self) -> &[usize] {
        &self.device_map
    }
}

impl<T: Transducer> Index<usize> for Geometry<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.transducers[idx]
    }
}

impl<T: Transducer> IndexMut<usize> for Geometry<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.transducers[idx]
    }
}

impl<'a, T> IntoIterator for &'a Geometry<T>
where
    T: Transducer,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> std::slice::Iter<'a, T> {
        self.transducers()
    }
}

impl<'a, T> IntoIterator for &'a mut Geometry<T>
where
    T: Transducer,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> std::slice::IterMut<'a, T> {
        self.transducers_mut()
    }
}
