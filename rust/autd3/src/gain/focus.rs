/*
 * File: focus.rs
 * Project: gain
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    gain::GainProps,
    geometry::{Geometry, Transducer, Vector3},
};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain)]
pub struct Focus {
    props: GainProps,
    amp: f64,
    pos: Vector3,
}

impl Focus {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `pos` - position of focal point
    ///
    pub fn new(pos: Vector3) -> Self {
        Self::with_amp(pos, 1.0)
    }

    /// constructor with duty
    ///
    /// # Arguments
    ///
    /// * `pos` - position of focal point
    /// * `amp` - normalized amp (from 0 to 1)
    ///
    pub fn with_amp(pos: Vector3, amp: f64) -> Self {
        Self {
            props: GainProps::new(),
            amp,
            pos,
        }
    }

    fn calc<T: Transducer>(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        geometry.transducers().for_each(|tr| {
            let dist = (self.pos - tr.position()).norm();
            let phase = tr.align_phase_at(dist);
            self.props.drives[tr.id()].amp = self.amp;
            self.props.drives[tr.id()].phase = phase;
        });
        Ok(())
    }
}
