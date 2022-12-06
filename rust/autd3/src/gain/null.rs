/*
 * File: null.rs
 * Project: gain
 * Created Date: 01/05/2022
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
    geometry::{Geometry, Transducer},
};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain)]
pub struct Null {
    props: GainProps,
}

impl Null {
    /// constructor
    pub fn new() -> Self {
        Self {
            props: GainProps::new(),
        }
    }

    fn calc<T: Transducer>(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        geometry.transducers().for_each(|tr| {
            self.props.drives[tr.id()].amp = 0.0;
            self.props.drives[tr.id()].phase = 0.0;
        });
        Ok(())
    }
}

impl Default for Null {
    fn default() -> Self {
        Self::new()
    }
}
