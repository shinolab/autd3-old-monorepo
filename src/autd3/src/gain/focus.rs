/*
 * File: focus.rs
 * Project: gain
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    derive::prelude::*,
    geometry::{Geometry, Vector3},
};

use autd3_derive::Gain;

/// Gain to produce a focal point
#[derive(Gain, Clone, Copy)]
pub struct Focus {
    amp: float,
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
        Self { pos, amp: 1.0 }
    }

    /// set amplitude
    ///
    /// # Arguments
    ///
    /// * `amp` - normalized amp (from 0 to 1)
    ///
    pub fn with_amp(self, amp: float) -> Self {
        Self { amp, ..self }
    }

    pub fn amp(&self) -> float {
        self.amp
    }

    pub fn pos(&self) -> Vector3 {
        self.pos
    }
}

impl<T: Transducer> Gain<T> for Focus {
    fn calc(
        &self,
        geometry: &Geometry<T>,
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
