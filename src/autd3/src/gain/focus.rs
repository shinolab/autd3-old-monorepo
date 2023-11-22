/*
 * File: focus.rs
 * Project: gain
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    common::EmitIntensity,
    derive::prelude::*,
    geometry::{Geometry, Vector3},
};

use autd3_derive::Gain;

/// Gain to produce a focal point
#[derive(Gain, Clone, Copy)]
pub struct Focus {
    intensity: EmitIntensity,
    pos: Vector3,
}

impl Focus {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `pos` - position of the focal point
    ///
    pub fn new(pos: Vector3) -> Self {
        Self {
            pos,
            intensity: EmitIntensity::MAX,
        }
    }

    /// set emission intensity
    ///
    /// # Arguments
    ///
    /// * `intensity` - emission intensity
    ///
    pub fn with_intensity<A: Into<EmitIntensity>>(self, intensity: A) -> Self {
        Self {
            intensity: intensity.into(),
            ..self
        }
    }

    pub fn intensity(&self) -> EmitIntensity {
        self.intensity
    }

    pub fn pos(&self) -> Vector3 {
        self.pos
    }
}

impl Gain for Focus {
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |dev, tr| {
            let phase = tr.align_phase_at(self.pos, dev.sound_speed);
            Drive {
                phase,
                intensity: self.intensity,
            }
        }))
    }
}
