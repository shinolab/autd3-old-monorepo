/*
 * File: fourier.rs
 * Project: modulation
 * Created Date: 28/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::ops::{Deref, DerefMut};

use super::sine::Sine;
use autd3_core::{
    error::AUTDInternalError,
    float,
    modulation::{Modulation, ModulationProperty},
};

use num::integer::lcm;

/// Multi-frequency sine wave modulation
#[derive(Clone)]
pub struct Fourier {
    freq_div: u32,
    components: Vec<Sine>,
}

impl Fourier {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            freq_div: u32::MAX,
        }
    }

    pub fn add_component(self, sine: Sine) -> Self {
        let Self {
            mut components,
            freq_div,
        } = self;
        let freq_div = freq_div.min(sine.sampling_frequency_division());
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
        let Self {
            mut components,
            freq_div,
        } = self;
        let append = iter.into_iter().map(|m| m.into()).collect::<Vec<_>>();
        let freq_div = append
            .iter()
            .fold(freq_div, |acc, m| acc.min(m.sampling_frequency_division()));
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
}

impl Deref for Fourier {
    type Target = [Sine];

    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl DerefMut for Fourier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.components
    }
}

impl Default for Fourier {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Add<Sine> for Fourier {
    type Output = Self;

    fn add(self, rhs: Sine) -> Self::Output {
        self.add_component(rhs)
    }
}

impl std::ops::Add<Sine> for Sine {
    type Output = Fourier;

    fn add(self, rhs: Sine) -> Self::Output {
        Fourier::new().add_component(self).add_component(rhs)
    }
}

impl ModulationProperty for Fourier {
    fn sampling_frequency(&self) -> float {
        autd3_core::FPGA_SUB_CLK_FREQ as float / self.freq_div as float
    }

    fn sampling_frequency_division(&self) -> u32 {
        self.freq_div
    }
}

impl<T: autd3_core::geometry::Transducer> autd3_core::datagram::Datagram<T> for Fourier {
    type H = autd3_core::Modulation;
    type B = autd3_core::NullBody;

    fn operation(
        &self,
        _geometry: &autd3_core::geometry::Geometry<T>,
    ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
        let freq_div = self.freq_div;
        Ok((Self::H::new(self.calc()?, freq_div), Self::B::default()))
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
        let f0 = Sine::new(50);
        let f1 = Sine::new(100);
        let f2 = Sine::new(150);
        let f3 = Sine::new(200);
        let f4 = Sine::new(250);

        let f0_buf = f0.calc().unwrap();
        let f1_buf = f1.calc().unwrap();
        let f2_buf = f2.calc().unwrap();
        let f3_buf = f3.calc().unwrap();
        let f4_buf = f4.calc().unwrap();

        let f = (f0 + f1).add_component(f2).add_components_from_iter([f3]) + f4;

        let buf = f.calc().unwrap();

        for i in 0..buf.len() {
            assert_approx_eq::assert_approx_eq!(
                buf[i],
                (f0_buf[i % f0_buf.len()]
                    + f1_buf[i % f1_buf.len()]
                    + f2_buf[i % f2_buf.len()]
                    + f3_buf[i % f3_buf.len()]
                    + f4_buf[i % f4_buf.len()])
                    / 5.0
            );
        }
    }
}
