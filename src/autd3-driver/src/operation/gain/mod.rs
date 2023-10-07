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

mod advanced;
mod advanced_phase;
mod amplitude;
mod gain_control_flags;
mod legacy;

pub use amplitude::AmplitudeOp;
pub use gain_control_flags::GainControlFlags;

use std::collections::HashMap;

use crate::{datagram::Gain, defined::Drive, geometry::Transducer};

pub struct GainOp<T: Transducer, G: Gain<T>> {
    gain: G,
    drives: HashMap<usize, Vec<Drive>>,
    remains: HashMap<usize, usize>,
    phantom: std::marker::PhantomData<T>,
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
