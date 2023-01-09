/*
 * File: transducer.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::f64::consts::PI;

use autd3_driver::{GainOp, GainSTMOp, SyncOP};

use super::{Quaternion, UnitQuaternion, Vector3};

pub trait Transducer: Sized {
    type Sync: Default + SyncOP;
    type Gain: Default + GainOp;
    type GainSTM: Default + GainSTMOp;

    fn get_direction(dir: Vector3, rotation: &UnitQuaternion) -> Vector3 {
        let dir: UnitQuaternion = UnitQuaternion::from_quaternion(Quaternion::from_imag(dir));
        (rotation * dir * rotation.conjugate()).imag().normalize()
    }
    fn new(id: usize, pos: Vector3, rot: UnitQuaternion) -> Self;
    fn align_phase_at(&self, dist: f64, sound_speed: f64) -> f64 {
        dist * self.wavenumber(sound_speed)
    }
    fn position(&self) -> &Vector3;
    fn rotation(&self) -> &UnitQuaternion;
    fn id(&self) -> usize;
    fn x_direction(&self) -> Vector3 {
        Self::get_direction(Vector3::x(), self.rotation())
    }
    fn y_direction(&self) -> Vector3 {
        Self::get_direction(Vector3::y(), self.rotation())
    }
    fn z_direction(&self) -> Vector3 {
        Self::get_direction(Vector3::z(), self.rotation())
    }
    fn frequency(&self) -> f64;
    fn mod_delay(&self) -> u16;
    fn set_mod_delay(&mut self, value: u16);
    fn wavelength(&self, sound_speed: f64) -> f64 {
        sound_speed / self.frequency()
    }
    fn wavenumber(&self, sound_speed: f64) -> f64 {
        2.0 * PI * self.frequency() / sound_speed
    }
}
