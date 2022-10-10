/*
 * File: builder.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::marker::PhantomData;

use super::{Geometry, LegacyTransducer, NormalPhaseTransducer, NormalTransducer};

pub struct Normal;
pub struct NormalPhase;
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

impl GeometryBuilder<Normal> {
    pub fn new() -> Self {
        Self {
            attenuation: 0.0,
            sound_speed: 340.0,
            _mode: PhantomData,
        }
    }

    pub fn legacy_mode(self) -> GeometryBuilder<Legacy> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn normal_phase_mode(self) -> GeometryBuilder<NormalPhase> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn build(self) -> Geometry<NormalTransducer> {
        Geometry::<NormalTransducer>::new(self.attenuation, self.sound_speed)
    }
}

impl GeometryBuilder<Legacy> {
    pub fn normal_mode(self) -> GeometryBuilder<Normal> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn normal_phase_mode(self) -> GeometryBuilder<NormalPhase> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn build(self) -> Geometry<LegacyTransducer> {
        Geometry::<LegacyTransducer>::new(self.attenuation, self.sound_speed)
    }
}

impl GeometryBuilder<NormalPhase> {
    pub fn normal_mode(self) -> GeometryBuilder<Normal> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn legacy_mode(self) -> GeometryBuilder<Legacy> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn build(self) -> Geometry<NormalPhaseTransducer> {
        Geometry::<NormalPhaseTransducer>::new(self.attenuation, self.sound_speed)
    }
}

impl Default for GeometryBuilder<Normal> {
    fn default() -> Self {
        Self::new()
    }
}
