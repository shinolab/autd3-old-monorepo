/*
 * File: transform.rs
 * Project: modulation
 * Created Date: 15/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_derive::Modulation;
use autd3_driver::{defined::float, error::AUTDInternalError, modulation::Modulation};

/// Modulation to transform modulation data
#[derive(Modulation)]
pub struct Transform<M: Modulation, F: Fn(usize, &float) -> float> {
    m: M,
    freq_div: u32,
    f: F,
}

pub trait IntoTransform<M: Modulation> {
    /// transform modulation data
    ///
    /// # Arguments
    ///
    /// * `f` - transform function. The first argument is index of the element, and the second argument is the value of the element of the original modulation data.
    ///
    fn with_transform<F: Fn(usize, &float) -> float>(self, f: F) -> Transform<M, F>;
}

impl<M: Modulation> IntoTransform<M> for M {
    fn with_transform<F: Fn(usize, &float) -> float>(self, f: F) -> Transform<M, F> {
        Transform {
            freq_div: self.sampling_frequency_division(),
            f,
            m: self,
        }
    }
}

impl<M: Modulation, F: Fn(usize, &float) -> float> Modulation for Transform<M, F> {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        let m = self.m.calc()?;
        Ok(m.iter().enumerate().map(|(i, x)| (self.f)(i, x)).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::modulation::Sine;

    use super::*;

    #[test]
    fn test_transform_impl() {
        let m = Sine::new(100).with_amp(0.5);
        let m_transformed = m.with_transform(|i, x| i as float * x);

        let vec = m.calc().unwrap();
        let vec_transformed = m_transformed.calc().unwrap();

        for (i, (&x, &y)) in vec.iter().zip(&vec_transformed).enumerate() {
            assert_approx_eq::assert_approx_eq!(y, i as float * x);
        }
    }
}
