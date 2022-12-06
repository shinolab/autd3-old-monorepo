/*
 * File: legacy_transducer.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
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
    sound_speed: f64,
    attenuation: f64,
    mod_delay: u16,
}

impl Transducer for LegacyTransducer {
    fn new(id: usize, pos: Vector3, rot: UnitQuaternion) -> Self {
        Self {
            id,
            pos,
            rot,
            sound_speed: 340e3,
            attenuation: 0.0,
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

    fn cycle(&self) -> u16 {
        4096
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

    fn sound_speed(&self) -> f64 {
        self.sound_speed
    }

    fn set_sound_speed(&mut self, value: f64) {
        self.sound_speed = value;
    }

    fn attenuation(&self) -> f64 {
        self.attenuation
    }

    fn set_attenuation(&mut self, value: f64) {
        self.attenuation = value;
    }
}
