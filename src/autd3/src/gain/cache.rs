/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{derive::prelude::*, geometry::Geometry};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

/// Gain to cache the result of calculation
pub struct Cache<G: Gain> {
    gain: G,
    cache: Arc<Mutex<HashMap<usize, Vec<Drive>>>>,
}

pub trait IntoCache<G: Gain> {
    /// Cache the result of calculation
    fn with_cache(self) -> Cache<G>;
}

impl<G: Gain> IntoCache<G> for G {
    fn with_cache(self) -> Cache<G> {
        Cache::new(self)
    }
}

impl<G: Gain + Clone> Clone for Cache<G> {
    fn clone(&self) -> Self {
        Self {
            gain: self.gain.clone(),
            cache: self.cache.clone(),
        }
    }
}

impl<G: Gain> Cache<G> {
    /// constructor
    fn new(gain: G) -> Self {
        Self {
            gain,
            cache: Arc::new(Default::default()),
        }
    }

    pub fn init(&self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        if self.cache.lock().unwrap().len() != geometry.devices().count()
            || geometry
                .devices()
                .any(|dev| !self.cache.lock().unwrap().contains_key(&dev.idx()))
        {
            self.gain
                .calc(geometry, GainFilter::All)?
                .iter()
                .for_each(|(k, v)| {
                    self.cache.lock().unwrap().insert(*k, v.clone());
                });
        }
        Ok(())
    }

    /// get cached drives
    pub fn drives(&self) -> MutexGuard<'_, HashMap<usize, Vec<autd3_driver::common::Drive>>> {
        self.cache.lock().unwrap()
    }

    /// get cached drives mutably
    pub fn drives_mut(
        &mut self,
    ) -> MutexGuard<'_, HashMap<usize, Vec<autd3_driver::common::Drive>>> {
        self.cache.lock().unwrap()
    }
}

impl<G: Gain + 'static> autd3_driver::datagram::Datagram for Cache<G>
where
    autd3_driver::operation::GainOp<Self>: autd3_driver::operation::Operation,
{
    type O1 = autd3_driver::operation::GainOp<Self>;
    type O2 = autd3_driver::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
        Ok((Self::O1::new(self), Self::O2::default()))
    }
}

impl<G: Gain + 'static> autd3_driver::datagram::GainAsAny for Cache<G> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl<G: Gain + 'static> Gain for Cache<G> {
    fn calc(
        &self,
        geometry: &Geometry,
        _filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        self.init(geometry)?;
        Ok(self.cache.lock().unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    use autd3_driver::{
        autd3_device::AUTD3,
        common::Amplitude,
        geometry::{IntoDevice, Vector3},
    };

    use super::*;

    use crate::prelude::Plane;

    use autd3_derive::Gain;

    #[test]
    fn test_cache() {
        let geometry: Geometry =
            Geometry::new(vec![
                AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0)
            ]);

        let mut gain = Plane::new(Vector3::zeros()).with_cache();

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 0.0);
            assert_eq!(drive.amp.value(), 1.0);
        });

        gain.drives_mut()
            .get_mut(&0)
            .unwrap()
            .iter_mut()
            .for_each(|drive| {
                drive.phase = 1.0;
                drive.amp = Amplitude::new_clamped(0.5);
            });

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 1.0);
            assert_eq!(drive.amp.value(), 0.5);
        });
    }

    #[derive(Gain)]
    struct TestGain {
        pub calc_cnt: Arc<AtomicUsize>,
    }

    impl Gain for TestGain {
        fn calc(
            &self,
            geometry: &Geometry,
            filter: GainFilter,
        ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
            self.calc_cnt.fetch_add(1, Ordering::Relaxed);
            Ok(Self::transform(geometry, filter, |_, _| Drive {
                amp: Amplitude::MIN,
                phase: 0.0,
            }))
        }
    }

    #[test]
    fn test_cache_calc_once() {
        let geometry: Geometry =
            Geometry::new(vec![
                AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0)
            ]);

        let calc_cnt = Arc::new(AtomicUsize::new(0));

        let gain = TestGain {
            calc_cnt: calc_cnt.clone(),
        }
        .with_cache();

        assert_eq!(calc_cnt.load(Ordering::Relaxed), 0);
        let _ = gain.calc(&geometry, GainFilter::All).unwrap();
        assert_eq!(calc_cnt.load(Ordering::Relaxed), 1);
        let _ = gain.calc(&geometry, GainFilter::All).unwrap();
        assert_eq!(calc_cnt.load(Ordering::Relaxed), 1);
        let _ = gain.calc(&geometry, GainFilter::All).unwrap();
        assert_eq!(calc_cnt.load(Ordering::Relaxed), 1);
    }
}
