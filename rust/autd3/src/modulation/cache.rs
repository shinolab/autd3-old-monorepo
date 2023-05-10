/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, float, modulation::Modulation};
use autd3_traits::Modulation;

use std::ops::{Index, IndexMut};

#[derive(Modulation)]
pub struct Cache {
    cache: Vec<float>,
    freq_div: u32,
}

impl Clone for Cache {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
            freq_div: self.freq_div,
        }
    }
}

impl Cache {
    /// constructor
    pub fn new<M: Modulation>(modulation: M) -> Result<Self, AUTDInternalError> {
        let freq_div = modulation.sampling_frequency_division();
        Ok(Self {
            cache: modulation.calc()?,
            freq_div,
        })
    }

    pub fn buffer(&self) -> &[float] {
        &self.cache
    }

    pub fn buffer_mut(&mut self) -> &mut [float] {
        &mut self.cache
    }
}

impl Modulation for Cache {
    fn calc(self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.cache)
    }
}

impl Index<usize> for Cache {
    type Output = float;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.cache[idx]
    }
}

impl IndexMut<usize> for Cache {
    fn index_mut(&mut self, idx: usize) -> &mut float {
        &mut self.cache[idx]
    }
}

impl<'a> IntoIterator for &'a Cache {
    type Item = &'a float;
    type IntoIter = std::slice::Iter<'a, float>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.iter()
    }
}

impl<'a> IntoIterator for &'a mut Cache {
    type Item = &'a mut float;
    type IntoIter = std::slice::IterMut<'a, float>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.iter_mut()
    }
}
