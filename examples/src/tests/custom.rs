/*
 * File: custom.rs
 * Project: tests
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::{
    core::{
        error::AUTDInternalError,
        float,
        gain::Gain,
        geometry::{Geometry, Transducer},
        modulation::Modulation,
        Drive,
    },
    prelude::*,
    traits::{Gain, Modulation},
};

/// Gain to produce single focal point
#[derive(Gain, Clone, Copy)]
pub struct Uniform {}

impl Uniform {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Gain<T> for Uniform {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(Self::transform(geometry, |_| Drive {
            phase: 0.0,
            amp: 1.0,
        }))
    }
}

#[derive(Modulation, Clone, Copy)]
pub struct Burst {
    freq_div: u32,
}

impl Burst {
    pub fn new() -> Self {
        Self { freq_div: 5120 }
    }
}

impl Modulation for Burst {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError> {
        Ok((0..4000)
            .map(|i| if i == 3999 { 1.0 } else { 0.0 })
            .collect())
    }
}

pub fn custom<T: Transducer, L: Link<T>>(
    autd: &mut Controller<T, L>,
) -> anyhow::Result<bool, AUTDError> {
    autd.send(SilencerConfig::none())?;

    let g = Uniform::new();
    let m = Burst::new();

    autd.send((m, g))
}
