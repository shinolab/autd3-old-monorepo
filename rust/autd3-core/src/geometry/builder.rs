/*
 * File: builder.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::error::AUTDInternalError;

use super::{
    AdvancedPhaseTransducer, AdvancedTransducer, Device, Geometry, LegacyTransducer, Transducer,
};

use autd3_driver::{float, METER};

pub struct GeometryBuilder<T: Transducer> {
    attenuation: float,
    sound_speed: float,
    transducers: Vec<T>,
    device_map: Vec<usize>,
}

impl<T: Transducer> GeometryBuilder<T> {
    pub fn attenuation(mut self, attenuation: float) -> Self {
        self.attenuation = attenuation;
        self
    }

    pub fn sound_speed(mut self, sound_speed: float) -> Self {
        self.sound_speed = sound_speed;
        self
    }
}

impl GeometryBuilder<LegacyTransducer> {
    pub fn new() -> Self {
        Self {
            attenuation: 0.0,
            sound_speed: 340.0 * METER,
            transducers: vec![],
            device_map: vec![],
        }
    }

    pub fn add_device<D: Device<LegacyTransducer>>(mut self, dev: D) -> Self {
        let id = self.transducers.len();
        let mut transducers = dev.get_transducers(id);
        self.device_map.push(transducers.len());
        self.transducers.append(&mut transducers);
        self
    }

    pub fn build(self) -> Result<Geometry<LegacyTransducer>, AUTDInternalError> {
        Geometry::<LegacyTransducer>::new(
            self.transducers,
            self.device_map,
            self.sound_speed,
            self.attenuation,
        )
    }
}

impl GeometryBuilder<AdvancedTransducer> {
    pub fn new() -> Self {
        Self {
            attenuation: 0.0,
            sound_speed: 340.0 * METER,
            transducers: vec![],
            device_map: vec![],
        }
    }

    pub fn add_device<D: Device<AdvancedTransducer>>(mut self, dev: D) -> Self {
        let id = self.transducers.len();
        let mut transducers = dev.get_transducers(id);
        self.device_map.push(transducers.len());
        self.transducers.append(&mut transducers);
        self
    }

    pub fn build(self) -> Result<Geometry<AdvancedTransducer>, AUTDInternalError> {
        Geometry::<AdvancedTransducer>::new(
            self.transducers,
            self.device_map,
            self.sound_speed,
            self.attenuation,
        )
    }
}

impl GeometryBuilder<AdvancedPhaseTransducer> {
    pub fn new() -> Self {
        Self {
            attenuation: 0.0,
            sound_speed: 340.0 * METER,
            transducers: vec![],
            device_map: vec![],
        }
    }

    pub fn add_device<D: Device<AdvancedPhaseTransducer>>(mut self, dev: D) -> Self {
        let id = self.transducers.len();
        let mut transducers = dev.get_transducers(id);
        self.device_map.push(transducers.len());
        self.transducers.append(&mut transducers);
        self
    }

    pub fn build(self) -> Result<Geometry<AdvancedPhaseTransducer>, AUTDInternalError> {
        Geometry::<AdvancedPhaseTransducer>::new(
            self.transducers,
            self.device_map,
            self.sound_speed,
            self.attenuation,
        )
    }
}

impl Default for GeometryBuilder<LegacyTransducer> {
    fn default() -> Self {
        Self::new()
    }
}
