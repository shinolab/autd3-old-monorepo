/*
 * File: grouped.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{error::AUTDInternalError, gain::Gain, geometry::*, Drive};
use autd3_derive::Gain;

/// Gain to group multiple gains
#[derive(Gain, Default)]
pub struct Grouped<T: Transducer + 'static> {
    gain_map: HashMap<Vec<usize>, Box<dyn Gain<T> + 'static>>,
}

impl<T: Transducer> Grouped<T> {
    /// constructor
    pub fn new() -> Self {
        Self {
            gain_map: HashMap::new(),
        }
    }

    fn calc_impl(
        drives: HashMap<&Vec<usize>, Vec<Drive>>,
        geometry: &Geometry<T>,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok((0..geometry.num_devices())
            .flat_map(|dev| {
                if let Some(i) = drives.keys().find(|indices| indices.contains(&dev)) {
                    geometry
                        .transducers_of(dev)
                        .map(|tr| drives[i][tr.idx()])
                        .collect()
                } else {
                    vec![
                        Drive {
                            amp: 0.0,
                            phase: 0.0
                        };
                        geometry.transducers_of(dev).len()
                    ]
                }
            })
            .collect())
    }

    /// add gain
    ///
    /// # Arguments
    ///
    /// * `id` - Device id
    /// * `gain` - Gain
    ///
    pub fn add<G: Gain<T> + 'static>(mut self, id: usize, gain: G) -> Self {
        self.gain_map.insert(vec![id], Box::new(gain));
        self
    }

    /// add boxed gain
    ///
    /// # Arguments
    ///
    /// * `id` - Device id
    /// * `gain` - Gain
    ///
    pub fn add_boxed(mut self, id: usize, gain: Box<dyn Gain<T>>) -> Self {
        self.gain_map.insert(vec![id], gain);
        self
    }

    /// add gain by group
    ///
    /// # Arguments
    ///
    /// * `ids` - Device ids
    /// * `gain` - Gain
    ///
    pub fn add_by_group<G: Gain<T> + 'static>(mut self, ids: &[usize], gain: G) -> Self {
        self.gain_map.insert(ids.to_vec(), Box::new(gain));
        self
    }

    /// add boxed gain by group
    ///
    /// # Arguments
    ///
    /// * `ids` - Device ids
    /// * `gain` - Gain
    ///
    pub fn add_boxed_by_group(mut self, ids: &[usize], gain: Box<dyn Gain<T>>) -> Self {
        self.gain_map.insert(ids.to_vec(), gain);
        self
    }

    /// get gain map which maps device id to gain
    pub fn gain_map(&self) -> &HashMap<Vec<usize>, Box<dyn Gain<T>>> {
        &self.gain_map
    }

    /// get Gain of specified device id
    ///
    /// # Arguments
    ///
    /// * `idx` - Device id
    ///
    /// # Returns
    ///
    /// * Gain of specified device id if the type is matched, otherwise None
    ///
    pub fn get_gain<G: Gain<T> + 'static>(&self, idx: usize) -> Option<&G> {
        match self.gain_map.keys().find(|&k| k.contains(&idx)) {
            Some(k) => self.gain_map[k].as_any().downcast_ref::<G>(),
            None => None,
        }
    }
}

impl<T: Transducer> Gain<T> for Grouped<T> {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Self::calc_impl(
            self.gain_map
                .iter()
                .map(|(k, gain)| -> Result<_, AUTDInternalError> {
                    let d = gain.calc(geometry)?;
                    Ok((k, d))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
            geometry,
        )
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
    fn test_grouped() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let mut grouped_gain = Grouped::new();
        grouped_gain = grouped_gain.add(0, Null::new());
        grouped_gain = grouped_gain.add(1, Plane::new(Vector3::zeros()));
        grouped_gain =
            grouped_gain.add_by_group(&[2, 3], Plane::new(Vector3::zeros()).with_amp(0.5));

        let drives = grouped_gain.calc(&geometry).unwrap();

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
    fn test_grouped_without_specified() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let grouped_gain = Grouped::new().add(0, Plane::new(Vector3::zeros()));

        let drives = grouped_gain.calc(&geometry).unwrap();

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
    fn get() {
        let grouped_gain = Grouped::<LegacyTransducer>::new()
            .add(0, Null::new())
            .add(1, Plane::new(Vector3::zeros()))
            .add_by_group(&[2, 3], Plane::new(Vector3::zeros()).with_amp(0.5));

        assert!(grouped_gain.get_gain::<Null>(0).is_some());
        assert!(grouped_gain.get_gain::<Focus>(0).is_none());

        assert!(grouped_gain.get_gain::<Plane>(1).is_some());
        assert!(grouped_gain.get_gain::<Null>(1).is_none());
        assert_eq!(grouped_gain.get_gain::<Plane>(1).unwrap().amp(), 1.0);

        assert!(grouped_gain.get_gain::<Plane>(2).is_some());
        assert!(grouped_gain.get_gain::<Null>(2).is_none());
        assert_eq!(grouped_gain.get_gain::<Plane>(2).unwrap().amp(), 0.5);

        assert!(grouped_gain.get_gain::<Plane>(3).is_some());
        assert!(grouped_gain.get_gain::<Null>(3).is_none());
        assert_eq!(grouped_gain.get_gain::<Plane>(3).unwrap().amp(), 0.5);

        assert!(grouped_gain.get_gain::<Null>(4).is_none());
        assert!(grouped_gain.get_gain::<Focus>(4).is_none());
        assert!(grouped_gain.get_gain::<Plane>(4).is_none());
    }
}
