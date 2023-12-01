/*
 * File: custom.rs
 * Project: tests
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3::{
    derive::{Gain, Modulation},
    prelude::*,
};
use autd3_driver::derive::prelude::*;

#[derive(Gain, Clone, Copy)]
pub struct MyUniform {}

impl MyUniform {
    pub fn new() -> Self {
        Self {}
    }
}

impl Gain for MyUniform {
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |_dev, _tr| Drive {
            phase: 0.0,
            intensity: EmitIntensity::MAX,
        }))
    }
}

#[derive(Modulation, Clone, Copy)]
pub struct Burst {
    config: SamplingConfiguration,
}

impl Burst {
    pub fn new() -> Self {
        Self {
            config: SamplingConfiguration::from_frequency(4e3).unwrap(),
        }
    }
}

impl Modulation for Burst {
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
        Ok((0..4000)
            .map(|i| {
                if i == 3999 {
                    EmitIntensity::MAX
                } else {
                    EmitIntensity::MIN
                }
            })
            .collect())
    }
}

pub async fn custom<L: Link>(autd: &mut Controller<L>) -> anyhow::Result<bool> {
    autd.send(Silencer::disable()).await?;

    let g = MyUniform::new();
    let m = Burst::new();

    autd.send((m, g)).await?;

    Ok(true)
}
