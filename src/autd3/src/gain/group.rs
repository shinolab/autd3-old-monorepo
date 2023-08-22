/*
 * File: mod.rs
 * Project: group
 * Created Date: 18/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use bitvec::prelude::*;

use autd3_core::{
    error::AUTDInternalError,
    gain::{Gain, GainAsAny, GainFilter},
    geometry::*,
    Drive,
};

pub trait GroupBy {}

impl GroupBy for () {}

pub struct ByDevice<K, F> {
    phantom: PhantomData<K>,
    f: F,
}

impl<K, F> GroupBy for ByDevice<K, F> {}

pub struct ByTransducer<K, F> {
    phantom: PhantomData<K>,
    f: F,
}

impl<K, F> GroupBy for ByTransducer<K, F> {}

pub struct Group<B: GroupBy, K: Hash + Eq + Clone, T: Transducer, G: Gain<T>> {
    group_by: B,
    gain_map: HashMap<K, G>,
    _phantom: PhantomData<T>,
}

impl<K: Hash + Eq + Clone, T: Transducer> Group<(), K, T, Box<dyn Gain<T>>> {
    /// Group by device
    ///
    /// # Arguments
    /// `f` - function to get key from device index
    ///
    /// # Examples
    ///
    /// ```
    /// # use autd3::prelude::*;
    /// # let gain : autd3::gain::group::Group<_, _, LegacyTransducer, _> =
    /// Group::by_device(|dev| match dev {
    ///                 0 => Some("null"),
    ///                 1 => Some("focus"),
    ///                 _ => None,
    ///             })
    ///             .set("null", Null::new())
    ///             .set("focus", Focus::new(Vector3::new(0.0, 0.0, 150.0)));
    /// ```
    pub fn by_device<F>(f: F) -> Group<ByDevice<K, F>, K, T, Box<dyn Gain<T>>>
    where
        K: Hash + Eq + Clone,
        T: Transducer,
        F: Fn(usize) -> Option<K>,
    {
        Group {
            group_by: ByDevice {
                phantom: PhantomData,
                f,
            },
            gain_map: HashMap::new(),
            _phantom: PhantomData,
        }
    }

    /// Group by transducer
    ///
    /// # Arguments
    /// `f` - function to get key from transducer (currentry, transducer type annotation is required)
    ///
    /// # Examples
    ///
    /// ```
    /// # use autd3::prelude::*;
    /// # let gain : autd3::gain::group::Group<_, _, LegacyTransducer, _> =
    /// Group::by_transducer(|tr: &LegacyTransducer| match tr.idx() {
    ///                 0..=100 => Some("null"),
    ///                 101.. => Some("focus"),
    ///                 _ => None,
    ///             })
    ///             .set("null", Null::new())
    ///             .set("focus", Focus::new(Vector3::new(0.0, 0.0, 150.0)));
    /// ```
    pub fn by_transducer<F>(f: F) -> Group<ByTransducer<K, F>, K, T, Box<dyn Gain<T>>>
    where
        K: Hash + Eq + Clone,
        T: Transducer,
        F: Fn(&T) -> Option<K>,
    {
        Group {
            group_by: ByTransducer {
                phantom: PhantomData,
                f,
            },
            gain_map: HashMap::new(),
            _phantom: PhantomData,
        }
    }
}

impl<B: GroupBy, K: Hash + Eq + Clone, T: Transducer, G: Gain<T>> Group<B, K, T, G> {
    /// get gain map which maps device id to gain
    pub fn gain_map(&self) -> &HashMap<K, G> {
        &self.gain_map
    }
}

impl<'a, B: GroupBy, K: Hash + Eq + Clone, T: Transducer> Group<B, K, T, Box<dyn Gain<T> + 'a>> {
    /// set gain
    ///
    /// # Arguments
    ///
    /// * `key` - key
    /// * `gain` - Gain
    ///
    pub fn set<G: Gain<T> + 'a>(mut self, key: K, gain: G) -> Self {
        self.gain_map.insert(key, Box::new(gain));
        self
    }
}

impl<B: GroupBy, K: Hash + Eq + Clone, T: Transducer + 'static> Group<B, K, T, Box<dyn Gain<T>>> {
    /// get Gain of specified key
    ///
    /// # Arguments
    ///
    /// * `key` - key
    ///
    /// # Returns
    ///
    /// * Gain of specified key if exists and the type is matched, otherwise None
    ///
    pub fn get<G: Gain<T> + 'static>(&self, key: K) -> Option<&G> {
        self.gain_map
            .get(&key)
            .and_then(|g| g.as_ref().as_any().downcast_ref::<G>())
    }
}

impl<
        'a,
        K: Hash + Eq + Clone + 'a,
        T: Transducer + 'a,
        F: Fn(usize) -> Option<K> + 'a,
        G: Gain<T> + 'a,
    > Group<ByDevice<K, F>, K, T, G>
{
    fn get_filters(&self, geometry: &Geometry<T>) -> HashMap<K, BitVec> {
        let mut filters = HashMap::new();
        (0..geometry.num_devices()).for_each(|dev| {
            if let Some(key) = (self.group_by.f)(dev) {
                if !filters.contains_key(&key) {
                    let mut filter = BitVec::new();
                    filter.resize(geometry.num_transducers(), false);
                    filters.insert(key.clone(), filter);
                }
                let filter = filters.get_mut(&key).unwrap();
                geometry
                    .transducers_of(dev)
                    .for_each(|tr| filter.set(tr.idx(), true));
            }
        });
        filters
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        F: Fn(&T) -> Option<K> + 'static,
        G: Gain<T> + 'static,
    > Group<ByTransducer<K, F>, K, T, G>
{
    fn get_filters(&self, geometry: &Geometry<T>) -> HashMap<K, BitVec> {
        let mut filters = HashMap::new();
        geometry.transducers().for_each(|tr| {
            if let Some(key) = (self.group_by.f)(tr) {
                if !filters.contains_key(&key) {
                    let mut filter = BitVec::<usize, Lsb0>::new();
                    filter.resize(geometry.num_transducers(), false);
                    filters.insert(key.clone(), filter);
                }
                filters.get_mut(&key).unwrap().set(tr.idx(), true);
            }
        });
        filters
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        F: Fn(usize) -> Option<K> + 'static,
        G: Gain<T> + 'static,
    > autd3_core::datagram::Datagram<T> for Group<ByDevice<K, F>, K, T, G>
{
    type H = autd3_core::NullHeader;
    type B = T::Gain;

    fn operation(
        &self,
        geometry: &Geometry<T>,
    ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
        Ok((
            Self::H::default(),
            <Self::B as autd3_core::GainOp>::new(self.calc(geometry, GainFilter::All)?, || {
                geometry.transducers().map(|tr| tr.cycle()).collect()
            }),
        ))
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        F: Fn(&T) -> Option<K> + 'static,
        G: Gain<T> + 'static,
    > autd3_core::datagram::Datagram<T> for Group<ByTransducer<K, F>, K, T, G>
{
    type H = autd3_core::NullHeader;
    type B = T::Gain;

    fn operation(
        &self,
        geometry: &Geometry<T>,
    ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
        Ok((
            Self::H::default(),
            <Self::B as autd3_core::GainOp>::new(self.calc(geometry, GainFilter::All)?, || {
                geometry.transducers().map(|tr| tr.cycle()).collect()
            }),
        ))
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        F: Fn(usize) -> Option<K> + 'static,
        G: Gain<T> + 'static,
    > GainAsAny for Group<ByDevice<K, F>, K, T, G>
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        F: Fn(&T) -> Option<K> + 'static,
        G: Gain<T> + 'static,
    > GainAsAny for Group<ByTransducer<K, F>, K, T, G>
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        F: Fn(usize) -> Option<K> + 'static,
        G: Gain<T> + 'static,
    > Gain<T> for Group<ByDevice<K, F>, K, T, G>
{
    #[allow(clippy::uninit_vec)]
    fn calc(
        &self,
        geometry: &Geometry<T>,
        _filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        let mut drives = Vec::with_capacity(geometry.num_transducers());
        unsafe {
            drives.set_len(geometry.num_transducers());
        }

        let filters = self.get_filters(geometry);

        let mut drives_cache = HashMap::new();
        for dev in 0..geometry.num_devices() {
            if let Some(key) = (self.group_by.f)(dev) {
                if let Some(gain) = self.gain_map.get(&key) {
                    if !drives_cache.contains_key(&key) {
                        let filter = filters.get(&key).unwrap();
                        drives_cache.insert(
                            key.clone(),
                            gain.calc(geometry, GainFilter::Filter(filter))?,
                        );
                    }
                    let d = drives_cache.get(&key).unwrap();
                    geometry
                        .transducers_of(dev)
                        .for_each(|tr| drives[tr.idx()] = d[tr.idx()]);
                } else {
                    geometry.transducers_of(dev).for_each(|tr| {
                        drives[tr.idx()] = Drive {
                            amp: 0.0,
                            phase: 0.0,
                        }
                    });
                }
            } else {
                geometry.transducers_of(dev).for_each(|tr| {
                    drives[tr.idx()] = Drive {
                        amp: 0.0,
                        phase: 0.0,
                    }
                });
            }
        }
        Ok(drives)
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        F: Fn(&T) -> Option<K> + 'static,
        G: Gain<T> + 'static,
    > Gain<T> for Group<ByTransducer<K, F>, K, T, G>
{
    #[allow(clippy::uninit_vec)]
    fn calc(
        &self,
        geometry: &Geometry<T>,
        _filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        let mut drives = Vec::with_capacity(geometry.num_transducers());
        unsafe {
            drives.set_len(geometry.num_transducers());
        }

        let filters = self.get_filters(geometry);

        let mut drives_cache = HashMap::new();

        for tr in geometry.transducers() {
            if let Some(key) = (self.group_by.f)(tr) {
                if let Some(gain) = self.gain_map.get(&key) {
                    if !drives_cache.contains_key(&key) {
                        let filter = filters.get(&key).unwrap();
                        drives_cache.insert(
                            key.clone(),
                            gain.calc(geometry, GainFilter::Filter(filter))?,
                        );
                    }
                    let d = drives_cache.get(&key).unwrap();
                    drives[tr.idx()] = d[tr.idx()];
                } else {
                    drives[tr.idx()] = Drive {
                        amp: 0.0,
                        phase: 0.0,
                    }
                }
            } else {
                drives[tr.idx()] = Drive {
                    amp: 0.0,
                    phase: 0.0,
                }
            }
        }

        Ok(drives)
    }
}

#[cfg(test)]
mod tests {
    use autd3_core::autd3_device::AUTD3;
    use autd3_core::geometry::LegacyTransducer;

    use super::*;

    use crate::prelude::{Focus, Null, Plane};
    use crate::tests::GeometryBuilder;

    #[test]
    fn test_group_by_device() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let gain = Group::by_device(|dev| match dev {
            0 => Some("null"),
            1 => Some("plane"),
            2 | 3 => Some("plane2"),
            _ => None,
        })
        .set("null", Null::new())
        .set("plane", Plane::new(Vector3::zeros()))
        .set("plane2", Plane::new(Vector3::zeros()).with_amp(0.5));

        let drives = gain.calc(&geometry, GainFilter::All).unwrap();

        for (i, drive) in drives.iter().enumerate() {
            match i {
                i if i < geometry.device_map()[0] => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 0.0);
                }
                i if i < geometry.device_map()[0] + geometry.device_map()[1] => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 1.0);
                }
                _ => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 0.5);
                }
            }
        }
    }

    #[test]
    fn test_group_by_transducer() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let gain = Group::by_transducer(|tr: &LegacyTransducer| match tr.idx() {
            0..=99 => Some("null"),
            100..=199 => Some("plane"),
            200.. => Some("plane2"),
            _ => None,
        })
        .set("null", Null::new())
        .set("plane", Plane::new(Vector3::zeros()))
        .set("plane2", Plane::new(Vector3::zeros()).with_amp(0.5));

        let drives = gain.calc(&geometry, GainFilter::All).unwrap();

        for (i, drive) in drives.iter().enumerate() {
            match i {
                i if i <= 99 => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 0.0);
                }
                i if i <= 199 => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 1.0);
                }
                _ => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 0.5);
                }
            }
        }
    }

    #[test]
    fn test_group_by_device_without_specified() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let gain = Group::by_device(|dev| match dev {
            0 => Some("plane"),
            1 => Some("null"),
            _ => None,
        })
        .set("plane", Plane::new(Vector3::zeros()));

        let drives = gain.calc(&geometry, GainFilter::All).unwrap();

        assert_eq!(drives.len(), geometry.num_transducers());

        for (i, drive) in drives.iter().enumerate() {
            match i {
                i if i < geometry.device_map()[0] => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 1.0);
                }
                _ => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 0.0);
                }
            }
        }
    }

    #[test]
    fn test_group_by_transdcuer_without_specified() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let gain = Group::by_transducer(|tr: &LegacyTransducer| match tr.idx() {
            0..=99 => Some("plane"),
            100..=199 => Some("null"),
            _ => None,
        })
        .set("plane", Plane::new(Vector3::zeros()));

        let drives = gain.calc(&geometry, GainFilter::All).unwrap();

        assert_eq!(drives.len(), geometry.num_transducers());

        for (i, drive) in drives.iter().enumerate() {
            match i {
                i if i <= 99 => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 1.0);
                }
                _ => {
                    assert_eq!(drive.phase, 0.0);
                    assert_eq!(drive.amp, 0.0);
                }
            }
        }
    }

    #[test]
    fn test_get() {
        let gain: Group<_, _, LegacyTransducer, _> = Group::by_device(|dev| match dev {
            0 => Some("null"),
            1 => Some("plane"),
            2 | 3 => Some("plane2"),
            _ => None,
        })
        .set("null", Null::new())
        .set("plane", Plane::new(Vector3::zeros()))
        .set("plane2", Plane::new(Vector3::zeros()).with_amp(0.5));

        assert!(gain.get::<Null>("null").is_some());
        assert!(gain.get::<Focus>("null").is_none());

        assert!(gain.get::<Plane>("plane").is_some());
        assert!(gain.get::<Null>("plane").is_none());
        assert_eq!(gain.get::<Plane>("plane").unwrap().amp(), 1.0);

        assert!(gain.get::<Plane>("plane2").is_some());
        assert!(gain.get::<Null>("plane2").is_none());
        assert_eq!(gain.get::<Plane>("plane2").unwrap().amp(), 0.5);

        assert!(gain.get::<Null>("focus").is_none());
        assert!(gain.get::<Focus>("focus").is_none());
        assert!(gain.get::<Plane>("focus").is_none());
    }
}
