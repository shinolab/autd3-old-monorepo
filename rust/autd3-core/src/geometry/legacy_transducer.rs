/*
 * File: legacy_transducer.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use super::{Transducer, UnitQuaternion, Vector3};

pub struct LegacyTransducer {
    id: usize,
    pos: Vector3,
    rot: UnitQuaternion,
    mod_delay: u16,
}

impl Transducer for LegacyTransducer {
    fn new(id: usize, pos: Vector3, rot: UnitQuaternion) -> Self {
        Self {
            id,
            pos,
            rot,
            mod_delay: 0,
        }
    }

    fn position(&self) -> &Vector3 {
        &self.pos
    }

    fn rotation(&self) -> &UnitQuaternion {
        &self.rot
    }

    fn id(&self) -> usize {
        self.id
    }

    fn frequency(&self) -> f64 {
        40e3
    }

    fn mod_delay(&self) -> u16 {
        self.mod_delay
    }

    fn set_mod_delay(&mut self, delay: u16) {
        self.mod_delay = delay;
    }

    fn get_direction(dir: Vector3, rotation: &UnitQuaternion) -> Vector3 {
        let dir: UnitQuaternion =
            UnitQuaternion::from_quaternion(super::Quaternion::from_imag(dir));
        (rotation * dir * rotation.conjugate()).imag().normalize()
    }

    fn align_phase_at(&self, dist: f64, sound_speed: f64) -> f64 {
        dist * self.wavenumber(sound_speed)
    }

    fn x_direction(&self) -> Vector3 {
        Self::get_direction(Vector3::x(), self.rotation())
    }

    fn y_direction(&self) -> Vector3 {
        Self::get_direction(Vector3::y(), self.rotation())
    }

    fn z_direction(&self) -> Vector3 {
        Self::get_direction(Vector3::z(), self.rotation())
    }

    fn wavelength(&self, sound_speed: f64) -> f64 {
        sound_speed / self.frequency()
    }

    fn wavenumber(&self, sound_speed: f64) -> f64 {
        2.0 * std::f64::consts::PI * self.frequency() / sound_speed
    }
}
