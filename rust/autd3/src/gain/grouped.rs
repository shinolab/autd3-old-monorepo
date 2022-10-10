/*
 * File: grouped.rs
 * Project: gain
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_core::{
    gain::{Gain, GainProps, IGain},
    geometry::{Geometry, Transducer},
};

use autd3_traits::Gain;

use crate::error::AUTDError;

/// Gain to produce single focal point
#[derive(Gain)]
pub struct Grouped<'a, T: Transducer> {
    props: GainProps<T>,
    gain_map: HashMap<usize, Box<dyn 'a + Gain<T>>>,
}

impl<'a, T: Transducer> Grouped<'a, T> {
    /// constructor
    pub fn new() -> Self {
        Self {
            props: GainProps::new(),
            gain_map: HashMap::new(),
        }
    }

    pub fn add<G: 'a + Gain<T>>(&mut self, id: usize, gain: G) {
        self.gain_map.insert(id, Box::new(gain));
    }
}

impl<'a, T: Transducer> IGain<T> for Grouped<'a, T>
where
    Grouped<'a, T>: Gain<T>,
{
    fn calc(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
        for gain in self.gain_map.values_mut() {
            gain.build(geometry)?;
        }

        self.gain_map.iter().try_for_each(|(dev_id, gain)| {
            if *dev_id >= geometry.num_devices() {
                return Err(AUTDError::GroupedOutOfRange(*dev_id, geometry.num_devices()).into());
            }

            self.props.drives[*dev_id..(*dev_id + autd3_core::NUM_TRANS_IN_UNIT)].copy_from_slice(
                &gain.drives()[*dev_id..(*dev_id + autd3_core::NUM_TRANS_IN_UNIT)],
            );

            Ok(())
        })
    }
}

impl<'a, T: Transducer> Default for Grouped<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}
