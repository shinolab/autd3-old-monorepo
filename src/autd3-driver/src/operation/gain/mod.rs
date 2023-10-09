/*
 * File: mod.rs
 * Project: gain
 * Created Date: 08/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub mod advanced;
pub mod advanced_phase;
mod amplitude;
mod gain_control_flags;
pub mod legacy;

pub use amplitude::AmplitudeOp;
pub use gain_control_flags::GainControlFlags;

use std::collections::HashMap;

use crate::{
    datagram::Gain,
    defined::Drive,
    derive::prelude::GainFilter,
    geometry::{Device, Transducer},
};

use super::Operation;

pub struct GainOp<T: Transducer, G: Gain<T>> {
    gain: G,
    drives: HashMap<usize, Vec<Drive>>,
    remains: HashMap<usize, usize>,
    phantom: std::marker::PhantomData<T>,
}

pub trait GainOpDelegate<T: Transducer> {
    fn init(
        geometry: &crate::derive::prelude::Geometry<T>,
    ) -> Result<HashMap<usize, usize>, crate::derive::prelude::AUTDInternalError>;
    fn pack(
        drives: &HashMap<usize, Vec<Drive>>,
        remains: &HashMap<usize, usize>,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, crate::derive::prelude::AUTDInternalError>;
}

impl<T: Transducer, G: Gain<T>> GainOp<T, G> {
    pub fn new(gain: G) -> Self {
        Self {
            gain,
            drives: Default::default(),
            remains: Default::default(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Transducer, G: Gain<T>> Operation<T> for GainOp<T, G> {
    fn init(
        &mut self,
        geometry: &crate::derive::prelude::Geometry<T>,
    ) -> Result<(), crate::derive::prelude::AUTDInternalError> {
        self.drives = self.gain.calc(geometry, GainFilter::All)?;
        self.remains = T::GainOp::init(geometry)?;
        Ok(())
    }

    fn required_size(&self, device: &Device<T>) -> usize {
        2 + device.num_transducers() * std::mem::size_of::<u16>()
    }

    fn pack(
        &mut self,
        device: &Device<T>,
        tx: &mut [u8],
    ) -> Result<usize, crate::derive::prelude::AUTDInternalError> {
        T::GainOp::pack(&self.drives, &self.remains, device, tx)
    }

    fn commit(&mut self, device: &Device<T>) {
        self.remains
            .insert(device.idx(), self.remains[&device.idx()] - 1);
    }

    fn remains(&self, device: &Device<T>) -> usize {
        self.remains[&device.idx()]
    }
}
