/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, gain::Gain, geometry::*, Drive};
use autd3_traits::Gain;

use std::ops::{Deref, DerefMut};

/// Gain to cache the result of calculation
#[derive(Gain)]
pub struct CacheImpl {
    cache: Vec<Drive>,
}

pub trait Cache<T: Transducer, G: Gain<T>> {
    /// Cache the result of calculation
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
    fn new<T: Transducer, G: Gain<T>>(
        gain: G,
        geometry: &Geometry<T>,
    ) -> Result<Self, AUTDInternalError> {
        Ok(Self {
            cache: gain.calc(geometry)?,
        })
    }

    /// get cached drives
    pub fn drives(&self) -> &[Drive] {
        &self.cache
    }

    /// get cached drives mutably
    pub fn drives_mut(&mut self) -> &mut [Drive] {
        &mut self.cache
    }
}

impl<T: Transducer> Gain<T> for CacheImpl {
    fn calc(&self, _geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
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

#[cfg(test)]
mod tests {
    use autd3_core::autd3_device::AUTD3;
    use autd3_core::geometry::LegacyTransducer;

    use super::*;

    use crate::prelude::Plane;
    use crate::tests::GeometryBuilder;

    #[test]
    fn test_cache() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let mut gain = Plane::new(Vector3::zeros()).with_cache(&geometry).unwrap();

        for drive in gain.calc(&geometry).unwrap() {
            assert_eq!(drive.phase, 0.0);
            assert_eq!(drive.amp, 1.0);
        }

        for drive in gain.iter_mut() {
            drive.phase = 1.0;
        }

        for drive in gain.calc(&geometry).unwrap() {
            assert_eq!(drive.phase, 1.0);
            assert_eq!(drive.amp, 1.0);
        }
    }
}
