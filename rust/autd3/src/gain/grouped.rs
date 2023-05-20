/*
 * File: grouped.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{error::AUTDInternalError, gain::Gain, geometry::*, Drive};
use autd3_traits::Gain;

/// Gain to produce single focal point
#[derive(Gain, Default)]
pub struct Grouped<'a, T: Transducer> {
    gain_map: HashMap<usize, Box<dyn Gain<T> + 'a>>,
}

impl<'a, T: Transducer> Grouped<'a, T> {
    /// constructor
    pub fn new() -> Self {
        Self {
            gain_map: HashMap::new(),
        }
    }

    pub fn calc_impl(
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

    pub fn add<G: 'a + Gain<T>>(&mut self, id: usize, gain: G) {
        self.gain_map.insert(id, Box::new(gain));
    }

    pub fn add_boxed(&mut self, id: usize, gain: Box<dyn Gain<T> + 'a>) {
        self.gain_map.insert(id, gain);
    }
}

impl<'a, T: Transducer> Gain<T> for Grouped<'a, T> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let mut drives = HashMap::new();
        for (&i, gain) in &mut self.gain_map {
            let d = gain.calc(geometry)?;
            drives.insert(i, d);
        }
        Self::calc_impl(drives, geometry)
    }
}
