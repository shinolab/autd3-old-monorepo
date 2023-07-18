/*
 * File: grouped.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{error::AUTDInternalError, gain::Gain, geometry::*, Drive};
use autd3_traits::Gain;

/// Gain to group multiple gains
#[derive(Gain, Default)]
pub struct Grouped<T: Transducer + 'static> {
    gain_map: HashMap<usize, Box<dyn Gain<T> + 'static>>,
}

impl<T: Transducer> Grouped<T> {
    /// constructor
    pub fn new() -> Self {
        Self {
            gain_map: HashMap::new(),
        }
    }

    fn calc_impl(
        mut drives: HashMap<usize, Vec<Drive>>,
        geometry: &Geometry<T>,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok((0..geometry.num_devices())
            .flat_map(|i| {
                drives
                    .get_mut(&i)
                    .map(|g| {
                        let start = if i == 0 {
                            0
                        } else {
                            geometry.device_map()[i - 1]
                        };
                        let end = start + geometry.device_map()[i];
                        g[start..end].to_vec()
                    })
                    .unwrap_or_else(|| {
                        vec![
                            Drive {
                                phase: 0.0,
                                amp: 0.0,
                            };
                            geometry.device_map()[i]
                        ]
                    })
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
        self.gain_map.insert(id, Box::new(gain));
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
        self.gain_map.insert(id, gain);
        self
    }

    /// get gain map which maps device id to gain
    pub fn gain_map(&self) -> &HashMap<usize, Box<dyn Gain<T>>> {
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
        if !self.gain_map.contains_key(&idx) {
            return None;
        }
        self.gain_map[&idx].as_any().downcast_ref::<G>()
    }
}

impl<T: Transducer> Gain<T> for Grouped<T> {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Self::calc_impl(
            self.gain_map
                .iter()
                .map(|(&i, gain)| -> Result<_, AUTDInternalError> {
                    let d = gain.calc(geometry)?;
                    Ok((i, d))
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

    use crate::prelude::{Null, Plane};
    use crate::tests::GeometryBuilder;

    #[test]
    fn test_grouped() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::new(10.0, 0.0, 0.0), Vector3::zeros()))
            .build()
            .unwrap();

        let mut grouped_gain = Grouped::new();
        grouped_gain = grouped_gain.add(0, Null::new());
        grouped_gain = grouped_gain.add(1, Plane::new(Vector3::zeros()));

        let drives = grouped_gain.calc(&geometry).unwrap();

        for (i, drive) in drives.iter().enumerate() {
            if i < geometry.device_map()[0] {
                assert_eq!(drive.phase, 0.0);
                assert_eq!(drive.amp, 0.0);
            } else {
                assert_eq!(drive.phase, 0.0);
                assert_eq!(drive.amp, 1.0);
            }
        }
    }
}
