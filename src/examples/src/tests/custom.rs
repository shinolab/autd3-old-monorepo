/*
 * File: custom.rs
 * Project: tests
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3::{
    derive::{Gain, Modulation},
    driver::derive::prelude::*,
    prelude::*,
};

#[derive(Gain, Clone, Copy)]
pub struct MyUniform {}

impl MyUniform {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Gain<T> for MyUniform {
    fn calc(
        &self,
        geometry: &Geometry<T>,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |_dev, _tr| Drive {
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
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok((0..4000)
            .map(|i| if i == 3999 { 1.0 } else { 0.0 })
            .collect())
    }
}

pub fn custom<T: Transducer, L: Link<T>>(autd: &mut Controller<T, L>) -> anyhow::Result<bool>
where
    autd3::driver::operation::GainOp<T, MyUniform>: autd3::driver::operation::Operation<T>,
{
    autd.send(Silencer::disable())?;

    let g = MyUniform::new();
    let m = Burst::new();

    autd.send((m, g))?;

    Ok(true)
}
