/*
 * File: transform.rs
 * Project: modulation
 * Created Date: 15/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, float, modulation::Modulation};
use autd3_traits::Modulation;

#[derive(Modulation)]
pub struct TransformImpl<M: Modulation, F: Fn(usize, &float) -> float> {
    m: M,
    freq_div: u32,
    f: F,
}

pub trait Transform<M: Modulation> {
    fn with_transform<F: Fn(usize, &float) -> float>(self, f: F) -> TransformImpl<M, F>;
}

impl<M: Modulation> Transform<M> for M {
    fn with_transform<F: Fn(usize, &float) -> float>(self, f: F) -> TransformImpl<M, F> {
        TransformImpl {
            freq_div: self.sampling_frequency_division(),
            f,
            m: self,
        }
    }
}

impl<M: Modulation, F: Fn(usize, &float) -> float> Modulation for TransformImpl<M, F> {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError> {
        let m = self.m.calc()?;
        Ok(m.iter().enumerate().map(|(i, x)| (self.f)(i, x)).collect())
    }
}
