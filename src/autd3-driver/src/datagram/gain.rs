/*
 * File: gain.rs
 * Project: datagram
 * Created Date: 29/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    common::{Drive, EmitIntensity},
    error::AUTDInternalError,
    geometry::{Device, Geometry, Transducer},
};

use bitvec::prelude::*;

pub enum GainFilter<'a> {
    All,
    Filter(&'a HashMap<usize, BitVec<usize, Lsb0>>),
}

pub trait GainAsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Gain controls amplitude and phase of each transducer.
pub trait Gain: GainAsAny {
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError>;
    fn transform<F: Fn(&Device, &Transducer) -> Drive + Sync + Send>(
        geometry: &Geometry,
        filter: GainFilter,
        f: F,
    ) -> HashMap<usize, Vec<Drive>>
    where
        Self: Sized,
    {
        match filter {
            GainFilter::All => geometry
                .devices()
                .map(|dev| (dev.idx(), dev.iter().map(|tr| f(dev, tr)).collect()))
                .collect(),
            GainFilter::Filter(filter) => geometry
                .devices()
                .filter_map(|dev| {
                    filter.get(&dev.idx()).map(|filter| {
                        (
                            dev.idx(),
                            dev.iter()
                                .map(|tr| {
                                    if filter[tr.local_idx()] {
                                        f(dev, tr)
                                    } else {
                                        Drive {
                                            phase: 0.,
                                            amp: EmitIntensity::MIN,
                                        }
                                    }
                                })
                                .collect(),
                        )
                    })
                })
                .collect(),
        }
    }
}

impl<'a> GainAsAny for Box<dyn Gain + 'a> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn as_any(&self) -> &dyn std::any::Any {
        self.as_ref().as_any()
    }
}

impl<'a> Gain for Box<dyn Gain + 'a> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        self.as_ref().calc(geometry, filter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{geometry::tests::create_geometry, operation::tests::NullGain};

    #[test]
    fn test_gain_as_any() {
        let g = NullGain {};
        assert!(g.as_any().is::<NullGain>());
    }

    #[test]
    fn test_gain_transform_all() {
        let geometry = create_geometry(2, 249);
        let g = NullGain {}.calc(&geometry, GainFilter::All).unwrap();

        assert_eq!(g.len(), 2);

        assert!(g.contains_key(&0));

        let d0 = g.get(&0).unwrap();
        assert_eq!(d0.len(), 249);
        d0.iter().for_each(|d| {
            assert_eq!(d.amp.normalized(), 1.);
            assert_eq!(d.phase, 2.);
        });

        assert!(g.contains_key(&1));
        let d1 = g.get(&1).unwrap();
        assert_eq!(d1.len(), 249);
        d1.iter().for_each(|d| {
            assert_eq!(d.amp.normalized(), 1.);
            assert_eq!(d.phase, 2.);
        });
    }

    #[test]
    fn test_gain_transform_all_enabled() {
        let mut geometry = create_geometry(2, 249);
        geometry[0].enable = false;

        let g = NullGain {}.calc(&geometry, GainFilter::All).unwrap();

        assert_eq!(g.len(), 1);

        assert!(!g.contains_key(&0));

        assert!(g.contains_key(&1));
        let d1 = g.get(&1).unwrap();
        assert_eq!(d1.len(), 249);
        d1.iter().for_each(|d| {
            assert_eq!(d.amp.normalized(), 1.);
            assert_eq!(d.phase, 2.);
        });
    }

    #[test]
    fn test_gain_transform_filtered() {
        let geometry = create_geometry(3, 249);
        let filter = geometry
            .iter()
            .take(2)
            .map(|dev| {
                (
                    dev.idx(),
                    dev.iter().map(|tr| tr.local_idx() < 100).collect(),
                )
            })
            .collect::<HashMap<_, _>>();
        let g = NullGain {}
            .calc(&geometry, GainFilter::Filter(&filter))
            .unwrap();

        assert_eq!(g.len(), 2);

        assert!(g.contains_key(&0));
        let d0 = g.get(&0).unwrap();
        assert_eq!(d0.len(), 249);
        d0.iter().enumerate().for_each(|(i, d)| {
            if i < 100 {
                assert_eq!(d.amp.normalized(), 1.);
                assert_eq!(d.phase, 2.);
            } else {
                assert_eq!(d.amp.normalized(), 0.);
                assert_eq!(d.phase, 0.);
            }
        });

        assert!(g.contains_key(&1));
        let d1 = g.get(&1).unwrap();
        assert_eq!(d1.len(), 249);
        d1.iter().enumerate().for_each(|(i, d)| {
            if i < 100 {
                assert_eq!(d.amp.normalized(), 1.);
                assert_eq!(d.phase, 2.);
            } else {
                assert_eq!(d.amp.normalized(), 0.);
                assert_eq!(d.phase, 0.);
            }
        });

        assert!(!g.contains_key(&2));
    }

    #[test]
    fn test_gain_transform_filtered_enabled() {
        let mut geometry = create_geometry(2, 249);
        geometry[0].enable = false;

        let filter = geometry
            .iter()
            .map(|dev| {
                (
                    dev.idx(),
                    dev.iter().map(|tr| tr.local_idx() < 100).collect(),
                )
            })
            .collect::<HashMap<_, _>>();
        let g = NullGain {}
            .calc(&geometry, GainFilter::Filter(&filter))
            .unwrap();

        assert_eq!(g.len(), 1);

        assert!(!g.contains_key(&0));

        assert!(g.contains_key(&1));
        let d1 = g.get(&1).unwrap();
        assert_eq!(d1.len(), 249);
        d1.iter().enumerate().for_each(|(i, d)| {
            if i < 100 {
                assert_eq!(d.amp.normalized(), 1.);
                assert_eq!(d.phase, 2.);
            } else {
                assert_eq!(d.amp.normalized(), 0.);
                assert_eq!(d.phase, 0.);
            }
        });
    }
}
