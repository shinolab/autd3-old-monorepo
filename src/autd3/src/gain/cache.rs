/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, gain::Gain, geometry::*, Drive};
use autd3_traits::Gain;

use std::ops::{Deref, DerefMut};

#[derive(Gain)]
pub struct CacheImpl {
    cache: Vec<Drive>,
}

pub trait Cache<T: Transducer, G: Gain<T>> {
    fn with_cache(self, geometry: &Geometry<T>) -> Result<CacheImpl, AUTDInternalError>;
}

impl<T: Transducer, G: Gain<T>> Cache<T, G> for G {
    fn with_cache(self, geometry: &Geometry<T>) -> Result<CacheImpl, AUTDInternalError> {
        CacheImpl::new(self, geometry)
    }
}

impl Clone for CacheImpl {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
        }
    }
}

impl CacheImpl {
    /// constructor
    pub fn new<T: Transducer, G: Gain<T>>(
        mut gain: G,
        geometry: &Geometry<T>,
    ) -> Result<Self, AUTDInternalError> {
        Ok(Self {
            cache: gain.calc(geometry)?,
        })
    }

    pub fn drives(&self) -> &[Drive] {
        &self.cache
    }

    pub fn drives_mut(&mut self) -> &mut [Drive] {
        &mut self.cache
    }
}

impl<T: Transducer> Gain<T> for CacheImpl {
    fn calc(&mut self, _geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(self.cache.clone())
    }
}

impl Deref for CacheImpl {
    type Target = [Drive];

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl DerefMut for CacheImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}

impl<'a> IntoIterator for &'a Cache {
    type Item = &'a Drive;
    type IntoIter = std::slice::Iter<'a, Drive>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.iter()
    }
}

impl<'a> IntoIterator for &'a mut Cache {
    type Item = &'a mut Drive;
    type IntoIter = std::slice::IterMut<'a, Drive>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.iter_mut()
    }
}
