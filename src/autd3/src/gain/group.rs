/*
 * File: mod.rs
 * Project: group
 * Created Date: 18/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use bitvec::prelude::*;

use autd3_driver::{
    datagram::{Gain, GainAsAny, GainFilter},
    defined::Drive,
    error::AUTDInternalError,
    geometry::{Device, Transducer},
};

pub struct Group<
    K: Hash + Eq + Clone,
    T: Transducer,
    G: Gain<T>,
    F: Fn(&Device<T>, &T) -> Option<K>,
> {
    f: F,
    gain_map: HashMap<K, G>,
    _phantom: PhantomData<T>,
}

impl<K: Hash + Eq + Clone, T: Transducer, F: Fn(&Device<T>, &T) -> Option<K>>
    Group<K, T, Box<dyn Gain<T>>, F>
{
    /// Group by transducer
    ///
    /// # Arguments
    /// `f` - function to get key from transducer (currentry, transducer type annotation is required)
    ///
    /// # Examples
    ///
    /// ```
    /// # use autd3::prelude::*;
    /// # let gain : autd3::gain::Group<_, LegacyTransducer, _, _> =
    /// Group::new(|dev, tr: &LegacyTransducer| match tr.local_idx() {
    ///                 0..=100 => Some("null"),
    ///                 101.. => Some("focus"),
    ///                 _ => None,
    ///             })
    ///             .set("null", Null::new())
    ///             .set("focus", Focus::new(Vector3::new(0.0, 0.0, 150.0)));
    /// ```
    pub fn new(f: F) -> Group<K, T, Box<dyn Gain<T>>, F> {
        Group {
            f,
            gain_map: HashMap::new(),
            _phantom: PhantomData,
        }
    }
}

impl<K: Hash + Eq + Clone, T: Transducer, G: Gain<T>, F: Fn(&Device<T>, &T) -> Option<K>>
    Group<K, T, G, F>
{
    /// get gain map which maps device id to gain
    pub fn gain_map(&self) -> &HashMap<K, G> {
        &self.gain_map
    }
}

impl<'a, K: Hash + Eq + Clone, T: Transducer, F: Fn(&Device<T>, &T) -> Option<K>>
    Group<K, T, Box<dyn Gain<T> + 'a>, F>
{
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

impl<K: Hash + Eq + Clone, T: Transducer + 'static, F: Fn(&Device<T>, &T) -> Option<K>>
    Group<K, T, Box<dyn Gain<T>>, F>
{
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
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        G: Gain<T> + 'static,
        F: Fn(&Device<T>, &T) -> Option<K> + 'static,
    > autd3_driver::datagram::Datagram<T> for Group<K, T, G, F>
where
    autd3_driver::operation::GainOp<T, Self>: autd3_driver::operation::Operation<T>,
{
    type O1 = autd3_driver::operation::GainOp<T, Self>;
    type O2 = autd3_driver::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
        Ok((Self::O1::new(self), Self::O2::default()))
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        G: Gain<T> + 'static,
        F: Fn(&Device<T>, &T) -> Option<K> + 'static,
    > GainAsAny for Group<K, T, G, F>
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        G: Gain<T> + 'static,
        F: Fn(&Device<T>, &T) -> Option<K> + 'static,
    > Group<K, T, G, F>
{
    fn get_filters(
        &self,
        devices: &[&Device<T>],
    ) -> HashMap<K, HashMap<usize, BitVec<usize, Lsb0>>> {
        let mut filters = HashMap::new();
        devices.iter().for_each(|dev| {
            dev.iter().for_each(|tr| {
                if let Some(key) = (self.f)(dev, tr) {
                    if !filters.contains_key(&key) {
                        let mut filter = BitVec::<usize, Lsb0>::new();
                        filter.resize(dev.num_transducers(), false);
                        let filter: HashMap<usize, BitVec<usize, Lsb0>> =
                            [(dev.idx(), filter)].into();
                        filters.insert(key.clone(), filter);
                    }
                    if !filters.get_mut(&key).unwrap().contains_key(&dev.idx()) {
                        let mut filter = BitVec::<usize, Lsb0>::new();
                        filter.resize(dev.num_transducers(), false);
                        filters.get_mut(&key).unwrap().insert(dev.idx(), filter);
                    }
                    filters
                        .get_mut(&key)
                        .unwrap()
                        .get_mut(&dev.idx())
                        .unwrap()
                        .set(tr.local_idx(), true);
                }
            })
        });
        filters
    }
}

impl<
        K: Hash + Eq + Clone + 'static,
        T: Transducer + 'static,
        G: Gain<T> + 'static,
        F: Fn(&Device<T>, &T) -> Option<K> + 'static,
    > Gain<T> for Group<K, T, G, F>
{
    #[allow(clippy::uninit_vec)]
    fn calc(
        &self,
        devices: &[&Device<T>],
        _filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        let filters = self.get_filters(devices);

        let drives_cache = self
            .gain_map
            .iter()
            .map(|(k, g)| {
                let k = k.clone();
                let filter = if let Some(f) = filters.get(&k) {
                    f
                } else {
                    return Err(AUTDInternalError::UnknownGroupKey);
                };
                let d = g.calc(devices, GainFilter::Filter(filter))?;
                Ok((k, d))
            })
            .collect::<Result<HashMap<_, HashMap<usize, Vec<Drive>>>, _>>()?;

        devices
            .iter()
            .map(|dev| {
                let mut d: Vec<Drive> = Vec::with_capacity(dev.num_transducers());
                unsafe {
                    d.set_len(dev.num_transducers());
                }
                for tr in dev.iter() {
                    if let Some(key) = (self.f)(dev, tr) {
                        let g = if let Some(g) = drives_cache.get(&key) {
                            g
                        } else {
                            return Err(AUTDInternalError::UnspecifiedGroupKey);
                        };
                        d[tr.local_idx()] = g[&dev.idx()][tr.local_idx()];
                    } else {
                        d[tr.local_idx()] = Drive {
                            amp: 0.0,
                            phase: 0.0,
                        }
                    }
                }
                Ok((dev.idx(), d))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use autd3_driver::geometry::{IntoDevice, LegacyTransducer, Vector3};

    use super::*;

    use crate::{
        autd3_device::AUTD3,
        gain::{Focus, Null, Plane},
    };

    #[test]
    fn test_group() {
        let devices = [
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0),
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(1),
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(2),
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(3),
        ];

        let gain = Group::new(
            |dev, tr: &LegacyTransducer| match (dev.idx(), tr.local_idx()) {
                (0, 0..=99) => Some("null"),
                (0, 100..=199) => Some("plane"),
                (1, 200..) => Some("plane2"),
                _ => None,
            },
        )
        .set("null", Null::new())
        .set("plane", Plane::new(Vector3::zeros()))
        .set("plane2", Plane::new(Vector3::zeros()).with_amp(0.5));

        let drives = gain
            .calc(&devices.iter().collect::<Vec<_>>(), GainFilter::All)
            .unwrap();
        assert_eq!(drives.len(), 4);
        assert!(drives.values().all(|d| d.len() == AUTD3::NUM_TRANS_IN_UNIT));

        drives[&0].iter().enumerate().for_each(|(i, d)| match i {
            i if i <= 99 => {
                assert_eq!(d.phase, 0.0);
                assert_eq!(d.amp, 0.0);
            }
            i if i <= 199 => {
                assert_eq!(d.phase, 0.0);
                assert_eq!(d.amp, 1.0);
            }
            _ => {
                assert_eq!(d.phase, 0.0);
                assert_eq!(d.amp, 0.0);
            }
        });
        drives[&1].iter().enumerate().for_each(|(i, d)| match i {
            i if i <= 199 => {
                assert_eq!(d.phase, 0.0);
                assert_eq!(d.amp, 0.0);
            }
            _ => {
                assert_eq!(d.phase, 0.0);
                assert_eq!(d.amp, 0.5);
            }
        });
        drives[&2].iter().for_each(|d| {
            assert_eq!(d.phase, 0.0);
            assert_eq!(d.amp, 0.0);
        });
        drives[&3].iter().for_each(|d| {
            assert_eq!(d.phase, 0.0);
            assert_eq!(d.amp, 0.0);
        });
    }

    #[test]
    fn test_group_unknown_key() {
        let devices = [
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0),
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(1),
        ];

        let gain = Group::new(|_dev, tr: &LegacyTransducer| match tr.local_idx() {
            0..=99 => Some("plane"),
            100..=199 => Some("null"),
            _ => None,
        })
        .set("plane2", Plane::new(Vector3::zeros()));

        let drives = gain.calc(&devices.iter().collect::<Vec<_>>(), GainFilter::All);
        assert!(drives.is_err());
        assert_eq!(drives.unwrap_err(), AUTDInternalError::UnknownGroupKey);
    }

    #[test]
    fn test_group_unspecified_key() {
        let devices = [
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(0),
            AUTD3::new(Vector3::zeros(), Vector3::zeros()).into_device(1),
        ];

        let gain = Group::new(|_dev, tr: &LegacyTransducer| match tr.local_idx() {
            0..=99 => Some("plane"),
            100..=199 => Some("null"),
            _ => None,
        })
        .set("plane", Plane::new(Vector3::zeros()));

        let drives = gain.calc(&devices.iter().collect::<Vec<_>>(), GainFilter::All);
        assert!(drives.is_err());
        assert_eq!(drives.unwrap_err(), AUTDInternalError::UnspecifiedGroupKey);
    }

    #[test]
    fn test_get() {
        let gain: Group<_, LegacyTransducer, _, _> =
            Group::new(|dev, _tr: &LegacyTransducer| match dev.idx() {
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
