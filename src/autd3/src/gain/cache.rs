/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{derive::prelude::*, geometry::Geometry};

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

/// Gain to cache the result of calculation
pub struct Cache<G: Gain> {
    gain: G,
    cache: Rc<RefCell<HashMap<usize, Vec<Drive>>>>,
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
            cache: Rc::new(Default::default()),
        }
    }

    pub fn init(&self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        if self.cache.borrow().len() != geometry.devices().count()
            || geometry
                .devices()
                .any(|dev| !self.cache.borrow().contains_key(&dev.idx()))
        {
            self.gain
                .calc(geometry, GainFilter::All)?
                .iter()
                .for_each(|(k, v)| {
                    self.cache.borrow_mut().insert(*k, v.clone());
                });
        }
        Ok(())
    }

    /// get cached drives
    pub fn drives(&self) -> Ref<'_, HashMap<usize, Vec<autd3_driver::common::Drive>>> {
        self.cache.borrow()
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
        Ok(self.cache.borrow().clone())
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
        common::EmitIntensity,
        geometry::{IntoDevice, Vector3},
    };

    use super::*;

    use crate::prelude::Plane;

    use autd3_derive::Gain;

    #[test]
    fn test_cache() {
        let geometry: Geometry = Geometry::new(vec![AUTD3::new(Vector3::zeros()).into_device(0)]);

        let mut gain = Plane::new(Vector3::zeros()).with_cache();

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase.value(), 0);
            assert_eq!(drive.intensity.value(), 0xFF);
        });

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase.value(), 1);
            assert_eq!(drive.intensity.value(), 0x1F);
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
                intensity: EmitIntensity::MIN,
                phase: Phase::new(0),
            }))
        }
    }

    #[test]
    fn test_cache_calc_once() {
        let geometry: Geometry = Geometry::new(vec![AUTD3::new(Vector3::zeros()).into_device(0)]);

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
