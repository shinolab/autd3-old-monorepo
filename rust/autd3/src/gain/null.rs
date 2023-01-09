/*
 * File: null.rs
 * Project: gain
 * Created Date: 01/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::geometry::{Geometry, Transducer};

use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain, Default)]
pub struct Null<T: Transducer> {
    op: T::Gain,
}

impl<T: Transducer> Null<T> {
    /// constructor
    pub fn new() -> Self {
        Self {
            op: Default::default(),
        }
    }

    fn calc(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        geometry.transducers().for_each(|tr| {
            self.op.set_drive(tr.id(), 0.0, 0.0);
        });
        Ok(())
    }
}
