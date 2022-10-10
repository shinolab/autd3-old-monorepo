/*
 * File: mod.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2022
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

use autd3_driver::NUM_TRANS_IN_UNIT;
pub use builder::*;
pub use device::*;
pub use legacy_transducer::*;
pub use normal_phase_transducer::*;
pub use normal_transducer::*;
pub use transducer::*;

#[derive(Default)]
pub struct Geometry<T: Transducer> {
    devices: Vec<Device<T>>,
    pub attenuation: f64,
    pub sound_speed: f64,
}

impl<T: Transducer> Geometry<T> {
    fn new(attenuation: f64, sound_speed: f64) -> Geometry<T> {
        Geometry {
            devices: vec![],
            attenuation,
            sound_speed,
        }
    }

    pub fn num_devices(&self) -> usize {
        self.devices.len()
    }

    pub fn num_transducers(&self) -> usize {
        self.devices.len() * NUM_TRANS_IN_UNIT
    }

    pub fn devices(&self) -> &[Device<T>] {
        &self.devices
    }

    pub fn devices_mut(&mut self) -> &mut [Device<T>] {
        &mut self.devices
    }

    pub fn transducers(&self) -> impl Iterator<Item = &T> {
        self.devices.iter().flat_map(|dev| dev.transducers())
    }

    pub fn transducers_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.devices
            .iter_mut()
            .flat_map(|dev| dev.transducers_mut())
    }

    pub fn center(&self) -> Vector3 {
        let sum: Vector3 = self.devices().iter().map(|d| d.center()).sum();
        sum / self.devices.len() as f64
    }

    pub fn sound_speed(&self) -> f64 {
        self.sound_speed
    }

    pub fn set_sound_speed(&mut self, sound_speed: f64) {
        self.sound_speed = sound_speed;
    }
}

impl Geometry<LegacyTransducer> {
    pub fn wavelength(&self) -> f64 {
        self.sound_speed() / 40e3
    }

    pub fn set_wavelength(&mut self, wavelength: f64) {
        let sound_speed = 40e3 * wavelength;
        self.set_sound_speed(sound_speed);
    }
}

impl<T: Transducer> Geometry<T> {
    /// Add device to the geometry.
    ///
    /// Use this method to specify the device geometry in order of proximity to the master.
    /// Call this method or [add_device_quaternion](#method.add_device_quaternion) as many times as the number of AUTDs connected to the master.
    ///
    /// # Arguments
    ///
    /// * `pos` - Global position of AUTD.
    /// * `rot` - ZYZ Euler angles.
    ///
    /// # Example
    ///
    /// ```
    /// use std::f64::consts::PI;
    /// use autd3_core::geometry::{Vector3, GeometryBuilder};
    ///
    /// let mut geometry = GeometryBuilder::new().build();
    ///
    /// geometry.add_device(Vector3::zeros(), Vector3::zeros());
    /// geometry.add_device(Vector3::new(192., 0., 0.), Vector3::new(-PI, 0., 0.));
    /// ```
    pub fn add_device(&mut self, position: Vector3, euler_angles: Vector3) {
        let q = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), euler_angles.x)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), euler_angles.y)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), euler_angles.z);
        self.add_device_quaternion(position, q)
    }

    /// Add device to the geometry.
    ///
    /// Use this method to specify the device geometry in order of proximity to the master.
    /// Call this method or [add_device](#method.add_device) as many times as the number of AUTDs connected to the master.
    ///
    /// # Arguments
    ///
    /// * `pos` - Global position of AUTD.
    /// * `rot` - Rotation quaternion.
    ///
    pub fn add_device_quaternion(&mut self, position: Vector3, rotation: UnitQuaternion) {
        let id = self.devices.len();
        self.devices.push(Device::<T>::new(id, position, rotation));
    }
}
