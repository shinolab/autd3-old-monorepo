/*
 * File: focus.rs
 * Project: gain
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    common::{EmitIntensity, TryIntoEmittIntensity},
    derive::prelude::*,
    geometry::{Geometry, Vector3},
};

use autd3_derive::Gain;

/// Gain to produce a focal point
#[derive(Gain, Clone, Copy)]
pub struct Focus {
    amp: EmitIntensity,
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
            amp: EmitIntensity::MAX,
        }
    }

    /// set amplitude
    ///
    /// # Arguments
    ///
    /// * `amp` - amplitude
    ///
    pub fn with_amp<A: TryIntoEmittIntensity>(self, amp: A) -> Result<Self, AUTDInternalError> {
        Ok(Self {
            amp: amp.try_into()?,
            ..self
        })
    }

    pub fn amp(&self) -> EmitIntensity {
        self.amp
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
                amp: self.amp,
            }
        }))
    }
}
