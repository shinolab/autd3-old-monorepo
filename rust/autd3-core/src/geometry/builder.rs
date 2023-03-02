/*
 * File: builder.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::marker::PhantomData;

use super::{AdvancedPhaseTransducer, AdvancedTransducer, Geometry, LegacyTransducer};

pub struct Advanced;
pub struct AdvancedPhase;
pub struct Legacy;

pub struct GeometryBuilder<M> {
    attenuation: f64,
    sound_speed: f64,
    _mode: PhantomData<M>,
}

impl<M> GeometryBuilder<M> {
    pub fn attenuation(mut self, attenuation: f64) -> Self {
        self.attenuation = attenuation;
        self
    }

    pub fn sound_speed(mut self, sound_speed: f64) -> Self {
        self.sound_speed = sound_speed;
        self
    }
}

impl GeometryBuilder<Advanced> {
    pub fn new() -> Self {
        Self {
            attenuation: 0.0,
            sound_speed: 340.0e3,
            _mode: PhantomData,
        }
    }

    pub fn legacy_mode(self) -> GeometryBuilder<Legacy> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn advanced_phase_mode(self) -> GeometryBuilder<AdvancedPhase> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn build(self) -> Geometry<AdvancedTransducer> {
        Geometry::<AdvancedTransducer>::new()
    }
}

impl GeometryBuilder<Legacy> {
    pub fn advanced_mode(self) -> GeometryBuilder<Advanced> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn advanced_phase_mode(self) -> GeometryBuilder<AdvancedPhase> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn build(self) -> Geometry<LegacyTransducer> {
        Geometry::<LegacyTransducer>::new()
    }
}

impl GeometryBuilder<AdvancedPhase> {
    pub fn advanced_mode(self) -> GeometryBuilder<Advanced> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn legacy_mode(self) -> GeometryBuilder<Legacy> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn build(self) -> Geometry<AdvancedPhaseTransducer> {
        Geometry::<AdvancedPhaseTransducer>::new()
    }
}

impl Default for GeometryBuilder<Advanced> {
    fn default() -> Self {
        Self::new()
    }
}
