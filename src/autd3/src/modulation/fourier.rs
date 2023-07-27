/*
 * File: fourier.rs
 * Project: modulation
 * Created Date: 28/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::sine::Sine;
use autd3_core::{
    error::AUTDInternalError,
    float,
    modulation::{Modulation, ModulationProperty},
};
use autd3_traits::Modulation;

use num::integer::lcm;

/// Multi-frequency sine wave modulation
#[derive(Modulation, Clone)]
pub struct Fourier {
    freq_div: u32,
    components: Vec<Sine>,
}

impl Fourier {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            freq_div: 5120,
        }
    }

    pub fn add_component(self, sine: Sine) -> Self {
        let Self { mut components, .. } = self;
        let freq_div = components
            .iter()
            .map(|c| c.sampling_frequency_division())
            .min()
            .unwrap_or(u32::MAX)
            .min(sine.sampling_frequency_division());
        components.push(sine.with_sampling_frequency_division(freq_div));
        Self {
            components,
            freq_div,
        }
    }

    pub fn add_components_from_iter<M: Into<Sine>, T: IntoIterator<Item = M>>(
        self,
        iter: T,
    ) -> Self {
        let Self { mut components, .. } = self;
        let append = iter.into_iter().map(|m| m.into()).collect::<Vec<_>>();
        let freq_div = components
            .iter()
            .map(|c| c.sampling_frequency_division())
            .min()
            .unwrap_or(u32::MAX)
            .min(
                append
                    .iter()
                    .map(|m| m.sampling_frequency_division())
                    .min()
                    .unwrap_or(u32::MAX),
            );
        components.extend(
            append
                .iter()
                .map(|m| m.with_sampling_frequency_division(freq_div)),
        );
        Self {
            components,
            freq_div,
        }
    }

    pub fn components(&self) -> &[Sine] {
        &self.components
    }
}

impl Default for Fourier {
    fn default() -> Self {
        Self::new()
    }
}

impl Modulation for Fourier {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        let n = self.components.len();
        let buffers = self
            .components
            .iter()
            .map(|c| c.calc())
            .collect::<Result<Vec<_>, _>>()?;
        let len = buffers.iter().fold(1, |acc, x| lcm(acc, x.len()));
        Ok(buffers
            .iter()
            .map(|b| b.iter().cycle().take(len).collect::<Vec<_>>())
            .fold(vec![0.0; len], |acc, x| {
                acc.iter()
                    .zip(x.iter())
                    .map(|(a, &b)| a + b)
                    .collect::<Vec<_>>()
            })
            .iter()
            .map(|x| x / n as float)
            .collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fourier() {
        let f1 = Sine::new(100);
        let f2 = Sine::new(150);
        let f3 = Sine::new(200);

        let f1_buf = f1.calc().unwrap();
        let f2_buf = f2.calc().unwrap();
        let f3_buf = f3.calc().unwrap();

        let f = Fourier::new()
            .add_component(f1)
            .add_components_from_iter([f2, f3]);

        let buf = f.calc().unwrap();

        for i in 0..buf.len() {
            assert_approx_eq::assert_approx_eq!(
                buf[i],
                (f1_buf[i % f1_buf.len()] + f2_buf[i % f2_buf.len()] + f3_buf[i % f3_buf.len()])
                    / 3.0
            );
        }
    }
}
