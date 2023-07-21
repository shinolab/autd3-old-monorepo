/*
 * File: transducer.rs
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

use autd3_driver::{float, PI};

use super::{Quaternion, UnitQuaternion, Vector3};

fn get_direction(dir: Vector3, rotation: &UnitQuaternion) -> Vector3 {
    let dir: UnitQuaternion = UnitQuaternion::from_quaternion(Quaternion::from_imag(dir));
    (rotation * dir * rotation.conjugate()).imag().normalize()
}

pub trait Transducer: Send + Sync {
    type Gain: autd3_driver::operation::GainOp;
    type Sync: autd3_driver::operation::SyncOp;
    type GainSTM: autd3_driver::operation::GainSTMOp;

    /// Create transducer
    fn new(id: usize, pos: Vector3, rot: UnitQuaternion) -> Self;
    /// Affine transformation
    fn affine(&mut self, pos: Vector3, rot: UnitQuaternion);
    /// Calculate the phase of the transducer to align the phase at the specified position
    fn align_phase_at(&self, pos: Vector3, sound_speed: float) -> float {
        (pos - self.position()).norm() * self.wavenumber(sound_speed)
    }
    /// Get the position of the transducer
    fn position(&self) -> &Vector3;
    /// Get the rotation of the transducer
    fn rotation(&self) -> &UnitQuaternion;
    /// Get the index of the transducer
    fn idx(&self) -> usize;
    /// Get the x-direction of the transducer
    fn x_direction(&self) -> Vector3 {
        get_direction(Vector3::x(), self.rotation())
    }
    /// Get the y-direction of the transducer
    fn y_direction(&self) -> Vector3 {
        get_direction(Vector3::y(), self.rotation())
    }
    /// Get the z-direction (axial direction) of the transducer
    fn z_direction(&self) -> Vector3 {
        get_direction(Vector3::z(), self.rotation())
    }
    /// Get the frequency of the transducer
    fn frequency(&self) -> float;
    /// Get the modulation delay of the transducer
    fn mod_delay(&self) -> u16;
    /// Set the modulation delay of the transducer
    fn set_mod_delay(&mut self, value: u16);
    /// Get the wavelength of the transducer
    fn wavelength(&self, sound_speed: float) -> float {
        sound_speed / self.frequency()
    }
    /// Get the wavenumber of the transducer
    fn wavenumber(&self, sound_speed: float) -> float {
        2.0 * PI * self.frequency() / sound_speed
    }
    /// Get the cycle of the transducer
    fn cycle(&self) -> u16;
}
