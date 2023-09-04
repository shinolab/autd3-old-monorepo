/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{
    datagram::{Gain, GainFilter},
    defined::Drive,
    error::AUTDInternalError,
    geometry::{Device, Transducer},
};
use bitvec::prelude::*;

use std::{cell::UnsafeCell, collections::HashMap};

/// Gain to cache the result of calculation
pub struct Cache<T: Transducer, G: Gain<T>> {
    gain: G,
    cache: UnsafeCell<HashMap<BitVec<usize, Lsb0>, HashMap<usize, Vec<Drive>>>>,
    _phantom: std::marker::PhantomData<T>,
}

pub trait IntoCache<T: Transducer, G: Gain<T>> {
    /// Cache the result of calculation
    fn with_cache(self) -> Cache<T, G>;
}

impl<T: Transducer, G: Gain<T>> IntoCache<T, G> for G {
    fn with_cache(self) -> Cache<T, G> {
        Cache::new(self)
    }
}

impl<T: Transducer, G: Gain<T> + Clone> Clone for Cache<T, G> {
    fn clone(&self) -> Self {
        Self {
            gain: self.gain.clone(),
            cache: unsafe { UnsafeCell::new(self.cache.get().as_ref().unwrap().clone()) },
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Transducer, G: Gain<T>> Cache<T, G> {
    /// constructor
    fn new(gain: G) -> Self {
        Self {
            gain,
            cache: UnsafeCell::new(Default::default()),
            _phantom: std::marker::PhantomData,
        }
    }

    fn init(&self, devices: &[&Device<T>]) -> Result<(), AUTDInternalError> {
        let key = BitVec::from_iter(devices.iter().map(|dev| dev.idx()));
        unsafe {
            if self.cache.get().as_ref().unwrap().get(&key).is_none() {
                self.cache
                    .get()
                    .as_mut()
                    .unwrap()
                    .insert(key, self.gain.calc(devices, GainFilter::All)?);
            }
        }
        Ok(())
    }

    /// get cached drives
    pub fn drives(
        &self,
        devices: &[&Device<T>],
    ) -> Result<&HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        self.init(devices)?;
        let key = BitVec::from_iter(devices.iter().map(|dev| dev.idx()));
        unsafe { Ok(self.cache.get().as_ref().unwrap().get(&key).unwrap()) }
    }

    /// get cached drives mutably
    pub fn drives_mut(
        &mut self,
        devices: &[&Device<T>],
    ) -> Result<&mut HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        self.init(devices)?;
        let key = BitVec::from_iter(devices.iter().map(|dev| dev.idx()));
        Ok(self.cache.get_mut().get_mut(&key).unwrap())
    }
}

impl<T: Transducer + 'static, G: Gain<T> + 'static> autd3_driver::datagram::Datagram<T>
    for Cache<T, G>
where
    autd3_driver::operation::GainOp<T, Self>: autd3_driver::operation::Operation<T>,
{
    type O1 = autd3_driver::operation::GainOp<T, Self>;
    type O2 = autd3_driver::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
        Ok((Self::O1::new(self), Self::O2::default()))
    }
}

impl<T: Transducer + 'static, G: Gain<T> + 'static> autd3_driver::datagram::GainAsAny
    for Cache<T, G>
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl<T: Transducer + 'static, G: Gain<T> + 'static> Gain<T> for Cache<T, G> {
    fn calc(
        &self,
        devices: &[&Device<T>],
        _filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        self.init(devices)?;
        let key = BitVec::from_iter(devices.iter().map(|dev| dev.idx()));
        Ok(unsafe { self.cache.get().as_ref().unwrap()[&key].clone() })
    }
}

#[cfg(test)]
mod tests {
    use autd3_driver::geometry::{IntoDevice, LegacyTransducer, Vector3};

    use super::*;

    use crate::{autd3_device::AUTD3, prelude::Plane};

    #[test]
    fn test_cache() {
        let device: Device<LegacyTransducer> =
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0);

        let mut gain = Plane::new(Vector3::zeros()).with_cache();

        let d = gain.calc(&[&device], GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 0.0);
            assert_eq!(drive.amp, 1.0);
        });

        let d = gain.drives_mut(&[&device]).unwrap();
        d.get_mut(&0).unwrap().iter_mut().for_each(|drive| {
            drive.phase = 1.0;
            drive.amp = 0.5;
        });

        let d = gain.calc(&[&device], GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 1.0);
            assert_eq!(drive.amp, 0.5);
        });
    }
}
