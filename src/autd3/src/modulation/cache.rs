/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_derive::Modulation;
use autd3_driver::derive::prelude::*;

use std::ops::{Deref, DerefMut};

/// Modulation to cache the result of calculation
#[derive(Modulation)]
pub struct Cache {
    cache: Vec<float>,
    freq_div: u32,
}

pub trait IntoCache<M: Modulation> {
    /// Cache the result of calculation
    fn with_cache(self) -> Result<Cache, AUTDInternalError>;
}

impl<M: Modulation> IntoCache<M> for M {
    fn with_cache(self) -> Result<Cache, AUTDInternalError> {
        Cache::new(self)
    }
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

    /// get cached modulation data
    pub fn buffer(&self) -> &[float] {
        &self.cache
    }

    /// get cached modulation data mutably
    pub fn buffer_mut(&mut self) -> &mut [float] {
        &mut self.cache
    }
}

impl Modulation for Cache {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.cache.clone())
    }
}

impl Deref for Cache {
    type Target = [float];

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl DerefMut for Cache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}

#[cfg(test)]
mod tests {
    use crate::modulation::Static;

    use super::*;

    #[test]
    fn test_cache() {
        let mut m = Static::new().with_cache().unwrap();

        for d in m.calc().unwrap() {
            assert_eq!(d, 1.0);
        }

        for d in m.iter_mut() {
            *d = 0.0;
        }

        for d in m.calc().unwrap() {
            assert_eq!(d, 0.0);
        }
    }
}
