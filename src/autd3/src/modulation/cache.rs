/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, float, modulation::Modulation};
use autd3_traits::Modulation;

use std::ops::{Deref, DerefMut};

#[derive(Modulation)]
pub struct CacheImpl {
    cache: Vec<float>,
    freq_div: u32,
}

pub trait Cache<M: Modulation> {
    fn with_cache(self) -> Result<CacheImpl, AUTDInternalError>;
}

impl<M: Modulation> Cache<M> for M {
    fn with_cache(self) -> Result<CacheImpl, AUTDInternalError> {
        CacheImpl::new(self)
    }
}

impl Clone for CacheImpl {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
            freq_div: self.freq_div,
        }
    }
}

impl CacheImpl {
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

impl Modulation for CacheImpl {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.cache.clone())
    }
}

impl Deref for CacheImpl {
    type Target = [float];

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl DerefMut for CacheImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Static;

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
