/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    gain::{Gain, GainFilter},
    geometry::*,
    Drive,
};
use autd3_derive::Gain;

use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

/// Gain to cache the result of calculation
#[derive(Gain)]
pub struct CacheImpl<T: Transducer + 'static, G: Gain<T> + 'static> {
    gain: G,
    cache: UnsafeCell<Vec<Drive>>,
    _phantom: std::marker::PhantomData<T>,
}

pub trait Cache<T: Transducer + 'static, G: Gain<T> + 'static> {
    /// Cache the result of calculation
    fn with_cache(self) -> CacheImpl<T, G>;
}

impl<T: Transducer + 'static, G: Gain<T> + 'static> Cache<T, G> for G {
    fn with_cache(self) -> CacheImpl<T, G> {
        CacheImpl::new(self)
    }
}

impl<T: Transducer + 'static, G: Gain<T> + 'static + Clone> Clone for CacheImpl<T, G> {
    fn clone(&self) -> Self {
        Self {
            gain: self.gain.clone(),
            cache: unsafe { UnsafeCell::new(self.cache.get().as_ref().unwrap().clone()) },
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Transducer + 'static, G: Gain<T> + 'static> CacheImpl<T, G> {
    /// constructor
    fn new(gain: G) -> Self {
        Self {
            gain,
            cache: UnsafeCell::new(vec![]),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn init(&self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        if self.drives().is_empty() {
            unsafe {
                self.cache
                    .get()
                    .as_mut()
                    .unwrap()
                    .extend(self.gain.calc(geometry, GainFilter::All)?)
            };
        }
        Ok(())
    }

    /// get cached drives
    pub fn drives(&self) -> &[Drive] {
        unsafe { self.cache.get().as_ref().unwrap() }
    }

    /// get cached drives mutably
    pub fn drives_mut(&mut self) -> &mut [Drive] {
        self.cache.get_mut()
    }
}

impl<T: Transducer + 'static, G: Gain<T> + 'static> Gain<T> for CacheImpl<T, G> {
    fn calc(
        &self,
        geometry: &Geometry<T>,
        _filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        self.init(geometry)?;
        Ok(unsafe { self.cache.get().as_ref().unwrap().clone() })
    }
}

impl<T: Transducer + 'static, G: Gain<T> + 'static> Deref for CacheImpl<T, G> {
    type Target = [Drive];

    fn deref(&self) -> &Self::Target {
        self.drives()
    }
}

impl<T: Transducer + 'static, G: Gain<T> + 'static> DerefMut for CacheImpl<T, G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.drives_mut()
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

        let mut gain = Plane::new(Vector3::zeros()).with_cache();

        for drive in gain.calc(&geometry, GainFilter::All).unwrap() {
            assert_eq!(drive.phase, 0.0);
            assert_eq!(drive.amp, 1.0);
        }

        for drive in gain.iter_mut() {
            drive.phase = 1.0;
        }

        for drive in gain.calc(&geometry, GainFilter::All).unwrap() {
            assert_eq!(drive.phase, 1.0);
            assert_eq!(drive.amp, 1.0);
        }
    }
}
