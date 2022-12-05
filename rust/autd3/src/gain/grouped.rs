/*
 * File: grouped.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{
    gain::{Gain, GainProps},
    geometry::{Geometry, Transducer},
    Drive,
};

use autd3_traits::Gain;

use crate::error::AUTDError;

/// Gain to produce single focal point
#[derive(Gain)]
pub struct Grouped {
    props: GainProps,
    gain_map: HashMap<usize, Vec<Drive>>,
}

impl Grouped {
    /// constructor
    pub fn new() -> Self {
        Self {
            props: GainProps::new(),
            gain_map: HashMap::new(),
        }
    }

    pub fn add<T: Transducer, G: Gain<T>>(
        &mut self,
        id: usize,
        gain: G,
        geometry: &Geometry<T>,
    ) -> anyhow::Result<()> {
        let mut gain = gain;
        gain.build(geometry)?;
        self.gain_map.insert(id, gain.take_drives());
        Ok(())
    }

    fn calc<T: Transducer>(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        self.gain_map.iter().try_for_each(|(dev_id, gain)| {
            if *dev_id >= geometry.num_devices() {
                return Err(AUTDError::GroupedOutOfRange(*dev_id, geometry.num_devices()).into());
            }
            let start = if *dev_id == 0 {
                0
            } else {
                geometry.device_map()[*dev_id - 1]
            };
            let end = start + geometry.device_map()[*dev_id];
            self.props.drives[start..end].copy_from_slice(&gain[start..end]);
            Ok(())
        })
    }
}

impl Default for Grouped {
    fn default() -> Self {
        Self::new()
    }
}
