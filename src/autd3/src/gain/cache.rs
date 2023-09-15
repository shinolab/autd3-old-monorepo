/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{derive::prelude::*, geometry::Geometry};

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

/// Gain to cache the result of calculation
pub struct Cache<T: Transducer, G: Gain<T>> {
    gain: G,
    cache: Rc<RefCell<HashMap<usize, Vec<Drive>>>>,
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
            cache: self.cache.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Transducer, G: Gain<T>> Cache<T, G> {
    /// constructor
    fn new(gain: G) -> Self {
        Self {
            gain,
            cache: Rc::new(Default::default()),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn init(&self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
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
    pub fn drives(&self) -> Ref<'_, HashMap<usize, Vec<autd3_driver::defined::Drive>>> {
        self.cache.borrow()
    }

    /// get cached drives mutably
    pub fn drives_mut(&mut self) -> RefMut<'_, HashMap<usize, Vec<autd3_driver::defined::Drive>>> {
        self.cache.borrow_mut()
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
        geometry: &Geometry<T>,
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

    use autd3_driver::geometry::{IntoDevice, LegacyTransducer, Vector3};

    use super::*;

    use crate::{autd3_device::AUTD3, prelude::Plane};

    use autd3_derive::Gain;

    #[test]
    fn test_cache() {
        let geometry: Geometry<LegacyTransducer> =
            Geometry::new(vec![
                AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0)
            ]);

        let mut gain = Plane::new(Vector3::zeros()).with_cache();

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 0.0);
            assert_eq!(drive.amp, 1.0);
        });

        gain.drives_mut()
            .get_mut(&0)
            .unwrap()
            .iter_mut()
            .for_each(|drive| {
                drive.phase = 1.0;
                drive.amp = 0.5;
            });

        let d = gain.calc(&geometry, GainFilter::All).unwrap();
        d[&0].iter().for_each(|drive| {
            assert_eq!(drive.phase, 1.0);
            assert_eq!(drive.amp, 0.5);
        });
    }

    #[derive(Gain)]
    struct TestGain {
        pub calc_cnt: Arc<AtomicUsize>,
    }

    impl<T: Transducer> Gain<T> for TestGain {
        fn calc(
            &self,
            geometry: &Geometry<T>,
            filter: GainFilter,
        ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
            self.calc_cnt.fetch_add(1, Ordering::Relaxed);
            Ok(Self::transform(geometry, filter, |_, _| Drive {
                amp: 0.0,
                phase: 0.0,
            }))
        }
    }

    #[test]
    fn test_cache_calc_once() {
        let geometry: Geometry<LegacyTransducer> =
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
